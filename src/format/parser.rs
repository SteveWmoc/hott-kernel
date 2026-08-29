use crate::error::FormatError;
use crate::format::printer::print_canonical;
use crate::syntax::{Arena, Declaration, Module, Natural, Term, TermId};
use std::collections::HashSet;

pub fn parse_transport(input: &[u8]) -> Result<Module, FormatError> {
    let text =
        std::str::from_utf8(input).map_err(|error| FormatError::malformed(error.valid_up_to()))?;
    if text.starts_with('\u{feff}') {
        return Err(FormatError::malformed(0));
    }
    Parser::new(text).parse_module()
}

pub fn parse_canonical(input: &[u8]) -> Result<Module, FormatError> {
    let module = parse_transport(input)?;
    let canonical = print_canonical(&module)?;
    if canonical != input {
        return Err(FormatError::noncanonical());
    }
    Ok(module)
}

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    const fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    fn parse_module(mut self) -> Result<Module, FormatError> {
        self.expect_lparen()?;
        self.expect_atom("hott-core")?;

        self.expect_lparen()?;
        self.expect_atom("format")?;
        let format_major = self.parse_natural()?;
        let format_minor = self.parse_natural()?;
        self.expect_rparen()?;

        self.expect_lparen()?;
        self.expect_atom("theory")?;
        let theory_name = self.parse_string()?;
        let theory_major = self.parse_natural()?;
        let theory_minor = self.parse_natural()?;
        self.expect_rparen()?;

        if format_major.as_str() != "0"
            || format_minor.as_str() != "1"
            || theory_name != "mltt-core"
            || theory_major.as_str() != "0"
            || theory_minor.as_str() != "1"
        {
            return Err(FormatError::unsupported());
        }

        self.expect_lparen()?;
        self.expect_atom("declarations")?;

        let mut arena = Arena::new();
        let mut declarations = Vec::new();
        loop {
            match self.lexer.next_token()? {
                Token {
                    kind: TokenKind::RParen,
                    ..
                } => break,
                Token {
                    kind: TokenKind::LParen,
                    ..
                } => {
                    declarations
                        .try_reserve(1)
                        .map_err(|_| FormatError::resource_exhausted())?;
                    declarations.push(self.parse_declaration_after_lparen(&mut arena)?);
                }
                token => return Err(FormatError::malformed(token.start)),
            }
        }

        self.expect_rparen()?;
        match self.lexer.next_token()? {
            Token {
                kind: TokenKind::Eof,
                ..
            } => {}
            token => return Err(FormatError::malformed(token.start)),
        }

        let mut names = HashSet::new();
        names
            .try_reserve(declarations.len())
            .map_err(|_| FormatError::resource_exhausted())?;
        for declaration in &declarations {
            if !names.insert(declaration.name()) {
                return Err(FormatError::malformed(0));
            }
        }

        Ok(Module::new(arena, declarations))
    }

    fn parse_declaration_after_lparen(
        &mut self,
        arena: &mut Arena,
    ) -> Result<Declaration, FormatError> {
        let tag = self.next_atom()?;
        match tag.text {
            "postulate" => {
                let name = self.parse_string()?;
                let ty = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(Declaration::Postulate { name, ty })
            }
            "transparent" => {
                let name = self.parse_string()?;
                let ty = self.parse_term(arena)?;
                let body = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(Declaration::Transparent { name, ty, body })
            }
            "opaque" => {
                let name = self.parse_string()?;
                let ty = self.parse_term(arena)?;
                let body = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(Declaration::Opaque { name, ty, body })
            }
            _ => Err(FormatError::malformed(tag.start)),
        }
    }

    fn parse_term(&mut self, arena: &mut Arena) -> Result<TermId, FormatError> {
        let mut frames = Vec::<TermFrame>::new();
        let mut value = None;

        loop {
            if let Some(id) = value.take() {
                let Some(frame) = frames.last_mut() else {
                    return Ok(id);
                };

                frame.push_arg(id);
                if frame.is_complete() {
                    let frame = frames.pop().expect("frame just observed");
                    self.expect_rparen()?;
                    value = Some(arena.push(frame.into_term())?);
                }
                continue;
            }

            let token = self.lexer.next_token()?;
            match token.kind {
                TokenKind::Atom(atom) => {
                    let term = match atom {
                        "empty" => Term::Empty,
                        "unit" => Term::Unit,
                        "star" => Term::Star,
                        "nat" => Term::Nat,
                        "zero" => Term::Zero,
                        _ => return Err(FormatError::malformed(token.start)),
                    };
                    value = Some(arena.push(term)?);
                }
                TokenKind::LParen => {
                    let tag = self.next_atom()?;
                    match tag.text {
                        "var" => {
                            let natural = self.parse_natural()?;
                            self.expect_rparen()?;
                            value = Some(arena.push(Term::Var(natural))?);
                        }
                        "global" => {
                            let natural = self.parse_natural()?;
                            self.expect_rparen()?;
                            value = Some(arena.push(Term::Global(natural))?);
                        }
                        "universe" => {
                            let natural = self.parse_natural()?;
                            self.expect_rparen()?;
                            value = Some(arena.push(Term::Universe(natural))?);
                        }
                        other => {
                            let kind = TermFrameKind::from_tag(other)
                                .ok_or_else(|| FormatError::malformed(tag.start))?;
                            frames
                                .try_reserve(1)
                                .map_err(|_| FormatError::resource_exhausted())?;
                            frames.push(TermFrame::new(kind));
                        }
                    }
                }
                _ => return Err(FormatError::malformed(token.start)),
            }
        }
    }

    fn parse_natural(&mut self) -> Result<Natural, FormatError> {
        let atom = self.next_atom()?;
        let bytes = atom.text.as_bytes();
        if bytes.is_empty()
            || !bytes.iter().all(u8::is_ascii_digit)
            || (bytes.len() > 1 && bytes[0] == b'0')
        {
            return Err(FormatError::malformed(atom.start));
        }
        Natural::from_decimal(atom.text).map_err(|error| match error.class() {
            crate::error::FormatErrorClass::ResourceExhausted => error,
            _ => FormatError::malformed(atom.start),
        })
    }

    fn parse_string(&mut self) -> Result<String, FormatError> {
        let token = self.lexer.next_token()?;
        match token.kind {
            TokenKind::String(text) => Ok(text),
            _ => Err(FormatError::malformed(token.start)),
        }
    }

    fn next_atom(&mut self) -> Result<AtomToken<'a>, FormatError> {
        let token = self.lexer.next_token()?;
        match token.kind {
            TokenKind::Atom(text) => Ok(AtomToken {
                text,
                start: token.start,
            }),
            _ => Err(FormatError::malformed(token.start)),
        }
    }

    fn expect_atom(&mut self, expected: &str) -> Result<(), FormatError> {
        let atom = self.next_atom()?;
        if atom.text == expected {
            Ok(())
        } else {
            Err(FormatError::malformed(atom.start))
        }
    }

    fn expect_lparen(&mut self) -> Result<(), FormatError> {
        let token = self.lexer.next_token()?;
        if matches!(token.kind, TokenKind::LParen) {
            Ok(())
        } else {
            Err(FormatError::malformed(token.start))
        }
    }

    fn expect_rparen(&mut self) -> Result<(), FormatError> {
        let token = self.lexer.next_token()?;
        if matches!(token.kind, TokenKind::RParen) {
            Ok(())
        } else {
            Err(FormatError::malformed(token.start))
        }
    }
}

struct AtomToken<'a> {
    text: &'a str,
    start: usize,
}

#[derive(Clone, Copy)]
enum TermFrameKind {
    Pi,
    Lam,
    App,
    Sigma,
    Pair,
    Fst,
    Snd,
    Id,
    Refl,
    J,
    EmptyElim,
    UnitElim,
    Succ,
    NatElim,
    Ann,
}

impl TermFrameKind {
    fn from_tag(tag: &str) -> Option<Self> {
        Some(match tag {
            "pi" => Self::Pi,
            "lam" => Self::Lam,
            "app" => Self::App,
            "sigma" => Self::Sigma,
            "pair" => Self::Pair,
            "fst" => Self::Fst,
            "snd" => Self::Snd,
            "id" => Self::Id,
            "refl" => Self::Refl,
            "j" => Self::J,
            "empty-elim" => Self::EmptyElim,
            "unit-elim" => Self::UnitElim,
            "succ" => Self::Succ,
            "nat-elim" => Self::NatElim,
            "ann" => Self::Ann,
            _ => return None,
        })
    }

    const fn arity(self) -> usize {
        match self {
            Self::Lam | Self::Fst | Self::Snd | Self::Refl | Self::Succ => 1,
            Self::Pi | Self::App | Self::Sigma | Self::Pair | Self::EmptyElim | Self::Ann => 2,
            Self::Id | Self::UnitElim => 3,
            Self::NatElim => 4,
            Self::J => 6,
        }
    }
}

struct TermFrame {
    kind: TermFrameKind,
    args: [TermId; 6],
    len: usize,
}

impl TermFrame {
    const fn new(kind: TermFrameKind) -> Self {
        Self {
            kind,
            args: [TermId::from_index(0); 6],
            len: 0,
        }
    }

    fn push_arg(&mut self, id: TermId) {
        debug_assert!(self.len < self.kind.arity());
        self.args[self.len] = id;
        self.len += 1;
    }

    const fn is_complete(&self) -> bool {
        self.len == self.kind.arity()
    }

    fn into_term(self) -> Term {
        let a = self.args;
        match self.kind {
            TermFrameKind::Pi => Term::Pi(a[0], a[1]),
            TermFrameKind::Lam => Term::Lam(a[0]),
            TermFrameKind::App => Term::App(a[0], a[1]),
            TermFrameKind::Sigma => Term::Sigma(a[0], a[1]),
            TermFrameKind::Pair => Term::Pair(a[0], a[1]),
            TermFrameKind::Fst => Term::Fst(a[0]),
            TermFrameKind::Snd => Term::Snd(a[0]),
            TermFrameKind::Id => Term::Id(a[0], a[1], a[2]),
            TermFrameKind::Refl => Term::Refl(a[0]),
            TermFrameKind::J => Term::J(a[0], a[1], a[2], a[3], a[4], a[5]),
            TermFrameKind::EmptyElim => Term::EmptyElim(a[0], a[1]),
            TermFrameKind::UnitElim => Term::UnitElim(a[0], a[1], a[2]),
            TermFrameKind::Succ => Term::Succ(a[0]),
            TermFrameKind::NatElim => Term::NatElim(a[0], a[1], a[2], a[3]),
            TermFrameKind::Ann => Term::Ann(a[0], a[1]),
        }
    }
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    const fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn next_token(&mut self) -> Result<Token<'a>, FormatError> {
        self.skip_whitespace();
        let start = self.pos;
        let bytes = self.input.as_bytes();
        if start == bytes.len() {
            return Ok(Token {
                kind: TokenKind::Eof,
                start,
            });
        }

        match bytes[start] {
            b'(' => {
                self.pos += 1;
                Ok(Token {
                    kind: TokenKind::LParen,
                    start,
                })
            }
            b')' => {
                self.pos += 1;
                Ok(Token {
                    kind: TokenKind::RParen,
                    start,
                })
            }
            b'"' => self.lex_string(),
            byte if is_ascii_atom_byte(byte) => self.lex_atom(),
            _ => Err(FormatError::malformed(start)),
        }
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.input.as_bytes();
        while self.pos < bytes.len() && matches!(bytes[self.pos], b'\t' | b'\n' | b'\r' | b' ') {
            self.pos += 1;
        }
    }

    fn lex_atom(&mut self) -> Result<Token<'a>, FormatError> {
        let start = self.pos;
        let bytes = self.input.as_bytes();
        while self.pos < bytes.len() {
            let byte = bytes[self.pos];
            if matches!(byte, b'(' | b')' | b'"' | b'\t' | b'\n' | b'\r' | b' ') {
                break;
            }
            if !is_ascii_atom_byte(byte) {
                return Err(FormatError::malformed(self.pos));
            }
            self.pos += 1;
        }
        Ok(Token {
            kind: TokenKind::Atom(&self.input[start..self.pos]),
            start,
        })
    }

    fn lex_string(&mut self) -> Result<Token<'a>, FormatError> {
        let start = self.pos;
        self.pos += 1;
        let mut decoded = String::new();

        while self.pos < self.input.len() {
            let rest = &self.input[self.pos..];
            let ch = rest.chars().next().expect("nonempty string slice");
            match ch {
                '"' => {
                    self.pos += 1;
                    if decoded.is_empty() {
                        return Err(FormatError::malformed(start));
                    }
                    return Ok(Token {
                        kind: TokenKind::String(decoded),
                        start,
                    });
                }
                '\\' => {
                    let escape_start = self.pos;
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err(FormatError::malformed(escape_start));
                    }
                    let escaped = self.input.as_bytes()[self.pos];
                    let decoded_char = match escaped {
                        b'"' => '"',
                        b'\\' => '\\',
                        _ => return Err(FormatError::malformed(escape_start)),
                    };
                    decoded
                        .try_reserve(decoded_char.len_utf8())
                        .map_err(|_| FormatError::resource_exhausted())?;
                    decoded.push(decoded_char);
                    self.pos += 1;
                }
                _ if allowed_direct_string_scalar(ch) => {
                    decoded
                        .try_reserve(ch.len_utf8())
                        .map_err(|_| FormatError::resource_exhausted())?;
                    decoded.push(ch);
                    self.pos += ch.len_utf8();
                }
                _ => return Err(FormatError::malformed(self.pos)),
            }
        }

        Err(FormatError::malformed(start))
    }
}

fn is_ascii_atom_byte(byte: u8) -> bool {
    (0x21..=0x7e).contains(&byte) && !matches!(byte, b'(' | b')' | b'"')
}

fn allowed_direct_string_scalar(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0020..=0x0021
            | 0x0023..=0x005b
            | 0x005d..=0x007e
            | 0x00a0..=0xd7ff
            | 0xe000..=0x10ffff
    )
}

struct Token<'a> {
    kind: TokenKind<'a>,
    start: usize,
}

enum TokenKind<'a> {
    LParen,
    RParen,
    Atom(&'a str),
    String(String),
    Eof,
}

use crate::error::FormatErrorClass;
use crate::surface::{
    SurfaceArena, SurfaceDeclaration, SurfaceError, SurfaceIdentifier, SurfaceModule, SurfaceTerm,
    SurfaceTermId,
};
use crate::syntax::Natural;

pub fn parse_surface(input: &[u8]) -> Result<SurfaceModule, SurfaceError> {
    let text =
        std::str::from_utf8(input).map_err(|error| SurfaceError::malformed(error.valid_up_to()))?;
    if text.starts_with('\u{feff}') {
        return Err(SurfaceError::malformed(0));
    }
    Parser::new(text).parse_module()
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

    fn parse_module(mut self) -> Result<SurfaceModule, SurfaceError> {
        self.expect_lparen()?;
        self.expect_atom("surface")?;
        let major = self.parse_natural()?;
        let minor = self.parse_natural()?;
        self.expect_rparen()?;

        if major.as_str() != "0" || minor.as_str() != "1" {
            return Err(SurfaceError::unsupported());
        }

        self.expect_lparen()?;
        self.expect_atom("module")?;
        let module_name = self.parse_identifier()?;
        self.expect_rparen()?;

        let mut arena = SurfaceArena::new();
        let mut declarations = Vec::new();

        loop {
            let token = self.lexer.next_token()?;
            match token.kind {
                TokenKind::Eof => break,
                TokenKind::LParen => {
                    declarations
                        .try_reserve(1)
                        .map_err(|_| SurfaceError::resource_exhausted())?;
                    declarations.push(self.parse_declaration_after_lparen(&mut arena)?);
                }
                _ => return Err(SurfaceError::malformed(token.start)),
            }
        }

        Ok(SurfaceModule::new(module_name, arena, declarations))
    }

    fn parse_declaration_after_lparen(
        &mut self,
        arena: &mut SurfaceArena,
    ) -> Result<SurfaceDeclaration, SurfaceError> {
        let tag = self.next_atom()?;
        match tag.text {
            "postulate" => {
                let name = self.parse_identifier()?;
                let ty = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(SurfaceDeclaration::Postulate { name, ty })
            }
            "transparent" => {
                let name = self.parse_identifier()?;
                let ty = self.parse_term(arena)?;
                let body = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(SurfaceDeclaration::Transparent { name, ty, body })
            }
            "opaque" => {
                let name = self.parse_identifier()?;
                let ty = self.parse_term(arena)?;
                let body = self.parse_term(arena)?;
                self.expect_rparen()?;
                Ok(SurfaceDeclaration::Opaque { name, ty, body })
            }
            _ => Err(SurfaceError::malformed(tag.start)),
        }
    }

    fn parse_term(&mut self, arena: &mut SurfaceArena) -> Result<SurfaceTermId, SurfaceError> {
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
                        "empty" => SurfaceTerm::Empty,
                        "unit" => SurfaceTerm::Unit,
                        "star" => SurfaceTerm::Star,
                        "nat" => SurfaceTerm::Nat,
                        "zero" => SurfaceTerm::Zero,
                        _ => return Err(SurfaceError::malformed(token.start)),
                    };
                    value = Some(arena.push(term)?);
                }
                TokenKind::LParen => {
                    let tag = self.next_atom()?;
                    match tag.text {
                        "ref" => {
                            let name = self.parse_identifier()?;
                            self.expect_rparen()?;
                            value = Some(arena.push(SurfaceTerm::Ref(name))?);
                        }
                        "universe" => {
                            let level = self.parse_natural()?;
                            self.expect_rparen()?;
                            value = Some(arena.push(SurfaceTerm::Universe(level))?);
                        }
                        "pi" => {
                            let binder = self.parse_identifier()?;
                            frames
                                .try_reserve(1)
                                .map_err(|_| SurfaceError::resource_exhausted())?;
                            frames.push(TermFrame::new(TermFrameKind::Pi(binder)));
                        }
                        "lam" => {
                            let binder = self.parse_identifier()?;
                            frames
                                .try_reserve(1)
                                .map_err(|_| SurfaceError::resource_exhausted())?;
                            frames.push(TermFrame::new(TermFrameKind::Lam(binder)));
                        }
                        "sigma" => {
                            let binder = self.parse_identifier()?;
                            frames
                                .try_reserve(1)
                                .map_err(|_| SurfaceError::resource_exhausted())?;
                            frames.push(TermFrame::new(TermFrameKind::Sigma(binder)));
                        }
                        other => {
                            let kind = TermFrameKind::from_tag(other)
                                .ok_or_else(|| SurfaceError::malformed(tag.start))?;
                            frames
                                .try_reserve(1)
                                .map_err(|_| SurfaceError::resource_exhausted())?;
                            frames.push(TermFrame::new(kind));
                        }
                    }
                }
                _ => return Err(SurfaceError::malformed(token.start)),
            }
        }
    }

    fn parse_natural(&mut self) -> Result<Natural, SurfaceError> {
        let atom = self.next_atom()?;
        let bytes = atom.text.as_bytes();
        if bytes.is_empty()
            || !bytes.iter().all(u8::is_ascii_digit)
            || (bytes.len() > 1 && bytes[0] == b'0')
        {
            return Err(SurfaceError::malformed(atom.start));
        }

        Natural::from_decimal(atom.text).map_err(|error| match error.class() {
            FormatErrorClass::ResourceExhausted => SurfaceError::resource_exhausted(),
            _ => SurfaceError::malformed(atom.start),
        })
    }

    fn parse_identifier(&mut self) -> Result<SurfaceIdentifier, SurfaceError> {
        let atom = self.next_atom()?;
        if is_reserved(atom.text) {
            return Err(SurfaceError::reserved(atom.start));
        }
        if !is_identifier(atom.text) {
            return Err(SurfaceError::malformed(atom.start));
        }
        SurfaceIdentifier::from_validated(atom.text)
    }

    fn next_atom(&mut self) -> Result<AtomToken<'a>, SurfaceError> {
        let token = self.lexer.next_token()?;
        match token.kind {
            TokenKind::Atom(text) => Ok(AtomToken {
                text,
                start: token.start,
            }),
            _ => Err(SurfaceError::malformed(token.start)),
        }
    }

    fn expect_atom(&mut self, expected: &str) -> Result<(), SurfaceError> {
        let atom = self.next_atom()?;
        if atom.text == expected {
            Ok(())
        } else {
            Err(SurfaceError::malformed(atom.start))
        }
    }

    fn expect_lparen(&mut self) -> Result<(), SurfaceError> {
        let token = self.lexer.next_token()?;
        if matches!(token.kind, TokenKind::LParen) {
            Ok(())
        } else {
            Err(SurfaceError::malformed(token.start))
        }
    }

    fn expect_rparen(&mut self) -> Result<(), SurfaceError> {
        let token = self.lexer.next_token()?;
        if matches!(token.kind, TokenKind::RParen) {
            Ok(())
        } else {
            Err(SurfaceError::malformed(token.start))
        }
    }
}

struct AtomToken<'a> {
    text: &'a str,
    start: usize,
}

enum TermFrameKind {
    Pi(SurfaceIdentifier),
    Lam(SurfaceIdentifier),
    App,
    Sigma(SurfaceIdentifier),
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
            "app" => Self::App,
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

    const fn arity(&self) -> usize {
        match self {
            Self::Lam(_) | Self::Fst | Self::Snd | Self::Refl | Self::Succ => 1,
            Self::Pi(_) | Self::App | Self::Sigma(_) | Self::Pair | Self::EmptyElim | Self::Ann => {
                2
            }
            Self::Id | Self::UnitElim => 3,
            Self::NatElim => 4,
            Self::J => 6,
        }
    }
}

struct TermFrame {
    kind: TermFrameKind,
    args: [SurfaceTermId; 6],
    len: usize,
}

impl TermFrame {
    const fn new(kind: TermFrameKind) -> Self {
        Self {
            kind,
            args: [SurfaceTermId::from_index(0); 6],
            len: 0,
        }
    }

    fn push_arg(&mut self, id: SurfaceTermId) {
        debug_assert!(self.len < self.kind.arity());
        self.args[self.len] = id;
        self.len += 1;
    }

    fn is_complete(&self) -> bool {
        self.len == self.kind.arity()
    }

    fn into_term(self) -> SurfaceTerm {
        let a = self.args;
        match self.kind {
            TermFrameKind::Pi(binder) => SurfaceTerm::Pi {
                binder,
                domain: a[0],
                codomain: a[1],
            },
            TermFrameKind::Lam(binder) => SurfaceTerm::Lam { binder, body: a[0] },
            TermFrameKind::App => SurfaceTerm::App(a[0], a[1]),
            TermFrameKind::Sigma(binder) => SurfaceTerm::Sigma {
                binder,
                domain: a[0],
                codomain: a[1],
            },
            TermFrameKind::Pair => SurfaceTerm::Pair(a[0], a[1]),
            TermFrameKind::Fst => SurfaceTerm::Fst(a[0]),
            TermFrameKind::Snd => SurfaceTerm::Snd(a[0]),
            TermFrameKind::Id => SurfaceTerm::Id(a[0], a[1], a[2]),
            TermFrameKind::Refl => SurfaceTerm::Refl(a[0]),
            TermFrameKind::J => SurfaceTerm::J(a[0], a[1], a[2], a[3], a[4], a[5]),
            TermFrameKind::EmptyElim => SurfaceTerm::EmptyElim(a[0], a[1]),
            TermFrameKind::UnitElim => SurfaceTerm::UnitElim(a[0], a[1], a[2]),
            TermFrameKind::Succ => SurfaceTerm::Succ(a[0]),
            TermFrameKind::NatElim => SurfaceTerm::NatElim(a[0], a[1], a[2], a[3]),
            TermFrameKind::Ann => SurfaceTerm::Ann(a[0], a[1]),
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

    fn next_token(&mut self) -> Result<Token<'a>, SurfaceError> {
        self.skip_layout();
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
            byte if is_atom_byte(byte) => self.lex_atom(),
            _ => Err(SurfaceError::malformed(start)),
        }
    }

    fn skip_layout(&mut self) {
        let bytes = self.input.as_bytes();
        loop {
            while self.pos < bytes.len() && matches!(bytes[self.pos], b'\t' | b'\n' | b'\r' | b' ')
            {
                self.pos += 1;
            }

            if self.pos >= bytes.len() || bytes[self.pos] != b';' {
                break;
            }

            self.pos += 1;
            while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
                self.pos += 1;
            }
        }
    }

    fn lex_atom(&mut self) -> Result<Token<'a>, SurfaceError> {
        let start = self.pos;
        let bytes = self.input.as_bytes();

        while self.pos < bytes.len() {
            let byte = bytes[self.pos];
            if matches!(byte, b'(' | b')' | b';' | b'\t' | b'\n' | b'\r' | b' ') {
                break;
            }
            if !is_atom_byte(byte) {
                return Err(SurfaceError::malformed(self.pos));
            }
            self.pos += 1;
        }

        Ok(Token {
            kind: TokenKind::Atom(&self.input[start..self.pos]),
            start,
        })
    }
}

fn is_atom_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
}

fn is_identifier(text: &str) -> bool {
    let mut bytes = text.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn is_reserved(text: &str) -> bool {
    matches!(
        text,
        "surface"
            | "module"
            | "postulate"
            | "transparent"
            | "opaque"
            | "ref"
            | "universe"
            | "pi"
            | "lam"
            | "app"
            | "sigma"
            | "pair"
            | "fst"
            | "snd"
            | "id"
            | "refl"
            | "j"
            | "empty"
            | "empty-elim"
            | "unit"
            | "star"
            | "unit-elim"
            | "nat"
            | "zero"
            | "succ"
            | "nat-elim"
            | "ann"
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
    Eof,
}

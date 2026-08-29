use crate::error::FormatError;
use crate::syntax::{Arena, Declaration, Module, Term, TermId};

pub fn print_canonical(module: &Module) -> Result<Vec<u8>, FormatError> {
    let mut out = Buffer::new();
    out.push_str("(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations")?;
    for declaration in module.declarations() {
        out.push_str(" ")?;
        write_declaration(&mut out, module.arena(), declaration, Projection::Artifact)?;
    }
    out.push_str("))\n")?;
    Ok(out.into_bytes())
}

pub fn print_semantic(module: &Module) -> Result<Vec<u8>, FormatError> {
    let mut out = Buffer::new();
    out.push_str("(hott-semantic (projection 0 1) (theory \"mltt-core\" 0 1) (declarations")?;
    for declaration in module.declarations() {
        out.push_str(" ")?;
        write_declaration(&mut out, module.arena(), declaration, Projection::Semantic)?;
    }
    out.push_str("))\n")?;
    Ok(out.into_bytes())
}

#[derive(Clone, Copy)]
enum Projection {
    Artifact,
    Semantic,
}

fn write_declaration(
    out: &mut Buffer,
    arena: &Arena,
    declaration: &Declaration,
    projection: Projection,
) -> Result<(), FormatError> {
    match declaration {
        Declaration::Postulate { name, ty } => {
            out.push_str("(postulate")?;
            if matches!(projection, Projection::Artifact) {
                out.push_str(" ")?;
                write_string(out, name)?;
            }
            out.push_str(" ")?;
            write_term(out, arena, *ty)?;
            out.push_str(")")?;
        }
        Declaration::Transparent { name, ty, body } => {
            out.push_str("(transparent")?;
            if matches!(projection, Projection::Artifact) {
                out.push_str(" ")?;
                write_string(out, name)?;
            }
            out.push_str(" ")?;
            write_term(out, arena, *ty)?;
            out.push_str(" ")?;
            write_term(out, arena, *body)?;
            out.push_str(")")?;
        }
        Declaration::Opaque { name, ty, body } => {
            out.push_str("(opaque")?;
            if matches!(projection, Projection::Artifact) {
                out.push_str(" ")?;
                write_string(out, name)?;
            }
            out.push_str(" ")?;
            write_term(out, arena, *ty)?;
            out.push_str(" ")?;
            write_term(out, arena, *body)?;
            out.push_str(")")?;
        }
    }
    Ok(())
}

fn write_string(out: &mut Buffer, value: &str) -> Result<(), FormatError> {
    out.push_str("\"")?;
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\"")?,
            '\\' => out.push_str("\\\\")?,
            _ => out.push_char(ch)?,
        }
    }
    out.push_str("\"")
}

fn write_term(out: &mut Buffer, arena: &Arena, root: TermId) -> Result<(), FormatError> {
    let mut stack = Vec::new();
    stack
        .try_reserve(1)
        .map_err(|_| FormatError::resource_exhausted())?;
    stack.push(PrintOp::Term(root));

    while let Some(op) = stack.pop() {
        match op {
            PrintOp::Text(text) => out.push_str(text)?,
            PrintOp::Term(id) => {
                let term = arena.get(id).expect("module term-id invariant");
                match term {
                    Term::Var(n) => {
                        out.push_str("(var ")?;
                        out.push_str(n.as_str())?;
                        out.push_str(")")?;
                    }
                    Term::Global(n) => {
                        out.push_str("(global ")?;
                        out.push_str(n.as_str())?;
                        out.push_str(")")?;
                    }
                    Term::Universe(n) => {
                        out.push_str("(universe ")?;
                        out.push_str(n.as_str())?;
                        out.push_str(")")?;
                    }
                    Term::Pi(a, b) => push_terms(&mut stack, out, "(pi ", &[*a, *b])?,
                    Term::Lam(t) => push_terms(&mut stack, out, "(lam ", &[*t])?,
                    Term::App(a, b) => push_terms(&mut stack, out, "(app ", &[*a, *b])?,
                    Term::Sigma(a, b) => push_terms(&mut stack, out, "(sigma ", &[*a, *b])?,
                    Term::Pair(a, b) => push_terms(&mut stack, out, "(pair ", &[*a, *b])?,
                    Term::Fst(t) => push_terms(&mut stack, out, "(fst ", &[*t])?,
                    Term::Snd(t) => push_terms(&mut stack, out, "(snd ", &[*t])?,
                    Term::Id(a, b, c) => push_terms(&mut stack, out, "(id ", &[*a, *b, *c])?,
                    Term::Refl(t) => push_terms(&mut stack, out, "(refl ", &[*t])?,
                    Term::J(a, b, c, d, e, f) => {
                        push_terms(&mut stack, out, "(j ", &[*a, *b, *c, *d, *e, *f])?
                    }
                    Term::Empty => out.push_str("empty")?,
                    Term::EmptyElim(a, b) => {
                        push_terms(&mut stack, out, "(empty-elim ", &[*a, *b])?
                    }
                    Term::Unit => out.push_str("unit")?,
                    Term::Star => out.push_str("star")?,
                    Term::UnitElim(a, b, c) => {
                        push_terms(&mut stack, out, "(unit-elim ", &[*a, *b, *c])?
                    }
                    Term::Nat => out.push_str("nat")?,
                    Term::Zero => out.push_str("zero")?,
                    Term::Succ(t) => push_terms(&mut stack, out, "(succ ", &[*t])?,
                    Term::NatElim(a, b, c, d) => {
                        push_terms(&mut stack, out, "(nat-elim ", &[*a, *b, *c, *d])?
                    }
                    Term::Ann(a, b) => push_terms(&mut stack, out, "(ann ", &[*a, *b])?,
                }
            }
        }
    }
    Ok(())
}

fn push_terms(
    stack: &mut Vec<PrintOp>,
    out: &mut Buffer,
    prefix: &'static str,
    terms: &[TermId],
) -> Result<(), FormatError> {
    out.push_str(prefix)?;
    let operations = terms.len().saturating_mul(2);
    stack
        .try_reserve(operations)
        .map_err(|_| FormatError::resource_exhausted())?;
    stack.push(PrintOp::Text(")"));
    for (index, term) in terms.iter().enumerate().rev() {
        stack.push(PrintOp::Term(*term));
        if index != 0 {
            stack.push(PrintOp::Text(" "));
        }
    }
    Ok(())
}

enum PrintOp {
    Text(&'static str),
    Term(TermId),
}

struct Buffer(String);

impl Buffer {
    const fn new() -> Self {
        Self(String::new())
    }

    fn push_str(&mut self, text: &str) -> Result<(), FormatError> {
        self.0
            .try_reserve(text.len())
            .map_err(|_| FormatError::resource_exhausted())?;
        self.0.push_str(text);
        Ok(())
    }

    fn push_char(&mut self, ch: char) -> Result<(), FormatError> {
        self.0
            .try_reserve(ch.len_utf8())
            .map_err(|_| FormatError::resource_exhausted())?;
        self.0.push(ch);
        Ok(())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }
}

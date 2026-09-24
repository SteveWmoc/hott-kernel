use core::fmt;

use crate::syntax::Natural;

mod checked;
mod compile;
mod elaborate;
mod parser;

pub use checked::{SurfaceCheckError, compile_and_check_surface};
pub use compile::{SurfaceCompileError, compile_surface};
pub use elaborate::elaborate_surface;
pub use parser::parse_surface;

pub const SURFACE_FORMAT: &str = "hott-surface/0.1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SurfaceErrorClass {
    MalformedSyntax,
    UnsupportedVersion,
    ReservedIdentifier,
    DuplicateGlobalDeclarationName,
    UnknownName,
    ResourceExhausted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceError {
    class: SurfaceErrorClass,
    offset: Option<usize>,
}

impl SurfaceError {
    pub const fn class(&self) -> SurfaceErrorClass {
        self.class
    }

    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }

    const fn malformed(offset: usize) -> Self {
        Self {
            class: SurfaceErrorClass::MalformedSyntax,
            offset: Some(offset),
        }
    }

    const fn unsupported() -> Self {
        Self {
            class: SurfaceErrorClass::UnsupportedVersion,
            offset: None,
        }
    }

    const fn reserved(offset: usize) -> Self {
        Self {
            class: SurfaceErrorClass::ReservedIdentifier,
            offset: Some(offset),
        }
    }

    const fn duplicate_global() -> Self {
        Self {
            class: SurfaceErrorClass::DuplicateGlobalDeclarationName,
            offset: None,
        }
    }

    const fn unknown_name() -> Self {
        Self {
            class: SurfaceErrorClass::UnknownName,
            offset: None,
        }
    }

    const fn resource_exhausted() -> Self {
        Self {
            class: SurfaceErrorClass::ResourceExhausted,
            offset: None,
        }
    }
}

impl fmt::Display for SurfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.class {
            SurfaceErrorClass::MalformedSyntax => "malformed surface syntax",
            SurfaceErrorClass::UnsupportedVersion => "unsupported surface version",
            SurfaceErrorClass::ReservedIdentifier => "reserved surface identifier",
            SurfaceErrorClass::DuplicateGlobalDeclarationName => {
                "duplicate global declaration name"
            }
            SurfaceErrorClass::UnknownName => "unknown surface name",
            SurfaceErrorClass::ResourceExhausted => "resource exhausted",
        };
        match self.offset {
            Some(offset) => write!(formatter, "{label} at byte {offset}"),
            None => formatter.write_str(label),
        }
    }
}

impl std::error::Error for SurfaceError {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SurfaceIdentifier(String);

impl SurfaceIdentifier {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_validated(text: &str) -> Result<Self, SurfaceError> {
        let mut owned = String::new();
        owned
            .try_reserve_exact(text.len())
            .map_err(|_| SurfaceError::resource_exhausted())?;
        owned.push_str(text);
        Ok(Self(owned))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SurfaceTermId(usize);

impl SurfaceTermId {
    pub const fn index(self) -> usize {
        self.0
    }

    const fn from_index(index: usize) -> Self {
        Self(index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceTerm {
    Ref(SurfaceIdentifier),
    Universe(Natural),
    Pi {
        binder: SurfaceIdentifier,
        domain: SurfaceTermId,
        codomain: SurfaceTermId,
    },
    Lam {
        binder: SurfaceIdentifier,
        body: SurfaceTermId,
    },
    App(SurfaceTermId, SurfaceTermId),
    Sigma {
        binder: SurfaceIdentifier,
        domain: SurfaceTermId,
        codomain: SurfaceTermId,
    },
    Pair(SurfaceTermId, SurfaceTermId),
    Fst(SurfaceTermId),
    Snd(SurfaceTermId),
    Id(SurfaceTermId, SurfaceTermId, SurfaceTermId),
    Refl(SurfaceTermId),
    J(
        SurfaceTermId,
        SurfaceTermId,
        SurfaceTermId,
        SurfaceTermId,
        SurfaceTermId,
        SurfaceTermId,
    ),
    Empty,
    EmptyElim(SurfaceTermId, SurfaceTermId),
    Unit,
    Star,
    UnitElim(SurfaceTermId, SurfaceTermId, SurfaceTermId),
    Nat,
    Zero,
    Succ(SurfaceTermId),
    NatElim(SurfaceTermId, SurfaceTermId, SurfaceTermId, SurfaceTermId),
    Ann(SurfaceTermId, SurfaceTermId),
}

impl SurfaceTerm {
    fn all_children_before(&self, upper: usize) -> bool {
        let before = |id: &SurfaceTermId| id.index() < upper;
        match self {
            Self::Ref(_) | Self::Universe(_) => true,
            Self::Pi {
                domain, codomain, ..
            }
            | Self::Sigma {
                domain, codomain, ..
            } => before(domain) && before(codomain),
            Self::Lam { body, .. }
            | Self::Fst(body)
            | Self::Snd(body)
            | Self::Refl(body)
            | Self::Succ(body) => before(body),
            Self::App(a, b) | Self::Pair(a, b) | Self::EmptyElim(a, b) | Self::Ann(a, b) => {
                before(a) && before(b)
            }
            Self::Id(a, b, c) | Self::UnitElim(a, b, c) => before(a) && before(b) && before(c),
            Self::J(a, b, c, d, e, f) => {
                before(a) && before(b) && before(c) && before(d) && before(e) && before(f)
            }
            Self::NatElim(a, b, c, d) => before(a) && before(b) && before(c) && before(d),
            Self::Empty | Self::Unit | Self::Star | Self::Nat | Self::Zero => true,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SurfaceArena {
    terms: Vec<SurfaceTerm>,
}

impl SurfaceArena {
    pub const fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn get(&self, id: SurfaceTermId) -> Option<&SurfaceTerm> {
        self.terms.get(id.index())
    }

    fn push(&mut self, term: SurfaceTerm) -> Result<SurfaceTermId, SurfaceError> {
        let next = self.terms.len();
        debug_assert!(term.all_children_before(next));
        if !term.all_children_before(next) {
            return Err(SurfaceError::malformed(0));
        }
        self.terms
            .try_reserve(1)
            .map_err(|_| SurfaceError::resource_exhausted())?;
        self.terms.push(term);
        Ok(SurfaceTermId::from_index(next))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceDeclaration {
    Postulate {
        name: SurfaceIdentifier,
        ty: SurfaceTermId,
    },
    Transparent {
        name: SurfaceIdentifier,
        ty: SurfaceTermId,
        body: SurfaceTermId,
    },
    Opaque {
        name: SurfaceIdentifier,
        ty: SurfaceTermId,
        body: SurfaceTermId,
    },
}

impl SurfaceDeclaration {
    pub fn name(&self) -> &SurfaceIdentifier {
        match self {
            Self::Postulate { name, .. }
            | Self::Transparent { name, .. }
            | Self::Opaque { name, .. } => name,
        }
    }

    pub fn ty(&self) -> SurfaceTermId {
        match self {
            Self::Postulate { ty, .. } | Self::Transparent { ty, .. } | Self::Opaque { ty, .. } => {
                *ty
            }
        }
    }

    pub fn body(&self) -> Option<SurfaceTermId> {
        match self {
            Self::Postulate { .. } => None,
            Self::Transparent { body, .. } | Self::Opaque { body, .. } => Some(*body),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceModule {
    name: SurfaceIdentifier,
    arena: SurfaceArena,
    declarations: Vec<SurfaceDeclaration>,
}

impl SurfaceModule {
    fn new(
        name: SurfaceIdentifier,
        arena: SurfaceArena,
        declarations: Vec<SurfaceDeclaration>,
    ) -> Self {
        Self {
            name,
            arena,
            declarations,
        }
    }

    pub const fn format(&self) -> &'static str {
        SURFACE_FORMAT
    }

    pub const fn name(&self) -> &SurfaceIdentifier {
        &self.name
    }

    pub const fn arena(&self) -> &SurfaceArena {
        &self.arena
    }

    pub fn declarations(&self) -> &[SurfaceDeclaration] {
        &self.declarations
    }
}

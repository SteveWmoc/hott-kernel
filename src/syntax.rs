use crate::error::FormatError;
use core::{cmp::Ordering, fmt};

/// Canonical decimal representation of a mathematically unbounded natural.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Natural(String);

impl Natural {
    pub fn from_decimal(text: &str) -> Result<Self, FormatError> {
        if text.is_empty()
            || !text.as_bytes().iter().all(u8::is_ascii_digit)
            || (text.len() > 1 && text.as_bytes()[0] == b'0')
        {
            return Err(FormatError::malformed(0));
        }

        let mut owned = String::new();
        owned
            .try_reserve_exact(text.len())
            .map_err(|_| FormatError::resource_exhausted())?;
        owned.push_str(text);
        Ok(Self(owned))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.as_str();
        let right = other.as_str();

        // Canonical decimals have no leading zeroes, so digit count orders
        // magnitude and lexicographic order breaks equal-length ties.
        left.len().cmp(&right.len()).then_with(|| left.cmp(right))
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Natural").field(&self.as_str()).finish()
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable index into an append-only term arena.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TermId(usize);

impl TermId {
    pub const fn index(self) -> usize {
        self.0
    }

    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index)
    }
}

/// The 23 frozen Core v0.1 raw-term constructors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Var(Natural),
    Global(Natural),
    Universe(Natural),
    Pi(TermId, TermId),
    Lam(TermId),
    App(TermId, TermId),
    Sigma(TermId, TermId),
    Pair(TermId, TermId),
    Fst(TermId),
    Snd(TermId),
    Id(TermId, TermId, TermId),
    Refl(TermId),
    J(TermId, TermId, TermId, TermId, TermId, TermId),
    Empty,
    EmptyElim(TermId, TermId),
    Unit,
    Star,
    UnitElim(TermId, TermId, TermId),
    Nat,
    Zero,
    Succ(TermId),
    NatElim(TermId, TermId, TermId, TermId),
    Ann(TermId, TermId),
}

impl Term {
    fn all_children_before(&self, upper: usize) -> bool {
        let before = |id: &TermId| id.index() < upper;
        match self {
            Self::Var(_) | Self::Global(_) | Self::Universe(_) => true,
            Self::Pi(a, b)
            | Self::App(a, b)
            | Self::Sigma(a, b)
            | Self::Pair(a, b)
            | Self::EmptyElim(a, b)
            | Self::Ann(a, b) => before(a) && before(b),
            Self::Lam(t) | Self::Fst(t) | Self::Snd(t) | Self::Refl(t) | Self::Succ(t) => before(t),
            Self::Id(a, b, c) | Self::UnitElim(a, b, c) => before(a) && before(b) && before(c),
            Self::J(a, b, c, d, e, f) => {
                before(a) && before(b) && before(c) && before(d) && before(e) && before(f)
            }
            Self::NatElim(a, b, c, d) => before(a) && before(b) && before(c) && before(d),
            Self::Empty | Self::Unit | Self::Star | Self::Nat | Self::Zero => true,
        }
    }
}

/// Append-only storage for raw terms.
///
/// Every term may reference only nodes already present in the arena. This makes
/// the internal graph acyclic without recursive ownership.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Arena {
    terms: Vec<Term>,
}

impl Arena {
    pub const fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn get(&self, id: TermId) -> Option<&Term> {
        self.terms.get(id.index())
    }

    pub(crate) fn push(&mut self, term: Term) -> Result<TermId, FormatError> {
        let next = self.terms.len();
        debug_assert!(term.all_children_before(next));
        if !term.all_children_before(next) {
            return Err(FormatError::malformed(0));
        }
        self.terms
            .try_reserve(1)
            .map_err(|_| FormatError::resource_exhausted())?;
        self.terms.push(term);
        Ok(TermId::from_index(next))
    }
}

/// One declaration in source order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    Postulate {
        name: String,
        ty: TermId,
    },
    Transparent {
        name: String,
        ty: TermId,
        body: TermId,
    },
    Opaque {
        name: String,
        ty: TermId,
        body: TermId,
    },
}

impl Declaration {
    pub fn name(&self) -> &str {
        match self {
            Self::Postulate { name, .. }
            | Self::Transparent { name, .. }
            | Self::Opaque { name, .. } => name,
        }
    }

    pub fn ty(&self) -> TermId {
        match self {
            Self::Postulate { ty, .. } | Self::Transparent { ty, .. } | Self::Opaque { ty, .. } => {
                *ty
            }
        }
    }

    pub fn body(&self) -> Option<TermId> {
        match self {
            Self::Postulate { .. } => None,
            Self::Transparent { body, .. } | Self::Opaque { body, .. } => Some(*body),
        }
    }
}

/// A decoded supported `hott-core/0.1` module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    arena: Arena,
    declarations: Vec<Declaration>,
}

impl Module {
    pub(crate) fn new(arena: Arena, declarations: Vec<Declaration>) -> Self {
        Self {
            arena,
            declarations,
        }
    }

    pub fn arena(&self) -> &Arena {
        &self.arena
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }
}

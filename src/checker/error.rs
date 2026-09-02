use crate::syntax::TermId;
use core::fmt;

/// Stable public classification of checker-layer failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckErrorClass {
    InvalidJudgment,
    ResourceExhausted,
}

/// The kind of unavailable reference found by reference validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    Local,
    Global,
}

/// A checker-layer failure.
///
/// Diagnostic detail is not part of the stable Core v0.1 result vocabulary.
/// Consumers should branch on [`CheckError::class`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckError {
    class: CheckErrorClass,
    declaration_index: usize,
    term_id: Option<TermId>,
    reference_kind: Option<ReferenceKind>,
}

impl CheckError {
    pub const fn class(&self) -> CheckErrorClass {
        self.class
    }

    /// Zero-based index of the declaration being checked when failure occurred.
    pub const fn declaration_index(&self) -> usize {
        self.declaration_index
    }

    /// Arena term responsible for a logical reference failure, when available.
    pub const fn term_id(&self) -> Option<TermId> {
        self.term_id
    }

    /// Whether the unavailable reference was local or global, when applicable.
    pub const fn reference_kind(&self) -> Option<ReferenceKind> {
        self.reference_kind
    }

    pub(super) const fn invalid_reference(
        declaration_index: usize,
        term_id: TermId,
        reference_kind: ReferenceKind,
    ) -> Self {
        Self {
            class: CheckErrorClass::InvalidJudgment,
            declaration_index,
            term_id: Some(term_id),
            reference_kind: Some(reference_kind),
        }
    }

    pub(super) const fn resource_exhausted(declaration_index: usize) -> Self {
        Self {
            class: CheckErrorClass::ResourceExhausted,
            declaration_index,
            term_id: None,
            reference_kind: None,
        }
    }
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.class {
            CheckErrorClass::InvalidJudgment => "invalid-judgment",
            CheckErrorClass::ResourceExhausted => "resource-exhausted",
        };
        f.write_str(label)
    }
}

impl std::error::Error for CheckError {}

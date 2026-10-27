//! Candidate propositions considered by the discovery engine.

use std::cmp::Ordering;

use crate::Term;

/// A non-reflexive equation with a canonical orientation.
///
/// The smaller term under the finder-side structural ordering is always stored
/// on the left. Thus s = t and t = s have the same representation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EquationCandidate {
    lhs: Term,
    rhs: Term,
}

impl EquationCandidate {
    /// Creates a canonical non-reflexive equation.
    ///
    /// Returns None for the syntactic tautology t = t.
    #[must_use]
    pub fn new(lhs: Term, rhs: Term) -> Option<Self> {
        match lhs.cmp(&rhs) {
            Ordering::Less => Some(Self { lhs, rhs }),
            Ordering::Equal => None,
            Ordering::Greater => Some(Self { lhs: rhs, rhs: lhs }),
        }
    }

    /// Returns the canonical left-hand side.
    #[must_use]
    pub fn lhs(&self) -> &Term {
        &self.lhs
    }

    /// Returns the canonical right-hand side.
    #[must_use]
    pub fn rhs(&self) -> &Term {
        &self.rhs
    }
}

/// Counts the successive cheap structural filtering stages for equations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandidateCounts {
    ordered_with_reflexive: u128,
    ordered_non_reflexive: u128,
    canonical_non_reflexive: u128,
}

impl CandidateCounts {
    /// Computes candidate counts from the number of distinct terms.
    ///
    /// Returns None if an intermediate exact count exceeds u128.
    #[must_use]
    pub fn from_term_count(term_count: u128) -> Option<Self> {
        let ordered_with_reflexive = term_count.checked_mul(term_count)?;
        let ordered_non_reflexive = if term_count == 0 {
            0
        } else {
            term_count.checked_mul(term_count - 1)?
        };
        let canonical_non_reflexive = ordered_non_reflexive / 2;

        Some(Self {
            ordered_with_reflexive,
            ordered_non_reflexive,
            canonical_non_reflexive,
        })
    }

    /// Returns the number of ordered equations before removing t = t.
    #[must_use]
    pub fn ordered_with_reflexive(self) -> u128 {
        self.ordered_with_reflexive
    }

    /// Returns the number of ordered equations after removing t = t.
    #[must_use]
    pub fn ordered_non_reflexive(self) -> u128 {
        self.ordered_non_reflexive
    }

    /// Returns the number after also identifying s = t with t = s.
    #[must_use]
    pub fn canonical_non_reflexive(self) -> u128 {
        self.canonical_non_reflexive
    }
}

#[cfg(test)]
mod tests {
    use super::{CandidateCounts, EquationCandidate};
    use crate::Term;

    #[test]
    fn rejects_reflexive_candidate() {
        assert!(EquationCandidate::new(Term::Var(0), Term::Var(0)).is_none());
    }

    #[test]
    fn canonicalizes_equation_orientation() {
        let forward = EquationCandidate::new(Term::Var(0), Term::One).unwrap();
        let reverse = EquationCandidate::new(Term::One, Term::Var(0)).unwrap();

        assert_eq!(forward, reverse);
        assert!(forward.lhs() < forward.rhs());
    }

    #[test]
    fn candidate_counts_record_structural_pruning() {
        let counts = CandidateCounts::from_term_count(55).unwrap();

        assert_eq!(counts.ordered_with_reflexive(), 3_025);
        assert_eq!(counts.ordered_non_reflexive(), 2_970);
        assert_eq!(counts.canonical_non_reflexive(), 1_485);
    }

    #[test]
    fn empty_term_set_has_no_candidates() {
        assert_eq!(
            CandidateCounts::from_term_count(0),
            Some(CandidateCounts {
                ordered_with_reflexive: 0,
                ordered_non_reflexive: 0,
                canonical_non_reflexive: 0,
            })
        );
    }
}

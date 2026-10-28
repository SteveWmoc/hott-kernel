//! Candidate propositions considered by the discovery engine.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

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

    /// Returns the least representative under bijective renaming of variables.
    ///
    /// Variable names carry no meaning after universal closure. The initial
    /// experiment uses at most three variables, so this exact implementation
    /// simply checks every bijection onto 0, ..., k - 1.
    #[must_use]
    pub fn canonicalize_variables(&self) -> Self {
        let mut variables = BTreeSet::new();
        self.lhs.collect_variables(&mut variables);
        self.rhs.collect_variables(&mut variables);

        let originals: Vec<_> = variables.into_iter().collect();
        if originals.is_empty() {
            return self.clone();
        }

        let variable_count =
            u32::try_from(originals.len()).expect("finder variable count must fit in u32");
        let mut targets: Vec<_> = (0..variable_count).collect();
        let mut best = None;

        for_each_permutation(&mut targets, 0, &mut |permutation| {
            let mapping: BTreeMap<_, _> = originals
                .iter()
                .copied()
                .zip(permutation.iter().copied())
                .collect();
            let candidate = Self::new(
                self.lhs.rename_variables(&mapping),
                self.rhs.rename_variables(&mapping),
            )
            .expect("renaming preserves non-reflexivity");

            match &best {
                Some(current) if current <= &candidate => {}
                _ => best = Some(candidate),
            }
        });

        best.expect("a nonempty variable set has at least one permutation")
    }
}

fn for_each_permutation<F>(values: &mut [u32], start: usize, visit: &mut F)
where
    F: FnMut(&[u32]),
{
    if start == values.len() {
        visit(values);
        return;
    }

    for index in start..values.len() {
        values.swap(start, index);
        for_each_permutation(values, start + 1, visit);
        values.swap(start, index);
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
    fn canonicalizes_uniform_variable_renaming() {
        let first = EquationCandidate::new(
            Term::Add(Box::new(Term::Var(2)), Box::new(Term::Var(0))),
            Term::Var(1),
        )
        .unwrap()
        .canonicalize_variables();
        let second = EquationCandidate::new(
            Term::Add(Box::new(Term::Var(1)), Box::new(Term::Var(2))),
            Term::Var(0),
        )
        .unwrap()
        .canonicalize_variables();

        assert_eq!(first, second);
    }

    #[test]
    fn variable_canonicalization_does_not_merge_distinct_variables() {
        let candidate =
            EquationCandidate::new(Term::Var(2), Term::Var(7)).unwrap().canonicalize_variables();

        assert_eq!(candidate.lhs(), &Term::Var(0));
        assert_eq!(candidate.rhs(), &Term::Var(1));
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

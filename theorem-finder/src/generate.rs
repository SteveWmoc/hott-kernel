//! Bounded term generation for the first theorem-finding experiment.

use std::collections::BTreeSet;

use crate::Term;

/// Returns the atomic terms for the requested number of free variables.
#[must_use]
pub fn atomic_terms(variable_count: u32) -> BTreeSet<Term> {
    let mut terms = BTreeSet::new();

    for variable in 0..variable_count {
        terms.insert(Term::Var(variable));
    }
    terms.insert(Term::Zero);
    terms.insert(Term::One);

    terms
}

/// Generates terms grouped by exact AST depth.
///
/// The returned vector has length `max_depth + 1`; entry `d` contains exactly
/// the terms whose [`Term::depth`] is `d`. Generation is deterministic and
/// exact-AST duplicates are removed.
///
/// This function materializes the search space and therefore grows extremely
/// quickly. Use [`count_terms_up_to_depth`] when only the size of a deeper
/// bounded search space is needed.
#[must_use]
pub fn terms_by_exact_depth(variable_count: u32, max_depth: usize) -> Vec<BTreeSet<Term>> {
    let atoms = atomic_terms(variable_count);
    let mut layers = vec![atoms.clone()];
    let mut up_to_previous = atoms;

    for depth in 1..=max_depth {
        let mut layer = BTreeSet::new();

        for left in &up_to_previous {
            for right in &up_to_previous {
                if 1 + left.depth().max(right.depth()) != depth {
                    continue;
                }

                layer.insert(Term::Add(
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ));
                layer.insert(Term::Mul(
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ));
            }
        }

        up_to_previous.extend(layer.iter().cloned());
        layers.push(layer);
    }

    layers
}

/// Counts terms of depth at most `max_depth` without materializing them.
///
/// With `a = variable_count + 2` atoms and two binary constructors, the count
/// satisfies
///
/// `T(0) = a` and `T(d + 1) = a + 2 * T(d)^2`.
///
/// Returns `None` if the exact count exceeds [`u128`].
#[must_use]
pub fn count_terms_up_to_depth(variable_count: u32, max_depth: usize) -> Option<u128> {
    let atoms = u128::from(variable_count).checked_add(2)?;
    let mut count = atoms;

    for _ in 0..max_depth {
        count = count
            .checked_mul(count)?
            .checked_mul(2)?
            .checked_add(atoms)?;
    }

    Some(count)
}

/// Counts terms of exactly `depth` without materializing them.
///
/// Returns `None` if the corresponding bounded count exceeds [`u128`].
#[must_use]
pub fn count_terms_exact_depth(variable_count: u32, depth: usize) -> Option<u128> {
    if depth == 0 {
        return u128::from(variable_count).checked_add(2);
    }

    let current = count_terms_up_to_depth(variable_count, depth)?;
    let previous = count_terms_up_to_depth(variable_count, depth - 1)?;
    current.checked_sub(previous)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        atomic_terms, count_terms_exact_depth, count_terms_up_to_depth, terms_by_exact_depth,
    };

    #[test]
    fn three_variable_atoms_are_variables_zero_and_one() {
        let atoms = atomic_terms(3);
        assert_eq!(atoms.len(), 5);
    }

    #[test]
    fn materialized_layers_have_expected_counts() {
        let layers = terms_by_exact_depth(3, 2);

        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].len(), 5);
        assert_eq!(layers[1].len(), 50);
        assert_eq!(layers[2].len(), 6_000);
        assert_eq!(layers.iter().map(BTreeSet::len).sum::<usize>(), 6_055);
    }

    #[test]
    fn every_term_is_in_the_claimed_depth_layer() {
        let layers = terms_by_exact_depth(3, 2);

        for (depth, layer) in layers.iter().enumerate() {
            assert!(layer.iter().all(|term| term.depth() == depth));
        }
    }

    #[test]
    fn symbolic_counts_match_materialized_small_depths() {
        assert_eq!(count_terms_up_to_depth(3, 0), Some(5));
        assert_eq!(count_terms_up_to_depth(3, 1), Some(55));
        assert_eq!(count_terms_up_to_depth(3, 2), Some(6_055));
        assert_eq!(count_terms_up_to_depth(3, 3), Some(73_326_055));

        assert_eq!(count_terms_exact_depth(3, 0), Some(5));
        assert_eq!(count_terms_exact_depth(3, 1), Some(50));
        assert_eq!(count_terms_exact_depth(3, 2), Some(6_000));
        assert_eq!(count_terms_exact_depth(3, 3), Some(73_320_000));
    }
}

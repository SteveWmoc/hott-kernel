//! Exact normal forms for the free noncommutative semiring.

use std::collections::{BTreeMap, BTreeSet};

use crate::Term;

/// A word in the free monoid on the finder variables.
///
/// The empty word represents the multiplicative unit.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Monomial(Vec<u32>);

impl Monomial {
    fn one() -> Self {
        Self(Vec::new())
    }

    fn variable(variable: u32) -> Self {
        Self(vec![variable])
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut factors = self.0.clone();
        factors.extend_from_slice(&other.0);
        Self(factors)
    }

    pub(crate) fn collect_variables(&self, variables: &mut BTreeSet<u32>) {
        variables.extend(self.0.iter().copied());
    }

    pub(crate) fn rename_variables(&self, mapping: &BTreeMap<u32, u32>) -> Self {
        Self(
            self.0
                .iter()
                .map(|variable| {
                    *mapping
                        .get(variable)
                        .expect("variable-renaming map must cover every variable")
                })
                .collect(),
        )
    }

    /// Returns the variable word represented by this monomial.
    #[must_use]
    pub fn factors(&self) -> &[u32] {
        &self.0
    }
}

/// Canonical free-noncommutative-semiring normal form.
///
/// A value is a sorted multiset of monomials. The empty multiset is zero.
/// Duplicate monomials encode natural-number coefficients.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SemiringNormalForm(Vec<Monomial>);

impl SemiringNormalForm {
    /// Computes the exact normal form of a finder-side term.
    #[must_use]
    pub fn from_term(term: &Term) -> Self {
        match term {
            Term::Var(variable) => Self(vec![Monomial::variable(*variable)]),
            Term::Zero => Self(Vec::new()),
            Term::One => Self(vec![Monomial::one()]),
            Term::Add(left, right) => {
                let mut summands = Self::from_term(left).0;
                summands.extend(Self::from_term(right).0);
                summands.sort();
                Self(summands)
            }
            Term::Mul(left, right) => {
                let left = Self::from_term(left);
                let right = Self::from_term(right);

                if left.0.is_empty() || right.0.is_empty() {
                    return Self(Vec::new());
                }

                let mut summands = Vec::with_capacity(left.0.len() * right.0.len());
                for left_monomial in &left.0 {
                    for right_monomial in &right.0 {
                        summands.push(left_monomial.multiply(right_monomial));
                    }
                }
                summands.sort();
                Self(summands)
            }
        }
    }

    /// Returns the sorted multiset of monomials.
    #[must_use]
    pub fn summands(&self) -> &[Monomial] {
        &self.0
    }

    pub(crate) fn collect_variables(&self, variables: &mut BTreeSet<u32>) {
        for monomial in &self.0 {
            monomial.collect_variables(variables);
        }
    }

    pub(crate) fn rename_variables(&self, mapping: &BTreeMap<u32, u32>) -> Self {
        let mut summands: Vec<_> = self
            .0
            .iter()
            .map(|monomial| monomial.rename_variables(mapping))
            .collect();
        summands.sort();
        Self(summands)
    }
}

#[cfg(test)]
mod tests {
    use super::SemiringNormalForm;
    use crate::Term;

    fn var(index: u32) -> Term {
        Term::Var(index)
    }

    #[test]
    fn normalizes_zero_and_one_laws() {
        let x = var(0);

        assert_eq!(
            SemiringNormalForm::from_term(&Term::Add(
                Box::new(x.clone()),
                Box::new(Term::Zero)
            )),
            SemiringNormalForm::from_term(&x)
        );
        assert_eq!(
            SemiringNormalForm::from_term(&Term::Mul(
                Box::new(x.clone()),
                Box::new(Term::One)
            )),
            SemiringNormalForm::from_term(&x)
        );
        assert_eq!(
            SemiringNormalForm::from_term(&Term::Mul(
                Box::new(Term::Zero),
                Box::new(x)
            )),
            SemiringNormalForm::from_term(&Term::Zero)
        );
    }

    #[test]
    fn normalizes_additive_commutativity_and_associativity() {
        let x = var(0);
        let y = var(1);
        let z = var(2);

        let left = Term::Add(
            Box::new(Term::Add(Box::new(x.clone()), Box::new(y.clone()))),
            Box::new(z.clone()),
        );
        let right = Term::Add(
            Box::new(z),
            Box::new(Term::Add(Box::new(y), Box::new(x))),
        );

        assert_eq!(
            SemiringNormalForm::from_term(&left),
            SemiringNormalForm::from_term(&right)
        );
    }

    #[test]
    fn normalizes_both_distributive_laws() {
        let x = var(0);
        let y = var(1);
        let z = var(2);

        let left_distributed = Term::Mul(
            Box::new(x.clone()),
            Box::new(Term::Add(Box::new(y.clone()), Box::new(z.clone()))),
        );
        let left_expanded = Term::Add(
            Box::new(Term::Mul(Box::new(x.clone()), Box::new(y.clone()))),
            Box::new(Term::Mul(Box::new(x.clone()), Box::new(z.clone()))),
        );

        let right_distributed = Term::Mul(
            Box::new(Term::Add(Box::new(x.clone()), Box::new(y.clone()))),
            Box::new(z.clone()),
        );
        let right_expanded = Term::Add(
            Box::new(Term::Mul(Box::new(x), Box::new(z.clone()))),
            Box::new(Term::Mul(Box::new(y), Box::new(z))),
        );

        assert_eq!(
            SemiringNormalForm::from_term(&left_distributed),
            SemiringNormalForm::from_term(&left_expanded)
        );
        assert_eq!(
            SemiringNormalForm::from_term(&right_distributed),
            SemiringNormalForm::from_term(&right_expanded)
        );
    }

    #[test]
    fn multiplication_remains_noncommutative() {
        let xy = Term::Mul(Box::new(var(0)), Box::new(var(1)));
        let yx = Term::Mul(Box::new(var(1)), Box::new(var(0)));

        assert_ne!(
            SemiringNormalForm::from_term(&xy),
            SemiringNormalForm::from_term(&yx)
        );
    }

    #[test]
    fn repeated_monomials_preserve_coefficients() {
        let x = var(0);
        let two_x = Term::Add(Box::new(x.clone()), Box::new(x.clone()));

        assert_ne!(
            SemiringNormalForm::from_term(&two_x),
            SemiringNormalForm::from_term(&x)
        );
        assert_eq!(SemiringNormalForm::from_term(&two_x).summands().len(), 2);
    }
}

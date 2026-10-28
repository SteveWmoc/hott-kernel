//! Finder-side syntax for the first bounded algebra experiment.

use std::collections::{BTreeMap, BTreeSet};

/// A one-sorted algebraic term used by the initial theorem-finding experiment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Term {
    /// A de Bruijn-like numbered free variable in the bounded experiment.
    Var(u32),
    /// The constant zero.
    Zero,
    /// The constant one.
    One,
    /// Addition.
    Add(Box<Term>, Box<Term>),
    /// Multiplication.
    Mul(Box<Term>, Box<Term>),
}

impl Term {
    /// Returns the AST depth, with atoms at depth zero.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::Var(_) | Self::Zero | Self::One => 0,
            Self::Add(left, right) | Self::Mul(left, right) => 1 + left.depth().max(right.depth()),
        }
    }

    /// Returns the number of AST nodes.
    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            Self::Var(_) | Self::Zero | Self::One => 1,
            Self::Add(left, right) | Self::Mul(left, right) => 1 + left.size() + right.size(),
        }
    }

    pub(crate) fn collect_variables(&self, variables: &mut BTreeSet<u32>) {
        match self {
            Self::Var(variable) => {
                variables.insert(*variable);
            }
            Self::Zero | Self::One => {}
            Self::Add(left, right) | Self::Mul(left, right) => {
                left.collect_variables(variables);
                right.collect_variables(variables);
            }
        }
    }

    pub(crate) fn rename_variables(&self, mapping: &BTreeMap<u32, u32>) -> Self {
        match self {
            Self::Var(variable) => Self::Var(
                *mapping
                    .get(variable)
                    .expect("variable-renaming map must cover every variable"),
            ),
            Self::Zero => Self::Zero,
            Self::One => Self::One,
            Self::Add(left, right) => Self::Add(
                Box::new(left.rename_variables(mapping)),
                Box::new(right.rename_variables(mapping)),
            ),
            Self::Mul(left, right) => Self::Mul(
                Box::new(left.rename_variables(mapping)),
                Box::new(right.rename_variables(mapping)),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::Term;

    #[test]
    fn reports_depth_and_size() {
        let term = Term::Mul(
            Box::new(Term::Add(Box::new(Term::Var(0)), Box::new(Term::Zero))),
            Box::new(Term::One),
        );

        assert_eq!(term.depth(), 2);
        assert_eq!(term.size(), 5);
    }

    #[test]
    fn collects_and_renames_variables_uniformly() {
        let term = Term::Add(Box::new(Term::Var(7)), Box::new(Term::Var(2)));
        let mut variables = BTreeSet::new();
        term.collect_variables(&mut variables);

        assert_eq!(variables, BTreeSet::from([2, 7]));

        let mapping = BTreeMap::from([(2, 0), (7, 1)]);
        assert_eq!(
            term.rename_variables(&mapping),
            Term::Add(Box::new(Term::Var(1)), Box::new(Term::Var(0)))
        );
    }
}

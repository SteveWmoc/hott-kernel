//! Finder-side syntax for the first bounded algebra experiment.

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
}

#[cfg(test)]
mod tests {
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
}

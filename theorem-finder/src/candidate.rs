//! Candidate propositions considered by the discovery engine.

use crate::Term;

/// A candidate universally quantified equation between finder-side terms.
///
/// Quantifier/context information will be added when bounded generation is
/// implemented; this first shell records the equation itself.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EquationCandidate {
    lhs: Term,
    rhs: Term,
}

impl EquationCandidate {
    /// Creates a candidate equation.
    #[must_use]
    pub fn new(lhs: Term, rhs: Term) -> Self {
        Self { lhs, rhs }
    }

    /// Returns the left-hand side.
    #[must_use]
    pub fn lhs(&self) -> &Term {
        &self.lhs
    }

    /// Returns the right-hand side.
    #[must_use]
    pub fn rhs(&self) -> &Term {
        &self.rhs
    }

    /// Returns whether the candidate is syntactically reflexive.
    #[must_use]
    pub fn is_reflexive(&self) -> bool {
        self.lhs == self.rhs
    }
}

#[cfg(test)]
mod tests {
    use super::EquationCandidate;
    use crate::Term;

    #[test]
    fn detects_reflexive_candidate() {
        let candidate = EquationCandidate::new(Term::Var(0), Term::Var(0));
        assert!(candidate.is_reflexive());
    }
}

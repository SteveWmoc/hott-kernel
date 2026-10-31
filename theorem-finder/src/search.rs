//! First bounded proof-search baseline over the explicit semiring axioms.

use crate::{
    BridgeError, EquationCandidate, Term, check_candidate_with_proof,
    kernel_bridge::render_axiom_application,
};

/// Explicit semiring axioms available to the baseline prover.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemiringAxiom {
    AddAssoc,
    AddComm,
    AddZeroRight,
    MulAssoc,
    MulOneLeft,
    MulOneRight,
    LeftDistrib,
    RightDistrib,
    ZeroMul,
    MulZero,
}

impl SemiringAxiom {
    /// Stable Surface declaration name for this axiom.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AddAssoc => "add_assoc",
            Self::AddComm => "add_comm",
            Self::AddZeroRight => "add_zero_right",
            Self::MulAssoc => "mul_assoc",
            Self::MulOneLeft => "mul_one_left",
            Self::MulOneRight => "mul_one_right",
            Self::LeftDistrib => "left_distrib",
            Self::RightDistrib => "right_distrib",
            Self::ZeroMul => "zero_mul",
            Self::MulZero => "mul_zero",
        }
    }

    fn specification(self) -> (Pattern, Pattern, usize) {
        use Pattern::{Add, Meta, Mul, One, Zero};

        match self {
            Self::AddAssoc => (
                Add(Box::new(Meta(0)), Box::new(Add(Box::new(Meta(1)), Box::new(Meta(2))))),
                Add(Box::new(Add(Box::new(Meta(0)), Box::new(Meta(1)))), Box::new(Meta(2))),
                3,
            ),
            Self::AddComm => (
                Add(Box::new(Meta(0)), Box::new(Meta(1))),
                Add(Box::new(Meta(1)), Box::new(Meta(0))),
                2,
            ),
            Self::AddZeroRight => (
                Meta(0),
                Add(Box::new(Meta(0)), Box::new(Zero)),
                1,
            ),
            Self::MulAssoc => (
                Mul(Box::new(Meta(0)), Box::new(Mul(Box::new(Meta(1)), Box::new(Meta(2))))),
                Mul(Box::new(Mul(Box::new(Meta(0)), Box::new(Meta(1)))), Box::new(Meta(2))),
                3,
            ),
            Self::MulOneLeft => (
                Meta(0),
                Mul(Box::new(One), Box::new(Meta(0))),
                1,
            ),
            Self::MulOneRight => (
                Meta(0),
                Mul(Box::new(Meta(0)), Box::new(One)),
                1,
            ),
            Self::LeftDistrib => (
                Add(
                    Box::new(Mul(Box::new(Meta(0)), Box::new(Meta(1)))),
                    Box::new(Mul(Box::new(Meta(0)), Box::new(Meta(2)))),
                ),
                Mul(
                    Box::new(Meta(0)),
                    Box::new(Add(Box::new(Meta(1)), Box::new(Meta(2)))),
                ),
                3,
            ),
            Self::RightDistrib => (
                Add(
                    Box::new(Mul(Box::new(Meta(0)), Box::new(Meta(2)))),
                    Box::new(Mul(Box::new(Meta(1)), Box::new(Meta(2)))),
                ),
                Mul(
                    Box::new(Add(Box::new(Meta(0)), Box::new(Meta(1)))),
                    Box::new(Meta(2)),
                ),
                3,
            ),
            Self::ZeroMul => (
                Zero,
                Mul(Box::new(Zero), Box::new(Meta(0))),
                1,
            ),
            Self::MulZero => (
                Zero,
                Mul(Box::new(Meta(0)), Box::new(Zero)),
                1,
            ),
        }
    }

    fn match_candidate(self, candidate: &EquationCandidate) -> Option<Vec<Term>> {
        let (lhs_pattern, rhs_pattern, arity) = self.specification();
        let mut substitution = vec![None; arity];

        if !match_pattern(&lhs_pattern, candidate.lhs(), &mut substitution)
            || !match_pattern(&rhs_pattern, candidate.rhs(), &mut substitution)
        {
            return None;
        }

        substitution.into_iter().collect()
    }
}

/// Stable search order for the baseline prover.
pub const DIRECT_AXIOMS: [SemiringAxiom; 10] = [
    SemiringAxiom::AddAssoc,
    SemiringAxiom::AddComm,
    SemiringAxiom::AddZeroRight,
    SemiringAxiom::MulAssoc,
    SemiringAxiom::MulOneLeft,
    SemiringAxiom::MulOneRight,
    SemiringAxiom::LeftDistrib,
    SemiringAxiom::RightDistrib,
    SemiringAxiom::ZeroMul,
    SemiringAxiom::MulZero,
];

#[derive(Clone, Debug)]
enum Pattern {
    Meta(usize),
    Zero,
    One,
    Add(Box<Pattern>, Box<Pattern>),
    Mul(Box<Pattern>, Box<Pattern>),
}

fn match_pattern(
    pattern: &Pattern,
    term: &Term,
    substitution: &mut [Option<Term>],
) -> bool {
    match (pattern, term) {
        (Pattern::Meta(index), term) => match &substitution[*index] {
            Some(existing) => existing == term,
            None => {
                substitution[*index] = Some(term.clone());
                true
            }
        },
        (Pattern::Zero, Term::Zero) | (Pattern::One, Term::One) => true,
        (Pattern::Add(left_pattern, right_pattern), Term::Add(left, right))
        | (Pattern::Mul(left_pattern, right_pattern), Term::Mul(left, right)) => {
            match_pattern(left_pattern, left, substitution)
                && match_pattern(right_pattern, right, substitution)
        }
        _ => false,
    }
}

/// Kernel-checked proof found by the direct-axiom baseline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectProof {
    axiom: SemiringAxiom,
    attempts: usize,
    proof_term: String,
    core_bytes: Vec<u8>,
}

impl DirectProof {
    /// Axiom used by the proof.
    #[must_use]
    pub const fn axiom(&self) -> SemiringAxiom {
        self.axiom
    }

    /// Number of axiom schemas examined before success.
    #[must_use]
    pub const fn attempts(&self) -> usize {
        self.attempts
    }

    /// Generated untrusted Surface proof term.
    #[must_use]
    pub fn proof_term(&self) -> &str {
        &self.proof_term
    }

    /// Size of the canonical checked Core artifact containing the proof.
    #[must_use]
    pub fn checked_core_size(&self) -> usize {
        self.core_bytes.len()
    }

    /// Dependency trace for this first baseline.
    #[must_use]
    pub fn dependencies(&self) -> [SemiringAxiom; 1] {
        [self.axiom]
    }
}

/// Result of bounded direct-axiom proof search.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectSearchResult {
    Proved(DirectProof),
    NotProved { attempts: usize },
}

/// Attempts to prove a candidate by one direct instance of an explicit axiom.
///
/// A syntactic schema match is never accepted on its own. A proposed proof is
/// rendered as ordinary Surface source and must pass the public Core checker.
pub fn search_direct_axiom(
    declaration_name: &str,
    candidate: &EquationCandidate,
) -> Result<DirectSearchResult, BridgeError> {
    for (index, axiom) in DIRECT_AXIOMS.iter().copied().enumerate() {
        let Some(arguments) = axiom.match_candidate(candidate) else {
            continue;
        };

        let proof_term = render_axiom_application(candidate, axiom.name(), &arguments);
        let core_bytes = check_candidate_with_proof(declaration_name, candidate, &proof_term)?;

        return Ok(DirectSearchResult::Proved(DirectProof {
            axiom,
            attempts: index + 1,
            proof_term,
            core_bytes,
        }));
    }

    Ok(DirectSearchResult::NotProved {
        attempts: DIRECT_AXIOMS.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::{DirectSearchResult, SemiringAxiom, search_direct_axiom};
    use crate::{
        EquationCandidate, Term, semiring_identity_candidates, terms_up_to_depth,
    };

    #[test]
    fn proves_direct_additive_identity_instance() {
        let candidate = EquationCandidate::new(
            Term::Var(0),
            Term::Add(Box::new(Term::Var(0)), Box::new(Term::Zero)),
        )
        .unwrap();

        let result = search_direct_axiom("direct_add_zero", &candidate).unwrap();

        let DirectSearchResult::Proved(proof) = result else {
            panic!("direct additive identity instance should be proved");
        };
        assert_eq!(proof.axiom(), SemiringAxiom::AddZeroRight);
        assert_eq!(proof.dependencies(), [SemiringAxiom::AddZeroRight]);
        assert!(proof.checked_core_size() > 0);
    }

    #[test]
    fn does_not_claim_nondirect_identity() {
        let candidate = EquationCandidate::new(
            Term::Add(
                Box::new(Term::Add(Box::new(Term::Var(0)), Box::new(Term::Zero))),
                Box::new(Term::Zero),
            ),
            Term::Var(0),
        )
        .unwrap();

        assert!(matches!(
            search_direct_axiom("not_direct", &candidate).unwrap(),
            DirectSearchResult::NotProved { .. }
        ));
    }

    #[test]
    fn depth_one_baseline_proves_fifteen_of_forty_two_identity_classes() {
        let terms = terms_up_to_depth(3, 1);
        let candidates = semiring_identity_candidates(&terms);

        assert_eq!(candidates.len(), 42);

        let proved = candidates
            .iter()
            .enumerate()
            .filter(|(index, candidate)| {
                matches!(
                    search_direct_axiom(&format!("baseline_{index}"), candidate).unwrap(),
                    DirectSearchResult::Proved(_)
                )
            })
            .count();

        assert_eq!(proved, 15);
    }
}

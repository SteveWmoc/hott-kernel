//! Boundary between untrusted discovery code and the trusted checker.
//!
//! Finder output crosses only the ordinary public hott-kernel Surface API.
//! This module has no privileged access to checker internals.

use std::collections::{BTreeMap, BTreeSet};

use crate::{EquationCandidate, Term};

pub use hott_kernel as kernel;

/// Self-contained experimental semiring theory written in Surface v0.1.
pub const SEMIRING_SURFACE: &str = include_str!("../theory/semiring.surface");

/// Renders a finder term as a Surface term using the supplied variable names.
fn render_term(term: &Term, variables: &BTreeMap<u32, String>) -> String {
    match term {
        Term::Var(variable) => {
            let name = variables
                .get(variable)
                .expect("rendering environment must contain every free variable");
            format!("(ref {name})")
        }
        Term::Zero => "(ref zeroA)".to_owned(),
        Term::One => "(ref oneA)".to_owned(),
        Term::Add(left, right) => format!(
            "(app (app (ref add) {}) {})",
            render_term(left, variables),
            render_term(right, variables)
        ),
        Term::Mul(left, right) => format!(
            "(app (app (ref mul) {}) {})",
            render_term(left, variables),
            render_term(right, variables)
        ),
    }
}

/// Renders a finder candidate as its universally closed Surface proposition.
///
/// Free-variable identifiers are irrelevant to the logical proposition. They
/// are collected in sorted order and rebound as x0, x1, ... in the emitted
/// Surface term.
#[must_use]
pub fn render_equation_type(candidate: &EquationCandidate) -> String {
    let mut free_variables = BTreeSet::new();
    candidate.lhs().collect_variables(&mut free_variables);
    candidate.rhs().collect_variables(&mut free_variables);

    let variable_names: BTreeMap<_, _> = free_variables
        .iter()
        .enumerate()
        .map(|(index, variable)| (*variable, format!("x{index}")))
        .collect();

    let lhs = render_term(candidate.lhs(), &variable_names);
    let rhs = render_term(candidate.rhs(), &variable_names);
    let mut proposition = format!("(id (ref A) {lhs} {rhs})");

    for variable in free_variables.iter().rev() {
        let name = variable_names
            .get(variable)
            .expect("collected variable must have a binder name");
        proposition = format!("(pi {name} (ref A) {proposition})");
    }

    proposition
}

/// Renders a transparent Surface declaration for a candidate and proof term.
///
/// Both the declaration name and proof text are untrusted input. Surface
/// parsing and the Core checker remain the only acceptance boundary.
#[must_use]
pub fn render_transparent_candidate(
    declaration_name: &str,
    candidate: &EquationCandidate,
    proof_term: &str,
) -> String {
    format!(
        "(transparent {declaration_name}\n  {}\n  {proof_term})\n",
        render_equation_type(candidate)
    )
}

/// Appends a generated declaration to the experimental semiring theory.
#[must_use]
pub fn semiring_source_with(declaration: &str) -> String {
    let mut source = String::with_capacity(SEMIRING_SURFACE.len() + declaration.len() + 2);
    source.push_str(SEMIRING_SURFACE);
    source.push('\n');
    source.push_str(declaration);
    source
}

/// Sends a generated candidate proof through the public Surface-to-Core checker.
pub fn check_candidate_with_proof(
    declaration_name: &str,
    candidate: &EquationCandidate,
    proof_term: &str,
) -> Result<Vec<u8>, kernel::SurfaceCheckError> {
    let declaration = render_transparent_candidate(declaration_name, candidate, proof_term);
    kernel::compile_and_check_surface(semiring_source_with(&declaration).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        SEMIRING_SURFACE, check_candidate_with_proof, kernel, render_equation_type,
        render_transparent_candidate,
    };
    use crate::{EquationCandidate, Term};

    #[test]
    fn experimental_semiring_theory_checks() {
        kernel::compile_and_check_surface(SEMIRING_SURFACE.as_bytes())
            .expect("experimental semiring theory must check");
    }

    #[test]
    fn renders_universal_finder_equation() {
        let candidate = EquationCandidate::new(
            Term::Add(Box::new(Term::Var(7)), Box::new(Term::Zero)),
            Term::Var(7),
        )
        .unwrap();

        assert_eq!(
            render_equation_type(&candidate),
            "(pi x0 (ref A) (id (ref A) (ref x0) (app (app (ref add) (ref x0)) (ref zeroA))))"
        );
    }

    #[test]
    fn generated_candidate_can_use_checked_semiring_axiom() {
        let candidate = EquationCandidate::new(
            Term::Add(Box::new(Term::Var(0)), Box::new(Term::Zero)),
            Term::Var(0),
        )
        .unwrap();

        let core = check_candidate_with_proof(
            "finder_add_zero_right",
            &candidate,
            "(lam x0 (app (ref add_zero_right) (ref x0)))",
        )
        .expect("matching generated candidate proof must check");

        assert!(!core.is_empty());
    }

    #[test]
    fn kernel_rejects_wrong_proof_for_generated_candidate() {
        let candidate = EquationCandidate::new(Term::Var(0), Term::One).unwrap();

        let error = check_candidate_with_proof(
            "finder_bad",
            &candidate,
            "(lam x0 (app (ref add_zero_right) (ref x0)))",
        )
        .expect_err("mismatched generated proof must be rejected");

        assert!(matches!(error, kernel::SurfaceCheckError::Check(_)));
    }

    #[test]
    fn rendered_declaration_remains_plain_surface_source() {
        let candidate = EquationCandidate::new(Term::Var(0), Term::One).unwrap();
        let declaration =
            render_transparent_candidate("candidate", &candidate, "(lam x0 (refl (ref x0)))");

        assert!(declaration.starts_with("(transparent candidate"));
        assert!(declaration.contains("(pi x0 (ref A)"));
    }
}

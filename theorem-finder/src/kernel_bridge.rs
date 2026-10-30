//! Boundary between untrusted discovery code and the trusted checker.
//!
//! Finder output crosses only the ordinary public hott-kernel Surface API.
//! This module has no privileged access to checker internals.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use crate::{EquationCandidate, Term};

pub use hott_kernel as kernel;

/// Self-contained experimental semiring theory written in Surface v0.1.
pub const SEMIRING_SURFACE: &str = include_str!("../theory/semiring.surface");

/// Failure while constructing or checking a finder-generated declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BridgeError {
    /// The supplied declaration name was not exactly one legal Surface identifier.
    InvalidDeclarationName,
    /// The generated suffix did not parse as exactly one transparent declaration.
    InvalidGeneratedDeclaration,
    /// The ordinary Surface/Core checking pipeline rejected the declaration.
    Check(kernel::SurfaceCheckError),
}

impl fmt::Display for BridgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeclarationName => {
                formatter.write_str("invalid theorem-finder declaration name")
            }
            Self::InvalidGeneratedDeclaration => {
                formatter.write_str("invalid theorem-finder generated declaration")
            }
            Self::Check(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for BridgeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidDeclarationName | Self::InvalidGeneratedDeclaration => None,
            Self::Check(error) => Some(error),
        }
    }
}

impl From<kernel::SurfaceCheckError> for BridgeError {
    fn from(error: kernel::SurfaceCheckError) -> Self {
        Self::Check(error)
    }
}

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

fn is_exact_surface_identifier(name: &str) -> bool {
    let probe = format!("(surface 0 1)\n(module Validate)\n(postulate {name} unit)\n");
    let Ok(module) = kernel::parse_surface(probe.as_bytes()) else {
        return false;
    };

    matches!(
        module.declarations(),
        [declaration] if declaration.name().as_str() == name
    )
}

fn semiring_source_with(declaration: &str) -> String {
    let mut source = String::with_capacity(SEMIRING_SURFACE.len() + declaration.len() + 2);
    source.push_str(SEMIRING_SURFACE);
    source.push('\n');
    source.push_str(declaration);
    source
}

fn validates_as_single_appended_transparent(source: &str, declaration_name: &str) -> bool {
    let Ok(base) = kernel::parse_surface(SEMIRING_SURFACE.as_bytes()) else {
        return false;
    };
    let Ok(module) = kernel::parse_surface(source.as_bytes()) else {
        return false;
    };

    if module.declarations().len() != base.declarations().len() + 1 {
        return false;
    }

    matches!(
        module.declarations().last(),
        Some(kernel::SurfaceDeclaration::Transparent { name, .. })
            if name.as_str() == declaration_name
    )
}

/// Renders exactly one transparent Surface declaration for a candidate.
///
/// Both the declaration name and proof text are untrusted. The name must be
/// exactly one legal Surface identifier, and the completed source must parse as
/// exactly one new transparent declaration appended to the fixed semiring
/// theory. This prevents either interpolation point from injecting declarations.
pub fn render_transparent_candidate(
    declaration_name: &str,
    candidate: &EquationCandidate,
    proof_term: &str,
) -> Result<String, BridgeError> {
    if !is_exact_surface_identifier(declaration_name) {
        return Err(BridgeError::InvalidDeclarationName);
    }

    let declaration = format!(
        "(transparent {declaration_name}\n  {}\n  {proof_term})\n",
        render_equation_type(candidate)
    );
    let source = semiring_source_with(&declaration);

    if !validates_as_single_appended_transparent(&source, declaration_name) {
        return Err(BridgeError::InvalidGeneratedDeclaration);
    }

    Ok(declaration)
}

/// Sends a generated candidate proof through the public Surface-to-Core checker.
pub fn check_candidate_with_proof(
    declaration_name: &str,
    candidate: &EquationCandidate,
    proof_term: &str,
) -> Result<Vec<u8>, BridgeError> {
    let declaration = render_transparent_candidate(declaration_name, candidate, proof_term)?;
    Ok(kernel::compile_and_check_surface(
        semiring_source_with(&declaration).as_bytes(),
    )?)
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeError, SEMIRING_SURFACE, check_candidate_with_proof, kernel, render_equation_type,
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

        assert!(matches!(
            error,
            BridgeError::Check(kernel::SurfaceCheckError::Check(_))
        ));
    }

    #[test]
    fn rejects_declaration_name_injection() {
        let candidate = EquationCandidate::new(Term::Zero, Term::One).unwrap();
        let injected_name = "decoy (ref A) (ref zeroA)) (postulate invented (id (ref A) (ref zeroA) (ref oneA))) (transparent result";

        let error = check_candidate_with_proof(injected_name, &candidate, "(ref invented)")
            .expect_err("declaration-name injection must be rejected");

        assert_eq!(error, BridgeError::InvalidDeclarationName);
    }

    #[test]
    fn rejects_proof_term_declaration_injection() {
        let candidate = EquationCandidate::new(Term::Zero, Term::One).unwrap();
        let injected_proof =
            "(ref zeroA))\n(postulate invented (id (ref A) (ref zeroA) (ref oneA)))\n;";

        let error = check_candidate_with_proof("candidate", &candidate, injected_proof)
            .expect_err("proof-term declaration injection must be rejected");

        assert_eq!(error, BridgeError::InvalidGeneratedDeclaration);
    }

    #[test]
    fn rendered_declaration_remains_plain_surface_source() {
        let candidate = EquationCandidate::new(Term::Var(0), Term::One).unwrap();
        let declaration =
            render_transparent_candidate("candidate", &candidate, "(lam x0 (refl (ref x0)))")
                .expect("well-formed generated declaration must render");

        assert!(declaration.starts_with("(transparent candidate"));
        assert!(declaration.contains("(pi x0 (ref A)"));
    }
}

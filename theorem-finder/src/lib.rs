#![forbid(unsafe_code)]

//! Experimental, untrusted theorem-discovery machinery for hott-kernel.
//!
//! This crate may generate candidates and proof objects, but logical trust ends
//! at the public hott-kernel checker boundary.

pub mod candidate;
pub mod generate;
pub mod kernel_bridge;
pub mod normal_form;
pub mod term;

pub use candidate::{CandidateCounts, EquationCandidate, NormalizedEquationCandidate};
pub use kernel_bridge::{
    SEMIRING_SURFACE, check_candidate_with_proof, render_equation_type,
    render_transparent_candidate, semiring_source_with,
};
pub use generate::{
    atomic_terms, canonical_equations, canonical_normalized_equations,
    count_candidate_stages_up_to_depth, count_equations_mod_variable_renaming,
    count_normalized_equations_mod_variable_renaming, count_semiring_normal_forms,
    count_terms_exact_depth, count_terms_up_to_depth, equations_mod_variable_renaming,
    normalized_equations_mod_variable_renaming, semiring_normal_forms, terms_by_exact_depth,
    terms_up_to_depth,
};
pub use normal_form::{Monomial, SemiringNormalForm};
pub use term::Term;

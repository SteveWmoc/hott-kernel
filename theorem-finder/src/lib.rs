#![forbid(unsafe_code)]

//! Experimental, untrusted theorem-discovery machinery for hott-kernel.
//!
//! This crate may generate candidates and proof objects, but logical trust ends
//! at the public hott-kernel checker boundary.

pub mod candidate;
pub mod generate;
pub mod kernel_bridge;
pub mod term;

pub use candidate::{CandidateCounts, EquationCandidate};
pub use generate::{
    atomic_terms, canonical_equations, count_terms_exact_depth, count_terms_up_to_depth,
    terms_by_exact_depth, terms_up_to_depth,
};
pub use term::Term;

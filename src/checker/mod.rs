//! Logical checks over parsed Core v0.1 syntax.
//!
//! The public checker layer exposes reference-only validation, the complete
//! frozen Core v0.1 one-pass declaration checker, and deterministic foundation
//! audit extraction. Structural transformations, checked state, reduction,
//! normalization, conversion, motive recognition, and bidirectional typing
//! remain internal TCB layers.

mod audit;
mod declarations;
mod error;
mod references;
// Land and audit frozen checker primitives before their public consumers so
// each trusted slice remains mechanically reviewable.
#[cfg_attr(not(test), allow(dead_code))]
mod convert;
#[cfg(test)]
mod convert_coverage;
#[cfg(test)]
mod motive_tests;
#[cfg_attr(not(test), allow(dead_code))]
mod motives;
#[cfg_attr(not(test), allow(dead_code))]
mod reduce;
#[cfg_attr(not(test), allow(dead_code))]
mod state;
#[cfg_attr(not(test), allow(dead_code))]
mod transform;
#[cfg_attr(not(test), allow(dead_code))]
mod typecheck;
#[cfg(test)]
mod typecheck_tests;

pub use audit::{
    AuditDeclarationKind, AuditDependencies, DeclarationAudit, FEATURE_VOCABULARY, FoundationAudit,
    KernelFeature, check_and_extract_audit,
};
pub use declarations::check_module;
pub use error::{CheckError, CheckErrorClass, ReferenceKind};
pub use references::check_references;

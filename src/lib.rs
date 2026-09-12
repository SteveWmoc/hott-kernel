#![forbid(unsafe_code)]

//! Safe-Rust implementation layers for the frozen Core v0.1 contract.
//!
//! This crate implements syntax, serialization, reference-availability
//! validation, full Core v0.1 declaration checking, and deterministic
//! structural foundation-audit extraction. Artifact hashing and complete
//! foundation-manifest packaging remain separate Phase 1 work.

pub mod checker;
pub mod error;
pub mod format;
pub mod syntax;

pub use checker::{
    AuditDeclarationKind, AuditDependencies, CheckError, CheckErrorClass, DeclarationAudit,
    FEATURE_VOCABULARY, FoundationAudit, KernelFeature, ReferenceKind, check_and_extract_audit,
    check_module, check_references,
};
pub use error::{FormatError, FormatErrorClass};
pub use format::{parse_canonical, parse_transport, print_canonical, print_semantic};
pub use syntax::{Arena, Declaration, Module, Natural, Term, TermId};

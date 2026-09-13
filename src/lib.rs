#![forbid(unsafe_code)]

//! Safe-Rust implementation layers for the frozen Core v0.1 contract.
//!
//! This crate implements syntax, canonical serialization, exact artifact and
//! semantic hashing, reference-availability validation, full Core v0.1
//! declaration checking, and deterministic structural foundation-audit
//! extraction. Complete JSON foundation-manifest packaging remains separate
//! Phase 1 work.

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
pub use format::{
    ARTIFACT_FORMAT, ModuleHashes, SEMANTIC_PROJECTION, Sha256Hex, compute_module_hashes,
    parse_canonical, parse_transport, print_canonical, print_semantic,
};
pub use syntax::{Arena, Declaration, Module, Natural, Term, TermId};

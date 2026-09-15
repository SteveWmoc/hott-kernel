#![forbid(unsafe_code)]

//! Safe-Rust implementation layers for the frozen Core v0.1 contract.
//!
//! This crate implements syntax, canonical serialization, exact artifact and
//! semantic hashing, reference-availability validation, full Core v0.1
//! declaration checking, deterministic structural foundation-audit extraction,
//! deterministic Foundation Manifest v0.1 generation, and strict manifest JSON
//! parsing/schema validation. Deterministic manifest comparison remains separate
//! Phase 1 work.

pub mod checker;
pub mod error;
pub mod format;
pub mod manifest;
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
pub use manifest::{
    FOUNDATION_MANIFEST_SCHEMA, FoundationManifest, KERNEL_THEORY, KERNEL_VERSION,
    ManifestBuildError, ParsedAuditDependencies, ParsedDeclarationAudit, ParsedFoundationManifest,
    ParsedManifestGenerator, ParsedProvenanceRecord, build_foundation_manifest,
    parse_foundation_manifest, print_foundation_manifest,
};
pub use syntax::{Arena, Declaration, Module, Natural, Term, TermId};

#![forbid(unsafe_code)]

//! Safe-Rust implementation layers for the frozen Core v0.1 contract.
//!
//! This crate implements syntax, serialization, and reference-availability
//! validation. It does not perform typing, conversion, normalization, hashing,
//! or foundation-manifest extraction.

pub mod checker;
pub mod error;
pub mod format;
pub mod syntax;

pub use checker::{CheckError, CheckErrorClass, ReferenceKind, check_references};
pub use error::{FormatError, FormatErrorClass};
pub use format::{parse_canonical, parse_transport, print_canonical, print_semantic};
pub use syntax::{Arena, Declaration, Module, Natural, Term, TermId};

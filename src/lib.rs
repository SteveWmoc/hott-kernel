#![forbid(unsafe_code)]

//! Format-layer implementation of the frozen Core v0.1 interchange.
//!
//! This crate deliberately stops at syntax and serialization. It does not
//! perform scope checking, typing, conversion, normalization, hashing, or
//! foundation-manifest extraction.

pub mod error;
pub mod format;
pub mod syntax;

pub use error::{FormatError, FormatErrorClass};
pub use format::{parse_canonical, parse_transport, print_canonical, print_semantic};
pub use syntax::{Arena, Declaration, Module, Natural, Term, TermId};

//! Logical checks over parsed Core v0.1 syntax.
//!
//! This module currently validates only local and global reference
//! availability. It does not perform typing, conversion, or normalization.

mod error;
mod references;

pub use error::{CheckError, CheckErrorClass, ReferenceKind};
pub use references::check_references;

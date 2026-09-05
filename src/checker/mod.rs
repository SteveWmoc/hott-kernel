//! Logical checks over parsed Core v0.1 syntax.
//!
//! The public checker layer currently validates only local and global
//! reference availability. Internal structural transformations and checked
//! context/environment state support the later typing, conversion, and
//! normalization layers.

mod error;
mod references;
// Land and audit frozen checker primitives before their public consumers so
// each trusted slice remains mechanically reviewable.
#[cfg_attr(not(test), allow(dead_code))]
mod state;
#[cfg_attr(not(test), allow(dead_code))]
mod transform;

pub use error::{CheckError, CheckErrorClass, ReferenceKind};
pub use references::check_references;

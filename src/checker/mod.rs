//! Logical checks over parsed Core v0.1 syntax.
//!
//! The public checker layer currently validates only local and global
//! reference availability. Internal structural transformations, checked
//! context/environment state, reduction, normalization, and conversion support
//! the later bidirectional typing layer.

mod error;
mod references;
// Land and audit frozen checker primitives before their public consumers so
// each trusted slice remains mechanically reviewable.
#[cfg_attr(not(test), allow(dead_code))]
mod convert;
#[cfg(test)]
mod convert_coverage;
#[cfg_attr(not(test), allow(dead_code))]
mod reduce;
#[cfg_attr(not(test), allow(dead_code))]
mod state;
#[cfg_attr(not(test), allow(dead_code))]
mod transform;

pub use error::{CheckError, CheckErrorClass, ReferenceKind};
pub use references::check_references;

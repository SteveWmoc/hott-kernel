//! Logical checks over parsed Core v0.1 syntax.
//!
//! The public checker layer currently validates only local and global
//! reference availability. Internal structural transformations, checked
//! context/environment state, reduction, normalization, conversion, motive
//! recognition, and bidirectional typing support the later complete checker.

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

pub use error::{CheckError, CheckErrorClass, ReferenceKind};
pub use references::check_references;

//! Boundary between untrusted discovery code and the trusted checker.
//!
//! Finder output must cross the ordinary public `hott-kernel` API. This module
//! must not acquire privileged access to checker internals.

pub use hott_kernel as kernel;

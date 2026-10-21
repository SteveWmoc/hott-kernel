//! Bounded term and conjecture generation.
//!
//! The first implementation will enumerate finder-side terms by AST depth and
//! construct canonical equation candidates. Search policy belongs here or in
//! later sibling modules, never in the trusted kernel.

# 0005: Safe Rust for the primary checker

- **Status:** Accepted
- **Date:** 2026-08-26

## Decision

The primary checker will be implemented in stable Rust. Unsafe Rust is
forbidden in the trusted kernel crate.

## Reason

Rust offers native performance, predictable deployment, strong memory safety,
and convenient testing and fuzzing without requiring a garbage-collected
runtime. A single binary is suitable for Linux, cloud CI, and Android/Termux.

## Consequences

- The trusted kernel crate will use `#![forbid(unsafe_code)]`.
- Parser and user-interface dependencies will remain outside that crate.
- Performance work will prioritize normalization and conversion algorithms
  before low-level micro-optimization.
- A later independent checker should use a different implementation and,
  preferably, a different language.

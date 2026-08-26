# 0004: Small trusted core and explicit core terms

- **Status:** Accepted
- **Date:** 2026-08-26

## Decision

Only a checker for fully explicit core terms determines logical acceptance.
Parsing, surface elaboration, tactics, external solvers, editor integration,
and proof generation are outside the trusted logical core.

The project will define a stable serialized core format suitable for
independent checking.

## Reason

A small trusted boundary makes failures easier to audit and permits multiple
independent front ends and checkers.

## Consequences

- Every surface feature must elaborate to an explicit core term.
- A tactic success message is not a checked theorem.
- Core serialization and environment hashing are part of the validation
  design.
- The audit engine must be reproducible from explicit terms and environments.

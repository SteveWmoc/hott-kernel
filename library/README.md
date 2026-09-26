# Checked HoTT library

This directory contains mathematical library source written in the untrusted
Surface language and accepted only through the frozen Core checker.

The library is not part of the trusted computing base. A library declaration
has no special status: it is an ordinary Surface declaration elaborated to
explicit Core, checked by the kernel, and visible to the existing foundation
audit.

## Initial discipline

- Library sources use `hott-surface/0.1` without extending the Surface grammar.
- Because Surface v0.1 has no imports, each source file is currently
  self-contained. Duplication across files is preferable to inventing an
  informal module system.
- Initial HoTT constructions are written explicitly at universe level 0. Later
  universe generalization must respect Core v0.1's explicit, noncumulative
  universe hierarchy rather than pretending universe polymorphism exists.
- Definitions and proofs may be transparent or opaque only by deliberate
  choice. Transparency is part of conversion behavior, not a synonym for
  "definition" versus "theorem".
- The constructive base library introduces no postulates or extensions.
  Anything added later as a postulate or extension must remain explicit and
  auditable.
- Every checked library source receives an integration regression that compiles
  it through Surface v0.1, checks the resulting Core module, and inspects its
  foundation audit.

These conventions are library policy only. They add no kernel rule, Surface
feature, or trusted component.

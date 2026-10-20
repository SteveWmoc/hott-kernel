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

## Quasi-inverse data

For a fixed map `f : A → B`, `quasi_inverse A B f` stores an inverse
`g : B → A` and homotopies `g ∘ f ~ id_A` and `f ∘ g ~ id_B`.
`quasi_inverse_intro` builds the data; `quasi_inverse_map`,
`quasi_inverse_section`, and `quasi_inverse_retraction` recover its fields.
`identity_quasi_inverse` supplies the identity example. All types are explicitly
at universe zero. This is chosen data, not a claim that quasi-inverse witnesses
form a proposition or a definition of `isEquiv`.

`transport_quasi_inverse A P x y p` packages the map from `P(x)` to `P(y)`
with inverse transport along `path_inverse A x y p`. Its section and retraction
are the existing `transport_inverse_after` and `transport_inverse_before`
proofs. The generic projections recover these fields by computation.

`quasi_inverse_symm A B f witness` supplies quasi-inverse data for the
chosen inverse map. Its inverse is `f` itself; its section is the original
retraction and its retraction is the original section. All three projection
identities hold by computation, for arbitrary witnesses.


`quasi_inverse_comp A B C f g witness_f witness_g` supplies chosen
quasi-inverse data for `g ∘ f`. Its inverse is `f_inverse ∘ g_inverse`.
The section applies `ap f_inverse` to the section of `g` at `f(x)`,
then concatenates with the section of `f` at `x`. The retraction applies
`ap g` to the retraction of `f` at `g_inverse(z)`, then concatenates
with the retraction of `g` at `z`. These are pointwise laws and require
no function extensionality. All three fields are tested through the public
projections for arbitrary witnesses, with a separate identity computation.

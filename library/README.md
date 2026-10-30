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

`quasi_inverse_homotopy A B f g homotopy_fg witness` transfers chosen
quasi-inverse data for `f` along the pointwise homotopy `f ~ g`. The inverse
map is unchanged. The new section at `x` concatenates `ap f_inverse` of the
reverse of `homotopy_fg(x)` with the old section at `x`. The new retraction
at `y` concatenates the reverse of `homotopy_fg(f_inverse(y))` with the
old retraction at `y`. This uses no function extensionality; generic projection
identities and identity-map computations are checked regressions.

## Homotopy naturality

For `f,g : A → B`, `H : f ~ g`, and `p : Id_A(x,y)`,
`homotopy_naturality A B f g H x y p` proves
`ap f p · H(y) = H(x) · ap g p`. The proof eliminates `p`; its
reflexivity branch is exactly `path_concat_left_refl B (f(x)) (g(x)) (H(x))`.
The branch is tested for arbitrary maps and homotopy witnesses, without
identifying arbitrary proofs. No function extensionality is needed.

## Identity and composition of function action

`ap_identity A x y p` proves `ap id p = p`.
For `f : A → B` and `g : B → C`, `ap_comp A B C f g x y p` proves
`ap (g ∘ f) p = ap g (ap f p)`. Both use path induction on `p`. Their
reflexivity branches are higher reflexivity, and generic computation tests
pin these precise proof terms for arbitrary types, maps, and base points.
Neither function extensionality nor a judgmental eta rule is used.

## Path cancellation

`path_concat_left_cancel A x y p z q s equality` cancels the common
prefix `p : Id_A(x,y)` from `p · q = p · s`, producing `q = s`.
`path_concat_right_cancel A x y z p s q equality` cancels the common
suffix `q : Id_A(y,z)` from `p · q = s · q`, producing `p = s`.
Both use path induction on the common path. At a reflexive suffix, right
cancellation returns the supplied equality proof unchanged. At a reflexive
prefix, left cancellation uses `(left_unit q)⁻¹ · (equality · left_unit s)`
in the path type. Tests pin these exact computations for arbitrary paths
and equality witnesses. Neither law requires uniqueness of identity proofs.

## Coherence of a homotopy to the identity

For `f : A → A` and `H : f ~ id`,
`homotopy_to_identity_coherence A f H x` proves `ap f (H(x)) = H(f(x))`.
Naturality at the path `H(x) : Id_A(f(x),x)` first gives
`ap f (H(x)) · H(x) = H(f(x)) · ap id (H(x))`.
Applying `ap_identity` under concatenation replaces the final path with `H(x)`,
and right cancellation gives the result. This is a derived higher path,
not an extra coherence axiom. For the identity map and reflexivity homotopy,
the construction computes to higher reflexivity for any type and base point.

## Half-adjoint inverse data

For a fixed map `f : A → B`, `half_adjoint_inverse A B f` stores an inverse
`g`, a section `eta : g ∘ f ~ id_A`, a retraction `epsilon : f ∘ g ~ id_B`,
and triangle coherence `ap f (eta(x)) = epsilon(f(x))` for every `x : A`.
`half_adjoint_inverse_intro` packages these four fields.
`half_adjoint_inverse_forget` returns the underlying chosen quasi-inverse
data, so the existing quasi-inverse projections recover its inverse and
homotopies. `half_adjoint_inverse_coherence` recovers the triangle witness.
`identity_half_adjoint_inverse` supplies the identity example.
These are explicit universe-zero data, not a new kernel rule or a claim that
ordinary quasi-inverse witnesses already satisfy the triangle law. Conversion
from arbitrary quasi-inverse data to half-adjoint data is not yet implemented.

## Retraction adjustment toward half-adjoint data

`quasi_inverse_adjoint_retraction A B f witness` constructs a new retraction
from chosen quasi-inverse data `(g, eta, epsilon)`. Its value at `b : B` is

`epsilon(f(g(b)))⁻¹ · (ap f (eta(g(b))) · epsilon(b))`.

The parentheses are part of the specified formula. The first path runs from
`f(g(b))` to `f(g(f(g(b))))`, the second returns to `f(g(b))`, and the last
ends at `b`. This is the retraction adjustment in the
[HoTT book, Theorem 4.2.3](https://hott.github.io/book/hott-online-76-ga40a78c.pdf).
`quasi_inverse_adjoint_adjust A B f witness` packages the original inverse map
and section with this new retraction as chosen quasi-inverse data.
Generic computation tests pin the complete formula and all three projections;
the identity quasi-inverse computes to itself. This slice constructs the
adjusted data only. Its triangle-coherence proof, and hence the conversion to
`half_adjoint_inverse`, remain to be implemented.

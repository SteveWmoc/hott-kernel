# Judgments Core v0.1 must reject

These are negative semantic and algorithmic requirements for the
[Core v0.1 calculus](../../docs/core-v0.1.md), not final diagnostic text.

## Universe inconsistency and silent cumulativity

Core v0.1 must reject

$$
\mathcal U_0:\mathcal U_0.
$$

It must also reject silent promotion of an arbitrary
$A:\mathcal U_i$ to $A:\mathcal U_{i+1}$. The maximum rule for dependent
products and sums does not add such a coercion.

If formation computes a mixed-level product or sum in
$\mathcal U_{\max(i,j)}$, the checker must reject an annotation that places it
in a different universe without a derivable conversion.

## False reflexivity

Reflexivity cannot inhabit an identity type whose endpoints are not
judgmentally equal. The core must reject

$$
\mathsf{refl}_{\mathbb N}(0)
:
\mathsf{Id}_{\mathbb N}(0,\mathsf{succ}(0)).
$$

## Ill-typed application and elimination

If $f:\Pi(x:A).B$ and $b:B'$ where $B'$ is not judgmentally equal to $A$,
the core must reject $f\,b$.

It must reject an eliminator when:

- the motive does not synthesize a function into a universe;
- a branch fails to check at the motive instance prescribed by the rule;
- the scrutinee has the wrong inductive type;
- the $J$ path endpoint does not match the explicit base point.

## Missing synthesis annotations

The bidirectional checker must reject a bare lambda or pair in synthesis
position. This is an algorithmic rejection, not a claim that the term is
declaratively untypable. An explicit `ann` may make it check.

For the same reason, a bare lambda used as an eliminator motive must be rejected
when no surrounding annotation exposes the motive's function type and universe
level. The core checker does not invent metavariables.

## No judgmental eta

Given $f:\Pi(x:A).B$, Core v0.1 must not conclude solely by conversion that

$$
f\equiv\lambda x.f\,x.
$$

Given $p:\Sigma(x:A).B$, it must likewise not conclude solely by conversion
that

$$
p\equiv(\mathsf{fst}(p),\mathsf{snd}(p)).
$$

Either eta rule would require a decision record and theory-version change.

## Definitional proof irrelevance

Given arbitrary

$$
p,q:\mathsf{Id}_A(x,y),
$$

the core must not treat $p$ and $q$ as judgmentally equal. In particular,
$\mathsf{refl}(p)$ must not check at $\mathsf{Id}(p,q)$ unless $p$ and $q$ are
independently judgmentally equal. Thus the naive term

$$
\lambda A\,x\,y\,p\,q.\mathsf{refl}(p)
$$

must not check as a proof of UIP.

## Equality reflection

From a term $p:\mathsf{Id}_A(a,b)$, the checker must not conclude
$a\equiv b:A$ during conversion.

## Opaque unfolding

Suppose an opaque definition has checked body
$c:\mathbb N:=\mathsf{succ}(0)$. The declaration itself is valid, but
conversion must reject

$$
c\equiv\mathsf{succ}(0):\mathbb N.
$$

Changing `opaque` to `transparent` changes this result and therefore must be
audited.

## Unannounced extensionality and axioms

Core v0.1 must not accept function extensionality, propositional
extensionality, univalence, excluded middle, choice, UIP, or axiom K as
primitive facts unless the exact principle is introduced as an explicit
postulate or versioned extension and reported accordingly.

## Invalid environments and malformed interchange

The checker must report `invalid-judgment` for:

- a global reference to the declaration currently being checked or to a later
  declaration;
- a local de Bruijn index outside the current context;
- a declaration body that does not check against its declared type.

The format layer must report `malformed-encoding` for:

- a postulate carrying a body;
- a term tag with the wrong arity;
- a metavariable, implicit argument, tactic, or unresolved name in core input.

The complete result vocabulary and exact examples are in
[`failure-classes.md`](../../docs/failure-classes.md) and
[`tests/format/`](../format/).

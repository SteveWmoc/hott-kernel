# Judgments Core v0.1 must accept

These are normative semantic examples for the
[Core v0.1 calculus](../../docs/core-v0.1.md). The S-expressions are candidate
golden inputs for the parser and checker; until implementation begins, the
displayed judgments are the source of truth.

## Contexts and universes

- The empty context is valid.
- If $\Gamma$ is valid and $\Gamma\vdash A:\mathcal U_i$, then
  $\Gamma,x:A$ is valid.
- $\mathcal U_i:\mathcal U_{i+1}$ for every concrete natural-number level
  $i$.
- If $A:\mathcal U_i$ and $x:A\vdash B:\mathcal U_j$, then both
  $\Pi(x:A).B$ and $\Sigma(x:A).B$ inhabit
  $\mathcal U_{\max(i,j)}$, including when $i\ne j$.

For example, from $A:\mathcal U_0$ and
$x:A\vdash\mathcal U_0:\mathcal U_1$, the checker derives

$$
\Pi(x:A).\mathcal U_0:\mathcal U_1.
$$

It does not use cumulativity to obtain this result; it computes the maximum of
the two formation levels.

## Dependent functions

For each fixed universe level $i$, polymorphic identity is derivable:

$$
\lambda (A:\mathcal U_i).\lambda (x:A).x
:
\Pi(A:\mathcal U_i).\Pi(x:A).A.
$$

The level-zero declaration has this readable transport form:

```text
(hott-core
  (format 0 1)
  (theory "mltt-core" 0 1)
  (declarations
    (transparent "id-U0"
      (pi (universe 0)
          (pi (var 0) (var 1)))
      (lam (lam (var 0))))))
```

Its exact canonical bytes and expected hashes are fixed by the
[`identity-u0` format fixture](../format/canonical/identity-u0.core).

Beta computation must establish

$$
(\lambda (x:A).t)\,a\equiv t[a/x]:B[a/x].
$$

An introduction form can be placed in synthesis position with an annotation:

```text
(ann (lam (var 0)) (pi unit unit))
```

## Dependent pairs

If $a:A$ and $b:B[a/x]$, then

$$
(a,b):\Sigma(x:A).B,
$$

with judgmental computations

$$
\mathsf{fst}(a,b)\equiv a:A
$$

and

$$
\mathsf{snd}(a,b)\equiv b:B[a/x].
$$

## Identity

Polymorphic reflexivity is derivable:

$$
\lambda (A:\mathcal U_i).\lambda (x:A).\mathsf{refl}_A(x)
:
\Pi(A:\mathcal U_i).\Pi(x:A).\mathsf{Id}_A(x,x).
$$

The following constructions must be definable using $J$ rather than new
kernel rules:

- path inversion;
- path concatenation;
- transport in a type family;
- application of a function to a path;
- dependent application to a path.

The $J$ eliminator computes judgmentally at reflexivity:

$$
J(A,a,C,d,a,\mathsf{refl}_A(a))\equiv d:C\,a\,\mathsf{refl}_A(a).
$$

## Empty and unit

- `empty` synthesizes $\mathcal U_0$.
- Given $e:\mathbf 0$ and an explicitly typed motive
  $C:\mathbf 0\to\mathcal U_i$, `empty-elim C e` synthesizes $C\,e$.
- `star` synthesizes $\mathbf 1$.
- Unit elimination computes at `star`:
  $\mathsf{unitElim}(C,c,\star)\equiv c$.

## Natural numbers

- $0:\mathbb N$.
- If $n:\mathbb N$, then $\mathsf{succ}(n):\mathbb N$.
- Natural-number elimination computes at zero and successors.
- Closed addition examples normalize to the expected numeral once addition is
  defined from `nat-elim`.

In particular,

$$
\mathsf{natElim}(C,z,s,0)\equiv z
$$

and

$$
\mathsf{natElim}(C,z,s,\mathsf{succ}(n))
\equiv s\,n\,(\mathsf{natElim}(C,z,s,n)).
$$

## Globals and transparency

- A well-typed postulate declaration is accepted and reported as a postulate.
- A transparent definition with a checked body is accepted, and its global
  reference unfolds during conversion.
- An opaque definition with a checked body is accepted, but its global
  reference remains neutral during conversion.
- A lambda motive for an eliminator is accepted in synthesis position when an
  annotation exposes its complete dependent function type and universe level.

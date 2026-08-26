# Judgments Core v0.1 must accept

These examples are semantic requirements, not final surface syntax. Each will
eventually be represented as a fully explicit core term and an executable
golden test.

## Contexts and universes

- The empty context is valid.
- Extending a valid context by a well-formed type yields a valid context.
- $\mathcal U_0 : \mathcal U_1$.
- More generally, $\mathcal U_i : \mathcal U_{i+1}$.

## Dependent functions

Polymorphic identity:

$$
\lambda (A:\mathcal U_i).\,\lambda (x:A).\,x
:
\prod_{A:\mathcal U_i} A\to A.
$$

Beta computation:

$$
(\lambda (x:A).\,x)\,a \equiv a : A.
$$

## Identity

Polymorphic reflexivity:

$$
\lambda (A:\mathcal U_i).\,\lambda (x:A).\,\mathsf{refl}_x
:
\prod_{A:\mathcal U_i}\prod_{x:A}\mathsf{Id}_A(x,x).
$$

The following constructions must be definable using $J$:

- path inversion;
- path concatenation;
- transport in a type family;
- application of a function to a path;
- dependent application to a path.

The $J$ eliminator must compute judgmentally at reflexivity.

## Dependent pairs

For $a:A$ and $b:B(a)$:

$$
\mathsf{fst}(a,b)\equiv a
$$

and

$$
\mathsf{snd}(a,b)\equiv b.
$$

## Natural numbers

- $0:\mathbb N$.
- If $n:\mathbb N$, then $\mathsf{succ}(n):\mathbb N$.
- Primitive recursion computes on zero and successors.
- Closed addition examples normalize to the expected numeral.

# Judgments Core v0.1 must reject

These are negative semantic requirements, not final diagnostic text.

## Universe inconsistency

Core v0.1 must reject:

$$
\mathcal U_0 : \mathcal U_0.
$$

It must also reject silent coercion of an arbitrary $A:\mathcal U_i$ to
$A:\mathcal U_{i+1}$, because the initial universes are noncumulative.

## False reflexivity

The reflexivity constructor cannot inhabit an identity type whose endpoints
are not judgmentally equal. For example, the core must reject:

$$
\mathsf{refl}_0 : \mathsf{Id}_{\mathbb N}(0,\mathsf{succ}(0)).
$$

## Ill-typed application

If $f:\prod_{x:A}B(x)$ and $b:B'$ where $B'$ is not judgmentally equal
to $A$, the core must reject $f\,b$.

## Definitional proof irrelevance

Given arbitrary

$$
p,q:\mathsf{Id}_A(x,y),
$$

the core must not treat $p$ and $q$ as judgmentally equal. In particular,
the purported body $\mathsf{refl}_p$ must not check at
$\mathsf{Id}(p,q)$ unless $p$ and $q$ are independently judgmentally
equal.

Thus the naive term

$$
\lambda A\,x\,y\,p\,q.\,\mathsf{refl}_p
$$

must not check as a proof of UIP.

## Equality reflection

From a term $p:\mathsf{Id}_A(a,b)$, the checker must not conclude
$a\equiv b:A$ during conversion.

## Unannounced extensionality

Core v0.1 must not accept function extensionality, propositional
extensionality, univalence, excluded middle, or choice as primitive facts
unless a corresponding postulate or extension is explicitly declared.

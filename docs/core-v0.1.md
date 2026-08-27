# Core v0.1 calculus

**Status:** Normative Phase 0 candidate.

This document specifies the declarative theory that the first `hott-kernel`
checker will implement. It also specifies a bidirectional checking algorithm
that is intended to decide the declarative judgments. The declarative rules
define what is true; the algorithm is an implementation whose soundness and
completeness are metatheoretic obligations.

Nothing in this document introduces proof irrelevance, equality reflection,
function extensionality, univalence, choice, excluded middle, or unrestricted
recursion.

## 1. Syntactic classes

Universe levels are natural numbers $i,j,k\in\mathbb N$. Local variables are
represented in serialized core terms by de Bruijn indices, with index `0`
referring to the newest binder. The inference rules use names for readability.

The raw term grammar is:

```text
t, A, B ::=
    var n                         local variable
  | global n                      earlier global declaration
  | universe i                    universe U_i
  | pi A B                        dependent function type; B binds one variable
  | lam t                         lambda; t binds one variable
  | app t u                       application
  | sigma A B                     dependent pair type; B binds one variable
  | pair t u                      pair
  | fst t                         first projection
  | snd t                         second projection
  | id A t u                      identity type
  | refl t                        reflexivity
  | j A a C d b p                 identity eliminator
  | empty                         empty type
  | empty-elim C e                empty elimination
  | unit                          unit type
  | star                          canonical unit element
  | unit-elim C c u               unit elimination
  | nat                           natural-number type
  | zero                          zero
  | succ t                        successor
  | nat-elim C z s n              natural-number elimination
  | ann t A                       explicit type annotation
```

Only `pi`, `sigma`, and `lam` bind local variables. Motives and branches for
eliminators are ordinary functions represented with these binders. This keeps
binding uniform throughout the core language.

The constructor `ann t A` is algorithmic evidence directing synthesis. It
adds no mathematical principle and is judgmentally equal to `t` after its type
has been checked.

## 2. Environments and contexts

A global environment $\mathcal E$ is a finite ordered sequence of closed
declarations. A declaration is one of:

- a postulate with a type and no body;
- a transparent definition with a type and checked body;
- an opaque definition with a type and checked body.

Bodies may refer only to earlier declarations. Core v0.1 has no recursive or
mutually recursive global definitions. A serialized `global n` is an absolute
zero-based index into this sequence, with zero denoting the first declaration.

A local context is a finite telescope


$$
\Gamma \equiv x_1:A_1,\ldots,x_n:A_n,
$$

where each (A_r) may depend on the preceding variables. When the environment
is fixed, it is omitted from judgments.

The declarative judgments are:

$$
\mathcal E\;\mathsf{env}
$$

$$
\mathcal E;\Gamma\;\mathsf{ctx}
$$

$$
\mathcal E;\Gamma \vdash A\;\mathsf{type}
$$

$$
\mathcal E;\Gamma \vdash t:A
$$

$$
\mathcal E;\Gamma \vdash t\equiv u:A.
$$

The type-formation judgment is shorthand for inhabiting some universe:

$$
\Gamma\vdash A\;\mathsf{type}
\quad\Longleftrightarrow\quad
\Gamma\vdash A:\mathcal U_i
\text{ for some }i.
$$

Because the universe hierarchy is noncumulative, the level is not silently
changed by this abbreviation.

## 3. Substitution

A simultaneous substitution $\Delta\vdash\sigma:\Gamma$ assigns to every
variable of $\Gamma$ a term in $\Delta$ of the corresponding substituted
type. The action of a substitution on a term is written $t[\sigma]$.

For de Bruijn syntax, a substitution is a map from indices to terms. Its action
is defined structurally. Under a binder, the lifted substitution
$\sigma^{\uparrow}$ is given by

$$
\sigma^{\uparrow}(0)=\mathsf{var}(0),
$$

$$
\sigma^{\uparrow}(n+1)=\mathsf{shift}(\sigma(n)),
$$

where `shift` increments free indices to account for the new binder.
Substitution through `pi`, `sigma`, and `lam` uses
$\sigma^{\uparrow}$ in the binding argument and $\sigma$ elsewhere.

Identity substitution and substitution composition are defined pointwise.
Capture avoidance follows from the de Bruijn representation.

The substitution and weakening properties stated later are theorems about
these rules. They are not additional kernel inference rules.

## 4. Structural rules

The empty context is valid:

$$
\frac{\mathcal E\;\mathsf{env}}
     {\mathcal E;\cdot\;\mathsf{ctx}}.
$$

A context may be extended by a type:

$$
\frac{\Gamma\;\mathsf{ctx}
      \qquad
      \Gamma\vdash A:\mathcal U_i}
     {\Gamma,x:A\;\mathsf{ctx}}.
$$

A variable has the type recorded by context lookup, with the weakening shifts
required by its de Bruijn depth:

$$
\frac{x:A\in\Gamma}
     {\Gamma\vdash x:A}.
$$

An earlier global declaration may be referenced at its declared type:

$$
\frac{\mathcal E(n)=g:A}
     {\mathcal E;\Gamma\vdash \mathsf{global}(n):A}.
$$

The conversion rule is:

$$
\frac{\Gamma\vdash t:A
      \qquad
      \Gamma\vdash A\equiv B:\mathcal U_i}
     {\Gamma\vdash t:B}.
$$

The level $i$ in this rule is determined, not chosen. Because the universes
are noncumulative and distinct universe constants have distinct normal forms,
every well-formed type has a unique normalized universe level. Thus the
equality premise is formed at the unique level at which both $A$ and $B$
inhabit a universe; the rule performs no implicit lifting to a common higher
level. Uniqueness of declarative typing is a metatheoretic obligation.

## 5. Universes

The universes are predicative and noncumulative:

$$
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash\mathcal U_i:\mathcal U_{i+1}}.
$$

There is no rule deriving $A:\mathcal U_{i+1}$ from
$A:\mathcal U_i$. In particular, Core v0.1 has neither type-in-type nor an
impredicative proposition universe.

## 6. Dependent function types

Formation uses the maximum of the domain and codomain levels:

$$
\frac{\Gamma\vdash A:\mathcal U_i
      \qquad
      \Gamma,x:A\vdash B:\mathcal U_j}
     {\Gamma\vdash\Pi(x:A).B:\mathcal U_{\max(i,j)}}.
$$

Introduction:

$$
\frac{\Gamma,x:A\vdash t:B}
     {\Gamma\vdash\lambda x.t:\Pi(x:A).B}.
$$

Elimination:

$$
\frac{\Gamma\vdash f:\Pi(x:A).B
      \qquad
      \Gamma\vdash a:A}
     {\Gamma\vdash f\,a:B[a/x]}.
$$

Computation:

$$
(\lambda x.t)\,a\;\longrightarrow\;t[a/x].
$$

There is no judgmental function-eta rule in Core v0.1.

## 7. Dependent pair types

Formation also uses the maximum rule:

$$
\frac{\Gamma\vdash A:\mathcal U_i
      \qquad
      \Gamma,x:A\vdash B:\mathcal U_j}
     {\Gamma\vdash\Sigma(x:A).B:\mathcal U_{\max(i,j)}}.
$$

Introduction:

$$
\frac{\Gamma\vdash a:A
      \qquad
      \Gamma\vdash b:B[a/x]}
     {\Gamma\vdash(a,b):\Sigma(x:A).B}.
$$

Projections:

$$
\frac{\Gamma\vdash p:\Sigma(x:A).B}
     {\Gamma\vdash\mathsf{fst}(p):A},
$$

$$
\frac{\Gamma\vdash p:\Sigma(x:A).B}
     {\Gamma\vdash\mathsf{snd}(p):B[\mathsf{fst}(p)/x]}.
$$

Computation:

$$
\mathsf{fst}(a,b)\longrightarrow a,
\qquad
\mathsf{snd}(a,b)\longrightarrow b.
$$

There is no judgmental pair-eta rule in Core v0.1.

## 8. Identity types

Formation:

$$
\frac{\Gamma\vdash A:\mathcal U_i
      \qquad
      \Gamma\vdash a:A
      \qquad
      \Gamma\vdash b:A}
     {\Gamma\vdash\mathsf{Id}_A(a,b):\mathcal U_i}.
$$

Reflexivity:

$$
\frac{\Gamma\vdash a:A}
     {\Gamma\vdash\mathsf{refl}_A(a):\mathsf{Id}_A(a,a)}.
$$

The identity eliminator is based at (a). Its motive is an ordinary dependent
function:

$$
\frac{
  \Gamma\vdash A:\mathcal U_i
  \quad
  \Gamma\vdash a:A
  \quad
  \Gamma\vdash C:
    \Pi(y:A).\Pi(p:\mathsf{Id}_A(a,y)).\mathcal U_j
  \quad
  \Gamma\vdash d:C\,a\,(\mathsf{refl}_A(a))
  \quad
  \Gamma\vdash b:A
  \quad
  \Gamma\vdash q:\mathsf{Id}_A(a,b)
}
{
  \Gamma\vdash
  J(A,a,C,d,b,q):C\,b\,q
}.
$$

In serialized core syntax, the motive `C` occurs in a synthesis position. If
the surface motive is a lambda, the surface elaborator is responsible for
inserting an `ann` that gives its full dependent function type, including the
target universe $\mathcal U_j$. The core checker never invents $j$ or any
other universe level. [Section 15.2](#152-synthesis) specifies this algorithmic
requirement; it does not alter the declarative rule above.

Computation at reflexivity:

$$
J(A,a,C,d,a,\mathsf{refl}_A(a))\longrightarrow d.
$$

Identity types live in the same data-relevant universe as their underlying
type. No rule collapses their inhabitants. Core v0.1 contains neither UIP,
axiom K, equality reflection, nor any extensionality principle.

## 9. Empty and unit types

The empty type is small:

$$
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash\mathbf 0:\mathcal U_0}.
$$

Its eliminator permits elimination into any explicit universe:

$$
\frac{\Gamma\vdash C:\mathbf 0\to\mathcal U_i
      \qquad
      \Gamma\vdash e:\mathbf 0}
     {\Gamma\vdash\mathsf{emptyElim}(C,e):C\,e}.
$$

There is no computation rule because the empty type has no constructor.

The unit type is small and has one constructor:

$$
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash\mathbf 1:\mathcal U_0},
\qquad
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash\star:\mathbf 1}.
$$

Its eliminator is:

$$
\frac{\Gamma\vdash C:\mathbf 1\to\mathcal U_i
      \qquad
      \Gamma\vdash c:C\,\star
      \qquad
      \Gamma\vdash u:\mathbf 1}
     {\Gamma\vdash\mathsf{unitElim}(C,c,u):C\,u}.
$$

Computation:

$$
\mathsf{unitElim}(C,c,\star)\longrightarrow c.
$$

This rule does not assert judgmental uniqueness of arbitrary inhabitants of
the unit type.

## 10. Natural numbers

Formation and constructors:

$$
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash\mathbb N:\mathcal U_0},
\qquad
\frac{\Gamma\;\mathsf{ctx}}
     {\Gamma\vdash 0:\mathbb N},
$$

$$
\frac{\Gamma\vdash n:\mathbb N}
     {\Gamma\vdash\mathsf{succ}(n):\mathbb N}.
$$

Dependent elimination:

$$
\frac{
  \Gamma\vdash C:\mathbb N\to\mathcal U_i
  \quad
  \Gamma\vdash z:C\,0
  \quad
  \Gamma\vdash s:\Pi(n:\mathbb N).C\,n\to C\,(\mathsf{succ}(n))
  \quad
  \Gamma\vdash n:\mathbb N
}
{
  \Gamma\vdash\mathsf{natElim}(C,z,s,n):C\,n
}.
$$

Computation:

$$
\mathsf{natElim}(C,z,s,0)\longrightarrow z,
$$

$$
\mathsf{natElim}(C,z,s,\mathsf{succ}(n))
\longrightarrow
s\,n\,(\mathsf{natElim}(C,z,s,n)).
$$

## 11. Type annotations

Annotations are checked terms:

$$
\frac{\Gamma\vdash A:\mathcal U_i
      \qquad
      \Gamma\vdash t:A}
     {\Gamma\vdash\mathsf{ann}(t,A):A}.
$$

They erase by computation:

$$
\mathsf{ann}(t,A)\longrightarrow t.
$$

## 12. Global declarations

All global declaration types and bodies are closed with respect to local
variables.

A postulate extends a valid environment when its type is a type:

$$
\frac{\mathcal E\;\mathsf{env}
      \qquad
      \mathcal E;\cdot\vdash A:\mathcal U_i}
     {\mathcal E,(\mathsf{postulate}\;g:A)\;\mathsf{env}}.
$$

A transparent or opaque definition additionally requires a checked body:

$$
\frac{\mathcal E\;\mathsf{env}
      \qquad
      \mathcal E;\cdot\vdash A:\mathcal U_i
      \qquad
      \mathcal E;\cdot\vdash t:A}
     {\mathcal E,(\kappa\;g:A:=t)\;\mathsf{env}},
$$

where $\kappa\in\{\mathsf{transparent},\mathsf{opaque}\}$.

Transparent globals unfold during judgmental equality. Opaque globals and
postulates remain neutral. The body of an opaque definition is nevertheless
checked and retained for auditing. There is no special proof or theorem class.

## 13. Reduction

One-step reduction $t\longrightarrow u$ is the compatible closure of:

- function beta reduction;
- pair projection reduction;
- (J) at reflexivity;
- unit elimination at `star`;
- natural-number elimination at `zero` and `succ`;
- annotation erasure;
- unfolding a transparent global declaration.

Reduction never unfolds an opaque definition or postulate. There are no eta,
proof-erasure, equality-reflection, quotient, or univalence reductions.

The reflexive transitive closure is written $\longrightarrow^{*}$.

## 14. Judgmental equality

Judgmental equality is the smallest typed equivalence relation that:

1. contains one-step reduction in both directions;
2. is compatible with every well-typed raw-term constructor;
3. is stable under well-typed substitution;
4. respects conversion of the ambient type.

Equivalently, for Core v0.1, two well-typed terms are judgmentally equal when
their beta-delta-iota normal forms are structurally identical up to binder
renaming. Delta reduction applies only to transparent globals. De Bruijn core
terms make alpha-equivalence syntactic.

The basic equality rules are:

$$
\frac{\Gamma\vdash t:A}
     {\Gamma\vdash t\equiv t:A},
$$

$$
\frac{\Gamma\vdash t\equiv u:A}
     {\Gamma\vdash u\equiv t:A},
\qquad
\frac{\Gamma\vdash t\equiv u:A
      \qquad
      \Gamma\vdash u\equiv v:A}
     {\Gamma\vdash t\equiv v:A}.
$$

The compatible-closure clause means that replacing a subterm by a
judgmentally equal subterm in any well-typed one-hole term context preserves
judgmental equality, after the conversions required by dependency.

Judgmental equality is a metalinguistic judgment used by the checker. It is not
the identity type and cannot be assumed or pattern-matched upon inside the
object theory.

## 15. Bidirectional algorithm

The checker uses three algorithmic judgments:

$$
\Gamma\vdash t\Rightarrow A
\quad\text{(synthesis)},
$$

$$
\Gamma\vdash t\Leftarrow A
\quad\text{(checking)},
$$

$$
\Gamma\vdash A\approx B
\quad\text{(algorithmic conversion)}.
$$

The invariant is that expected types supplied to checking have already been
validated as types.

### 15.1 Auxiliary operations

`inferUniverse(A)` synthesizes the type of `A`, normalizes that type to weak
head normal form, and succeeds with $i$ only when the result is
$\mathcal U_i$.

`whnf(t)` performs beta-delta-iota reduction until the head constructor is
visible, unfolding transparent globals only.

`convert(A,B)` normalizes the already well-typed inputs without eta expansion
and compares their normal forms structurally. The intended implementation is
normalization by evaluation. A simpler normalizer may be used initially if it
decides the same relation.

### 15.2 Synthesis

The following forms synthesize:

- `var n` and `global n` return their lookup types;
- `universe i` returns `universe (i+1)`;
- `pi A B` and `sigma A B` call `inferUniverse` on the domain and on the
  codomain in the extended context, returning `universe max(i,j)`;
- `app f a` synthesizes `f`, exposes a `pi A B`, checks `a` against `A`, and
  returns (B[a/x]);
- `fst p` exposes a synthesized sigma type and returns its domain;
- `snd p` exposes a synthesized sigma type and returns its codomain with
  `fst p` substituted;
- `id A a b` checks `A` as a universe and both endpoints against `A`, returning
  the same universe as `A`;
- `refl a` synthesizes $A$ for `a` and returns
  $\mathsf{Id}_A(a,a)$;
- `empty`, `unit`, and `nat` return $\mathcal U_0$;
- `star` returns `unit`, `zero` returns `nat`, and `succ n` checks `n` against
  `nat` and returns `nat`;
- `ann t A` calls `inferUniverse(A)`, checks `t` against `A`, and returns `A`.

The eliminators synthesize as follows:

- `j A a C d b p` calls `inferUniverse(A)`, checks `a` and `b` against `A`,
  and requires the synthesized type of `C` to be convertible to
  $\Pi(y:A).\Pi(q:\mathsf{Id}_A(a,y)).\mathcal U_j$ for some exposed
  universe level $j$. It checks `d` against
  $C\,a\,(\mathsf{refl}_A(a))$ and `p` against
  $\mathsf{Id}_A(a,b)$, then returns $C\,b\,p$.
- `empty-elim C e` requires the synthesized type of `C` to be convertible to
  $\Pi(x:\mathbf 0).\mathcal U_j$, checks `e` against $\mathbf 0$, and
  returns $C\,e$.
- `unit-elim C c u` requires the synthesized type of `C` to be convertible to
  $\Pi(x:\mathbf 1).\mathcal U_j$, checks `c` against $C\,\star$ and `u`
  against $\mathbf 1$, and returns $C\,u$.
- `nat-elim C z s n` requires the synthesized type of `C` to be convertible
  to $\Pi(k:\mathbb N).\mathcal U_j$, checks `z` against $C\,0$, checks `s`
  against
  $\Pi(k:\mathbb N).\Pi(h:C\,k).C\,(\mathsf{succ}(k))$, checks `n` against
  $\mathbb N$, and returns $C\,n$.

The checker obtains $j$ by exposing the synthesized motive type; it never
solves for a hidden universe metavariable. Consequently a lambda motive must
carry an `ann` whose type exhibits the relevant universe level. The surface
elaborator is responsible for inserting such annotations when translating
surface motives; the core checker will not guess them. Expected branch types
are instantiated internally from the already checked motive, so this process
adds no new typing rule.

### 15.3 Checking

A lambda checks against a function type:

$$
\frac{\mathsf{whnf}(T)=\Pi(x:A).B
      \qquad
      \Gamma,x:A\vdash t\Leftarrow B}
     {\Gamma\vdash\mathsf{lam}(t)\Leftarrow T}.
$$

A pair checks against a dependent pair type:

$$
\frac{\mathsf{whnf}(T)=\Sigma(x:A).B
      \qquad
      \Gamma\vdash a\Leftarrow A
      \qquad
      \Gamma\vdash b\Leftarrow B[a/x]}
     {\Gamma\vdash\mathsf{pair}(a,b)\Leftarrow T}.
$$

All other checking falls back to synthesis and conversion:

$$
\frac{\Gamma\vdash t\Rightarrow A
      \qquad
      \Gamma\vdash A\approx B}
     {\Gamma\vdash t\Leftarrow B}.
$$

Lambdas and pairs do not synthesize without an annotation. This is a property
of the algorithm, not a restriction on the declarative theory: `ann` can place
any declaratively typable introduction form into a synthesis position.

### 15.4 Algorithmic conversion

Algorithmic conversion:

1. evaluates both inputs to beta-delta-iota normal form;
2. compares identical constructors recursively;
3. compares binder bodies in contexts extended by the same fresh neutral;
4. compares neutral heads and spines structurally;
5. treats opaque globals and postulates as neutral constants;
6. performs no eta expansion and never identifies arbitrary identity proofs.

The checker reports failure when normal forms differ. The required
metatheorems are:

- conversion soundness: $A\approx B$ implies $A\equiv B$;
- conversion completeness on well-typed terms: $A\equiv B$ implies
  $A\approx B$;
- checking soundness;
- annotation completeness: every declaratively typable term can be annotated
  so that the bidirectional checker accepts it.

## 16. Representative derivations

### 16.1 Polymorphic identity

For fixed (i), the type

$$
\Pi(A:\mathcal U_i).\Pi(x:A).A
$$

lives in $\mathcal U_{i+1}$. In the context
$A:\mathcal U_i,x:A$, the variable rule gives $x:A$. Applying function
introduction twice derives

$$
\lambda A.\lambda x.x:
\Pi(A:\mathcal U_i).\Pi(x:A).A.
$$

The serialized body is `lam (lam (var 0))`; binder types come from the declared
expected type.

### 16.2 Polymorphic reflexivity

In the same context, reflexivity gives

$$
\mathsf{refl}_A(x):\mathsf{Id}_A(x,x).
$$

Two function introductions derive

$$
\lambda A.\lambda x.\mathsf{refl}_A(x):
\Pi(A:\mathcal U_i).\Pi(x:A).\mathsf{Id}_A(x,x).
$$

### 16.3 Why the naive UIP term fails

In a context containing arbitrary

$$
p,q:\mathsf{Id}_A(x,y),
$$

`refl p` synthesizes $\mathsf{Id}(p,p)$. It checks against
$\mathsf{Id}(p,q)$ only if conversion proves $p\equiv q$. No Core v0.1
rule supplies that conversion. Therefore

$$
\lambda A\,x\,y\,p\,q.\mathsf{refl}(p)
$$

does not check as a proof of UIP.

## 17. Metatheoretic obligations

The following are intended theorems, not kernel rules:

- weakening;
- substitution;
- substitution composition;
- subject reduction;
- uniqueness of declarative typing up to judgmental equality;
- uniqueness of synthesized types up to judgmental equality;
- strong normalization of well-typed Core v0.1 terms;
- decidability of checking and conversion;
- canonicity for natural-number terms closed in an environment without
  postulates or opaque definitions that can produce a natural number;
- soundness and annotation completeness of the bidirectional algorithm;
- consistency relative to a documented model;
- existence of a homotopically nontrivial model, showing that the rules do not
  force UIP.

The intended proof and implementation strategies are recorded in
[`metatheory.md`](metatheory.md).

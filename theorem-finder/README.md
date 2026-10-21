# hott-theorem-finder

Experimental theorem-discovery machinery for `hott-kernel`.

This crate is **not trusted**. It may eventually contain exhaustive generation,
heuristics, randomized search, MCTS, reinforcement learning, neural models, and
other speculative machinery. None of those mechanisms can certify a theorem.

The architectural boundary is:

```text
finder -> candidate / proof object -> public hott-kernel checker -> accepted or rejected
```

The dependency is intentionally one-way: `hott-theorem-finder` depends on
`hott-kernel`; the kernel must never depend on the finder.

The first experiment uses a tiny one-sorted algebraic language with constants
`0` and `1`, binary operations `+` and `*`, and equality. See
[SPEC.md](SPEC.md).

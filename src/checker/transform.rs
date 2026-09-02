use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Natural, Term, TermId};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TransformError;

/// Apply `shift_{increment, cutoff}` by appending only changed nodes.
///
/// `root` must belong to `arena`. On error, nodes appended by this operation
/// are removed before returning.
pub(super) fn shift(
    arena: &mut Arena,
    root: TermId,
    increment: usize,
    cutoff: usize,
) -> Result<TermId, TransformError> {
    assert!(
        arena.get(root).is_some(),
        "root term id must belong to arena"
    );
    if increment == 0 {
        return Ok(root);
    }

    let checkpoint = arena.len();
    let mut operation = Operation::Shift { increment };
    let result = rewrite(arena, root, cutoff, &mut operation);
    if result.is_err() {
        arena.truncate(checkpoint);
    }
    result
}

/// Substitute `replacement` for the newest local variable in `body`.
///
/// Both term IDs must belong to `arena`. On error, nodes appended by this
/// operation are removed before returning.
pub(super) fn substitute_top(
    arena: &mut Arena,
    body: TermId,
    replacement: TermId,
) -> Result<TermId, TransformError> {
    assert!(
        arena.get(body).is_some(),
        "body term id must belong to arena"
    );
    assert!(
        arena.get(replacement).is_some(),
        "replacement term id must belong to arena"
    );

    let checkpoint = arena.len();
    let mut operation = Operation::SubstituteTop {
        replacement,
        shifted_replacements: Vec::new(),
    };
    let result = rewrite(arena, body, 0, &mut operation);
    if result.is_err() {
        arena.truncate(checkpoint);
    }
    result
}

enum Operation {
    Shift {
        increment: usize,
    },
    SubstituteTop {
        replacement: TermId,
        shifted_replacements: Vec<Option<TermId>>,
    },
}

impl Operation {
    fn plan_variable(&self, index: &Natural, scope: usize) -> Result<VariablePlan, TransformError> {
        match self {
            Self::Shift { increment } => {
                if index.to_usize().is_some_and(|index| index < scope) {
                    Ok(VariablePlan::Keep)
                } else {
                    index
                        .try_add_usize(*increment)
                        .map(VariablePlan::Rebuild)
                        .map_err(from_format_error)
                }
            }
            Self::SubstituteTop { .. } => match index.to_usize() {
                Some(index) if index < scope => Ok(VariablePlan::Keep),
                Some(index) if index == scope => Ok(VariablePlan::Replacement),
                _ => index
                    .try_predecessor()
                    .map_err(from_format_error)
                    .map(|index| {
                        VariablePlan::Rebuild(
                            index.expect("an index above the binder depth is positive"),
                        )
                    }),
            },
        }
    }

    fn replacement_at_depth(
        &mut self,
        arena: &mut Arena,
        depth: usize,
    ) -> Result<TermId, TransformError> {
        let Self::SubstituteTop {
            replacement,
            shifted_replacements,
        } = self
        else {
            unreachable!("only substitution requests a replacement")
        };

        if depth == 0 {
            return Ok(*replacement);
        }

        let required = depth.checked_add(1).ok_or(TransformError)?;
        if shifted_replacements.len() < required {
            shifted_replacements
                .try_reserve(required - shifted_replacements.len())
                .map_err(|_| TransformError)?;
            shifted_replacements.resize(required, None);
        }
        if let Some(shifted) = shifted_replacements[depth] {
            return Ok(shifted);
        }

        let shifted = shift(arena, *replacement, depth, 0)?;
        shifted_replacements[depth] = Some(shifted);
        Ok(shifted)
    }
}

enum VariablePlan {
    Keep,
    Rebuild(Natural),
    Replacement,
}

fn rewrite(
    arena: &mut Arena,
    root: TermId,
    initial_scope: usize,
    operation: &mut Operation,
) -> Result<TermId, TransformError> {
    let mut frames = Vec::new();
    push_fallible(
        &mut frames,
        Frame::Visit {
            term_id: root,
            scope: initial_scope,
        },
    )?;
    let mut results = Vec::new();
    let mut memo = HashMap::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Visit { term_id, scope } => {
                if let Some(rewritten) = memo.get(&(term_id, scope)).copied() {
                    push_fallible(&mut results, rewritten)?;
                    continue;
                }

                let plan = match arena.get(term_id).expect("arena term-id invariant") {
                    Term::Var(index) => NodePlan::Variable(operation.plan_variable(index, scope)?),
                    term => Composite::from_term(term)
                        .map(NodePlan::Composite)
                        .unwrap_or(NodePlan::Keep),
                };

                match plan {
                    NodePlan::Keep | NodePlan::Variable(VariablePlan::Keep) => {
                        record_result(&mut results, &mut memo, term_id, scope, term_id)?;
                    }
                    NodePlan::Variable(VariablePlan::Rebuild(index)) => {
                        results.try_reserve(1).map_err(|_| TransformError)?;
                        memo.try_reserve(1).map_err(|_| TransformError)?;
                        let rewritten = arena.push(Term::Var(index)).map_err(from_format_error)?;
                        results.push(rewritten);
                        memo.insert((term_id, scope), rewritten);
                    }
                    NodePlan::Variable(VariablePlan::Replacement) => {
                        let replacement = operation.replacement_at_depth(arena, scope)?;
                        record_result(&mut results, &mut memo, term_id, scope, replacement)?;
                    }
                    NodePlan::Composite(composite) => {
                        let arity = composite.arity();
                        frames.try_reserve(arity + 1).map_err(|_| TransformError)?;
                        frames.push(Frame::Rebuild {
                            source: term_id,
                            composite,
                            scope,
                        });
                        for child_index in (0..arity).rev() {
                            let child_scope = if composite.binds(child_index) {
                                scope.checked_add(1).ok_or(TransformError)?
                            } else {
                                scope
                            };
                            frames.push(Frame::Visit {
                                term_id: composite.children[child_index],
                                scope: child_scope,
                            });
                        }
                    }
                }
            }
            Frame::Rebuild {
                source,
                composite,
                scope,
            } => {
                let arity = composite.arity();
                let first = results
                    .len()
                    .checked_sub(arity)
                    .expect("every child produces one rewritten term");
                let mut children = [TermId::from_index(0); 6];
                children[..arity].copy_from_slice(&results[first..]);
                results.truncate(first);
                results.try_reserve(1).map_err(|_| TransformError)?;
                memo.try_reserve(1).map_err(|_| TransformError)?;

                let rewritten = if composite.children[..arity] == children[..arity] {
                    source
                } else {
                    arena
                        .push(composite.rebuild(children))
                        .map_err(from_format_error)?
                };
                results.push(rewritten);
                memo.insert((source, scope), rewritten);
            }
        }
    }

    let result = results
        .pop()
        .expect("a rewrite produces exactly one root term");
    assert!(results.is_empty(), "a rewrite produces only one root term");
    Ok(result)
}

fn record_result(
    results: &mut Vec<TermId>,
    memo: &mut HashMap<(TermId, usize), TermId>,
    source: TermId,
    scope: usize,
    rewritten: TermId,
) -> Result<(), TransformError> {
    results.try_reserve(1).map_err(|_| TransformError)?;
    memo.try_reserve(1).map_err(|_| TransformError)?;
    results.push(rewritten);
    memo.insert((source, scope), rewritten);
    Ok(())
}

fn push_fallible<T>(items: &mut Vec<T>, item: T) -> Result<(), TransformError> {
    items.try_reserve(1).map_err(|_| TransformError)?;
    items.push(item);
    Ok(())
}

fn from_format_error(error: FormatError) -> TransformError {
    match error.class() {
        FormatErrorClass::ResourceExhausted => TransformError,
        _ => unreachable!("derived terms preserve the arena invariant"),
    }
}

enum NodePlan {
    Keep,
    Variable(VariablePlan),
    Composite(Composite),
}

#[derive(Clone, Copy)]
enum Frame {
    Visit {
        term_id: TermId,
        scope: usize,
    },
    Rebuild {
        source: TermId,
        composite: Composite,
        scope: usize,
    },
}

#[derive(Clone, Copy)]
struct Composite {
    shape: Shape,
    children: [TermId; 6],
}

impl Composite {
    fn from_term(term: &Term) -> Option<Self> {
        Some(match term {
            Term::Pi(a, b) => Self::new(Shape::Pi, &[*a, *b]),
            Term::Lam(a) => Self::new(Shape::Lam, &[*a]),
            Term::App(a, b) => Self::new(Shape::App, &[*a, *b]),
            Term::Sigma(a, b) => Self::new(Shape::Sigma, &[*a, *b]),
            Term::Pair(a, b) => Self::new(Shape::Pair, &[*a, *b]),
            Term::Fst(a) => Self::new(Shape::Fst, &[*a]),
            Term::Snd(a) => Self::new(Shape::Snd, &[*a]),
            Term::Id(a, b, c) => Self::new(Shape::Id, &[*a, *b, *c]),
            Term::Refl(a) => Self::new(Shape::Refl, &[*a]),
            Term::J(a, b, c, d, e, f) => Self::new(Shape::J, &[*a, *b, *c, *d, *e, *f]),
            Term::EmptyElim(a, b) => Self::new(Shape::EmptyElim, &[*a, *b]),
            Term::UnitElim(a, b, c) => Self::new(Shape::UnitElim, &[*a, *b, *c]),
            Term::Succ(a) => Self::new(Shape::Succ, &[*a]),
            Term::NatElim(a, b, c, d) => Self::new(Shape::NatElim, &[*a, *b, *c, *d]),
            Term::Ann(a, b) => Self::new(Shape::Ann, &[*a, *b]),
            Term::Var(_)
            | Term::Global(_)
            | Term::Universe(_)
            | Term::Empty
            | Term::Unit
            | Term::Star
            | Term::Nat
            | Term::Zero => return None,
        })
    }

    fn new(shape: Shape, children: &[TermId]) -> Self {
        let mut stored = [TermId::from_index(0); 6];
        stored[..children.len()].copy_from_slice(children);
        Self {
            shape,
            children: stored,
        }
    }

    const fn arity(self) -> usize {
        self.shape.arity()
    }

    const fn binds(self, child_index: usize) -> bool {
        matches!(
            (self.shape, child_index),
            (Shape::Pi | Shape::Sigma, 1) | (Shape::Lam, 0)
        )
    }

    fn rebuild(self, children: [TermId; 6]) -> Term {
        match self.shape {
            Shape::Pi => Term::Pi(children[0], children[1]),
            Shape::Lam => Term::Lam(children[0]),
            Shape::App => Term::App(children[0], children[1]),
            Shape::Sigma => Term::Sigma(children[0], children[1]),
            Shape::Pair => Term::Pair(children[0], children[1]),
            Shape::Fst => Term::Fst(children[0]),
            Shape::Snd => Term::Snd(children[0]),
            Shape::Id => Term::Id(children[0], children[1], children[2]),
            Shape::Refl => Term::Refl(children[0]),
            Shape::J => Term::J(
                children[0],
                children[1],
                children[2],
                children[3],
                children[4],
                children[5],
            ),
            Shape::EmptyElim => Term::EmptyElim(children[0], children[1]),
            Shape::UnitElim => Term::UnitElim(children[0], children[1], children[2]),
            Shape::Succ => Term::Succ(children[0]),
            Shape::NatElim => Term::NatElim(children[0], children[1], children[2], children[3]),
            Shape::Ann => Term::Ann(children[0], children[1]),
        }
    }
}

#[derive(Clone, Copy)]
enum Shape {
    Pi,
    Lam,
    App,
    Sigma,
    Pair,
    Fst,
    Snd,
    Id,
    Refl,
    J,
    EmptyElim,
    UnitElim,
    Succ,
    NatElim,
    Ann,
}

impl Shape {
    const fn arity(self) -> usize {
        match self {
            Self::Lam | Self::Fst | Self::Snd | Self::Refl | Self::Succ => 1,
            Self::Pi | Self::App | Self::Sigma | Self::Pair | Self::EmptyElim | Self::Ann => 2,
            Self::Id | Self::UnitElim => 3,
            Self::NatElim => 4,
            Self::J => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{shift, substitute_top};
    use crate::format::{parse_canonical, print_canonical};
    use crate::syntax::{Arena, Declaration, Module, Natural, Term, TermId};

    #[test]
    fn shift_covers_every_frozen_constructor() {
        let cases = [
            ("(var 0)", "(var 1)"),
            ("(global 8)", "(global 8)"),
            ("(universe 7)", "(universe 7)"),
            ("(pi (var 0) (var 0))", "(pi (var 1) (var 0))"),
            ("(lam (var 0))", "(lam (var 0))"),
            ("(app (var 0) (var 1))", "(app (var 1) (var 2))"),
            ("(sigma (var 0) (var 0))", "(sigma (var 1) (var 0))"),
            ("(pair (var 0) (var 1))", "(pair (var 1) (var 2))"),
            ("(fst (var 0))", "(fst (var 1))"),
            ("(snd (var 0))", "(snd (var 1))"),
            (
                "(id (var 0) (var 1) (var 2))",
                "(id (var 1) (var 2) (var 3))",
            ),
            ("(refl (var 0))", "(refl (var 1))"),
            (
                "(j (var 0) (var 1) (var 2) (var 3) (var 4) (var 5))",
                "(j (var 1) (var 2) (var 3) (var 4) (var 5) (var 6))",
            ),
            ("empty", "empty"),
            (
                "(empty-elim (var 0) (var 1))",
                "(empty-elim (var 1) (var 2))",
            ),
            ("unit", "unit"),
            ("star", "star"),
            (
                "(unit-elim (var 0) (var 1) (var 2))",
                "(unit-elim (var 1) (var 2) (var 3))",
            ),
            ("nat", "nat"),
            ("zero", "zero"),
            ("(succ (var 0))", "(succ (var 1))"),
            (
                "(nat-elim (var 0) (var 1) (var 2) (var 3))",
                "(nat-elim (var 1) (var 2) (var 3) (var 4))",
            ),
            ("(ann (var 0) (var 1))", "(ann (var 1) (var 2))"),
        ];

        for (source, expected) in cases {
            assert_shift(source, 1, 0, expected);
        }
    }

    #[test]
    fn shift_respects_cutoffs_and_composes() {
        assert_shift("(pair (var 1) (var 95))", 17, 2, "(pair (var 1) (var 112))");

        // A nonzero outer cutoff is retained in domains and raised only in
        // the binding argument of each frozen binder.
        assert_shift(
            "(pi (pair (var 0) (var 1)) (pair (var 0) (pair (var 1) (var 2))))",
            1,
            1,
            "(pi (pair (var 0) (var 2)) (pair (var 0) (pair (var 1) (var 3))))",
        );
        assert_shift(
            "(sigma (pair (var 0) (var 1)) (pair (var 0) (pair (var 1) (var 2))))",
            1,
            1,
            "(sigma (pair (var 0) (var 2)) (pair (var 0) (pair (var 1) (var 3))))",
        );
        assert_shift(
            "(lam (pair (var 0) (pair (var 1) (var 2))))",
            1,
            1,
            "(lam (pair (var 0) (pair (var 1) (var 3))))",
        );

        let source = "(lam (pair (var 0) (pair (var 1) (var 7))))";
        let composed = render_shift_sequence(source, &[(2, 0), (3, 0)]);
        let combined = render_shift_sequence(source, &[(5, 0)]);
        assert_eq!(composed, combined);
    }

    #[test]
    fn unchanged_shifts_reuse_the_original_graph() {
        let (mut arena, _, root) = fixture("(id (global 0) (universe 1) unit)");
        let original_len = arena.len();
        assert_eq!(shift(&mut arena, root, 4, 0).unwrap(), root);
        assert_eq!(arena.len(), original_len);

        let (mut arena, _, root) = fixture("(app (var 0) (var 1))");
        let original_len = arena.len();
        assert_eq!(shift(&mut arena, root, 0, 0).unwrap(), root);
        assert_eq!(arena.len(), original_len);
    }

    #[test]
    fn unbounded_indices_shift_and_decrement_without_machine_conversion() {
        let nines = "9".repeat(512);
        let power_of_ten = format!("1{}", "0".repeat(512));
        assert_shift(
            &format!("(var {nines})"),
            1,
            0,
            &format!("(var {power_of_ten})"),
        );
        assert_substitution(
            &format!("(var {power_of_ten})"),
            "star",
            &format!("(var {nines})"),
        );
    }

    #[test]
    fn top_substitution_removes_the_newest_binder() {
        assert_substitution(
            "(pair (var 0) (pair (var 1) (var 2)))",
            "star",
            "(pair star (pair (var 0) (var 1)))",
        );
    }

    #[test]
    fn top_substitution_lifts_only_through_binding_arguments() {
        let replacement = "(pair (var 0) star)";
        let domain_expected = "(pair (pair (var 0) star) (var 0))";
        let codomain_expected = "(pair (var 0) (pair (pair (var 1) star) (var 1)))";

        assert_substitution(
            "(pi (pair (var 0) (var 1)) (pair (var 0) (pair (var 1) (var 2))))",
            replacement,
            &format!("(pi {domain_expected} {codomain_expected})"),
        );
        assert_substitution(
            "(sigma (pair (var 0) (var 1)) (pair (var 0) (pair (var 1) (var 2))))",
            replacement,
            &format!("(sigma {domain_expected} {codomain_expected})"),
        );
        assert_substitution(
            "(lam (pair (var 0) (pair (var 1) (var 2))))",
            replacement,
            &format!("(lam {codomain_expected})"),
        );
    }

    #[test]
    fn repeated_lifted_replacements_share_one_rewrite() {
        let (mut arena, _, body, replacement) =
            substitution_fixture("(lam (pair (var 1) (var 1)))", "(var 0)");
        let rewritten = substitute_top(&mut arena, body, replacement).unwrap();
        let Term::Lam(pair) = arena.get(rewritten).unwrap() else {
            panic!("rewritten root must remain a lambda")
        };
        let Term::Pair(left, right) = arena.get(*pair).unwrap() else {
            panic!("rewritten lambda body must remain a pair")
        };
        assert_eq!(left, right);
    }

    #[test]
    fn shared_changed_subgraphs_are_rewritten_once_per_scope() {
        const DEPTH: usize = 18;
        let mut arena = Arena::new();
        let variable = arena
            .push(Term::Var(Natural::from_decimal("0").unwrap()))
            .unwrap();
        let mut body = variable;
        for _ in 0..DEPTH {
            body = arena.push(Term::Pair(body, body)).unwrap();
        }
        let replacement = arena.push(Term::Star).unwrap();

        let mut shifted_arena = arena.clone();
        let shifted_start = shifted_arena.len();
        let shifted = shift(&mut shifted_arena, body, 1, 0).unwrap();
        assert_eq!(shifted_arena.len(), shifted_start + DEPTH + 1);
        let shifted_leaf = shared_pair_leaf(&shifted_arena, shifted, DEPTH);
        let Term::Var(index) = shifted_arena.get(shifted_leaf).unwrap() else {
            panic!("shifted shared leaf must remain a variable")
        };
        assert_eq!(index.as_str(), "1");

        let substitution_start = arena.len();
        let substituted = substitute_top(&mut arena, body, replacement).unwrap();
        assert_eq!(arena.len(), substitution_start + DEPTH);
        assert_eq!(shared_pair_leaf(&arena, substituted, DEPTH), replacement);

        let mut scoped_arena = Arena::new();
        let shared = scoped_arena
            .push(Term::Var(Natural::from_decimal("0").unwrap()))
            .unwrap();
        let pi = scoped_arena.push(Term::Pi(shared, shared)).unwrap();
        let shifted_pi = shift(&mut scoped_arena, pi, 1, 0).unwrap();
        let Term::Pi(domain, codomain) = scoped_arena.get(shifted_pi).unwrap() else {
            panic!("shifted root must remain a pi")
        };
        assert_ne!(domain, codomain);
        assert_eq!(*codomain, shared);
        let Term::Var(domain_index) = scoped_arena.get(*domain).unwrap() else {
            panic!("shifted pi domain must remain a variable")
        };
        assert_eq!(domain_index.as_str(), "1");
    }

    #[test]
    fn deep_transformations_do_not_use_the_rust_call_stack() {
        const DEPTH: usize = 10_000;
        let source = nested_lambdas(DEPTH, DEPTH);
        let shifted = nested_lambdas(DEPTH, DEPTH + 1);
        assert_shift(&source, 1, 0, &shifted);
        assert_substitution(&source, "(var 0)", &source);
    }

    fn assert_shift(source: &str, increment: usize, cutoff: usize, expected: &str) {
        let actual = render_shift_sequence(source, &[(increment, cutoff)]);
        assert_eq!(actual, canonical(expected));
    }

    fn render_shift_sequence(source: &str, shifts: &[(usize, usize)]) -> String {
        let (mut arena, ty, mut root) = fixture(source);
        for (increment, cutoff) in shifts {
            root = shift(&mut arena, root, *increment, *cutoff).unwrap();
        }
        render(arena, ty, root)
    }

    fn assert_substitution(body: &str, replacement: &str, expected: &str) {
        let (mut arena, ty, body, replacement) = substitution_fixture(body, replacement);
        let rewritten = substitute_top(&mut arena, body, replacement).unwrap();
        assert_eq!(render(arena, ty, rewritten), canonical(expected));
    }

    fn fixture(term: &str) -> (Arena, TermId, TermId) {
        let module = parse_canonical(canonical(term).as_bytes()).unwrap();
        let declaration = &module.declarations()[0];
        (
            module.arena().clone(),
            declaration.ty(),
            declaration.body().unwrap(),
        )
    }

    fn substitution_fixture(body: &str, replacement: &str) -> (Arena, TermId, TermId, TermId) {
        let (arena, ty, pair) = fixture(&format!("(pair {body} {replacement})"));
        let (body, replacement) = match arena.get(pair).unwrap() {
            Term::Pair(body, replacement) => (*body, *replacement),
            _ => unreachable!("fixture root is a pair"),
        };
        (arena, ty, body, replacement)
    }

    fn render(arena: Arena, ty: TermId, body: TermId) -> String {
        let module = Module::new(
            arena,
            vec![Declaration::Transparent {
                name: String::from("fixture"),
                ty,
                body,
            }],
        );
        String::from_utf8(print_canonical(&module).unwrap()).unwrap()
    }

    fn canonical(term: &str) -> String {
        format!(
            "(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (transparent \"fixture\" unit {term})))\n"
        )
    }

    fn nested_lambdas(depth: usize, index: usize) -> String {
        let mut term = String::new();
        for _ in 0..depth {
            term.push_str("(lam ");
        }
        term.push_str(&format!("(var {index})"));
        for _ in 0..depth {
            term.push(')');
        }
        term
    }

    fn shared_pair_leaf(arena: &Arena, mut term_id: TermId, depth: usize) -> TermId {
        for _ in 0..depth {
            let Term::Pair(left, right) = arena.get(term_id).unwrap() else {
                panic!("shared spine must remain a pair")
            };
            assert_eq!(left, right);
            term_id = *left;
        }
        term_id
    }
}

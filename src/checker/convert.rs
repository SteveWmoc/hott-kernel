use super::CheckError;
use super::reduce::whnf;
use super::state::CheckedGlobals;
use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Term, TermId};
use std::collections::{HashMap, HashSet};

/// Fully beta-delta-iota normalize an already validated Core v0.1 term.
///
/// The traversal is iterative and memoized over the append-only arena. Opaque
/// globals and postulates remain neutral, annotations are erased, and no eta
/// rule is introduced. Nodes appended during a failed normalization are
/// removed before returning. Successful normalization may retain derived nodes
/// because the returned `TermId` can refer to them.
pub(super) fn normalize(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<TermId, CheckError> {
    assert!(
        arena.get(root).is_some(),
        "normalization root term id must belong to arena"
    );
    let checkpoint = arena.len();
    let result = normalize_inner(arena, globals, root);
    if result.is_err() {
        arena.truncate(checkpoint);
    }
    result
}

/// Decide algorithmic conversion of two already validated terms.
///
/// Both terms are fully normalized according to the frozen Core v0.1
/// beta-delta-iota rules and their normal forms are compared structurally.
/// Arena identity is not part of judgmental equality. Conversion returns only
/// a boolean, so no normalization-derived `TermId` escapes: all nodes appended
/// during the comparison are discarded before returning, whether the result is
/// `Ok(true)`, `Ok(false)`, or an error.
pub(super) fn convert(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    left: TermId,
    right: TermId,
) -> Result<bool, CheckError> {
    assert!(
        arena.get(left).is_some() && arena.get(right).is_some(),
        "conversion inputs must belong to arena"
    );
    let checkpoint = arena.len();
    let result = (|| {
        let left = normalize_inner(arena, globals, left)?;
        let right = normalize_inner(arena, globals, right)?;
        structurally_equal(arena, globals.next_declaration_index(), left, right)
    })();
    arena.truncate(checkpoint);
    result
}

fn normalize_inner(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<TermId, CheckError> {
    let declaration_index = globals.next_declaration_index();
    let mut frames = Vec::new();
    let mut results = Vec::new();
    let mut memo = HashMap::new();
    push_normalize_frame(&mut frames, declaration_index, NormalizeFrame::Visit(root))?;

    while let Some(frame) = frames.pop() {
        match frame {
            NormalizeFrame::Visit(source) => {
                if let Some(normalized) = memo.get(&source).copied() {
                    push_result(&mut results, declaration_index, normalized)?;
                    continue;
                }

                let exposed = whnf(arena, globals, source)?;
                if let Some(normalized) = memo.get(&exposed).copied() {
                    record_result(
                        &mut results,
                        &mut memo,
                        declaration_index,
                        source,
                        exposed,
                        normalized,
                    )?;
                    continue;
                }

                let Some(composite) = Composite::from_term(
                    arena
                        .get(exposed)
                        .expect("WHNF result belongs to normalization arena"),
                ) else {
                    record_result(
                        &mut results,
                        &mut memo,
                        declaration_index,
                        source,
                        exposed,
                        exposed,
                    )?;
                    continue;
                };

                let arity = composite.arity();
                frames
                    .try_reserve(arity + 1)
                    .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
                frames.push(NormalizeFrame::Rebuild {
                    source,
                    exposed,
                    composite,
                });
                for child in composite.children[..arity].iter().rev() {
                    frames.push(NormalizeFrame::Visit(*child));
                }
            }
            NormalizeFrame::Rebuild {
                source,
                exposed,
                composite,
            } => {
                let arity = composite.arity();
                let first = results
                    .len()
                    .checked_sub(arity)
                    .expect("every normalization child produces one result");
                let mut children = [TermId::from_index(0); 6];
                children[..arity].copy_from_slice(&results[first..]);
                results.truncate(first);

                let normalized = if composite.children[..arity] == children[..arity] {
                    exposed
                } else {
                    append(arena, declaration_index, composite.rebuild(children))?
                };
                record_result(
                    &mut results,
                    &mut memo,
                    declaration_index,
                    source,
                    exposed,
                    normalized,
                )?;
            }
        }
    }

    let result = results
        .pop()
        .expect("normalization produces exactly one root result");
    assert!(
        results.is_empty(),
        "normalization produces only one root result"
    );
    Ok(result)
}

fn structurally_equal(
    arena: &Arena,
    declaration_index: usize,
    left: TermId,
    right: TermId,
) -> Result<bool, CheckError> {
    let mut pending = Vec::new();
    let mut seen = HashSet::new();
    push_pair(&mut pending, declaration_index, left, right)?;

    while let Some((left, right)) = pending.pop() {
        if left == right {
            continue;
        }
        if seen.contains(&(left, right)) {
            continue;
        }
        seen.try_reserve(1)
            .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
        seen.insert((left, right));

        let left_term = arena
            .get(left)
            .expect("left normal form belongs to conversion arena");
        let right_term = arena
            .get(right)
            .expect("right normal form belongs to conversion arena");

        match (left_term, right_term) {
            (Term::Var(a), Term::Var(b))
            | (Term::Global(a), Term::Global(b))
            | (Term::Universe(a), Term::Universe(b)) => {
                if a != b {
                    return Ok(false);
                }
            }
            (Term::Pi(a1, b1), Term::Pi(a2, b2))
            | (Term::App(a1, b1), Term::App(a2, b2))
            | (Term::Sigma(a1, b1), Term::Sigma(a2, b2))
            | (Term::Pair(a1, b1), Term::Pair(a2, b2))
            | (Term::EmptyElim(a1, b1), Term::EmptyElim(a2, b2))
            | (Term::Ann(a1, b1), Term::Ann(a2, b2)) => {
                push_pairs(&mut pending, declaration_index, &[(*a1, *a2), (*b1, *b2)])?;
            }
            (Term::Lam(a), Term::Lam(b))
            | (Term::Fst(a), Term::Fst(b))
            | (Term::Snd(a), Term::Snd(b))
            | (Term::Refl(a), Term::Refl(b))
            | (Term::Succ(a), Term::Succ(b)) => {
                push_pair(&mut pending, declaration_index, *a, *b)?;
            }
            (Term::Id(a1, b1, c1), Term::Id(a2, b2, c2))
            | (Term::UnitElim(a1, b1, c1), Term::UnitElim(a2, b2, c2)) => {
                push_pairs(
                    &mut pending,
                    declaration_index,
                    &[(*a1, *a2), (*b1, *b2), (*c1, *c2)],
                )?;
            }
            (Term::J(a1, b1, c1, d1, e1, f1), Term::J(a2, b2, c2, d2, e2, f2)) => {
                push_pairs(
                    &mut pending,
                    declaration_index,
                    &[
                        (*a1, *a2),
                        (*b1, *b2),
                        (*c1, *c2),
                        (*d1, *d2),
                        (*e1, *e2),
                        (*f1, *f2),
                    ],
                )?;
            }
            (Term::NatElim(a1, b1, c1, d1), Term::NatElim(a2, b2, c2, d2)) => {
                push_pairs(
                    &mut pending,
                    declaration_index,
                    &[(*a1, *a2), (*b1, *b2), (*c1, *c2), (*d1, *d2)],
                )?;
            }
            (Term::Empty, Term::Empty)
            | (Term::Unit, Term::Unit)
            | (Term::Star, Term::Star)
            | (Term::Nat, Term::Nat)
            | (Term::Zero, Term::Zero) => {}
            _ => return Ok(false),
        }
    }

    Ok(true)
}

fn push_normalize_frame(
    frames: &mut Vec<NormalizeFrame>,
    declaration_index: usize,
    frame: NormalizeFrame,
) -> Result<(), CheckError> {
    frames
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    frames.push(frame);
    Ok(())
}

fn push_result(
    results: &mut Vec<TermId>,
    declaration_index: usize,
    result: TermId,
) -> Result<(), CheckError> {
    results
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    results.push(result);
    Ok(())
}

fn record_result(
    results: &mut Vec<TermId>,
    memo: &mut HashMap<TermId, TermId>,
    declaration_index: usize,
    source: TermId,
    exposed: TermId,
    normalized: TermId,
) -> Result<(), CheckError> {
    results
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    let needed = usize::from(source != exposed) + 1;
    memo.try_reserve(needed)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    memo.insert(exposed, normalized);
    memo.insert(source, normalized);
    results.push(normalized);
    Ok(())
}

fn push_pair(
    pending: &mut Vec<(TermId, TermId)>,
    declaration_index: usize,
    left: TermId,
    right: TermId,
) -> Result<(), CheckError> {
    pending
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    pending.push((left, right));
    Ok(())
}

fn push_pairs(
    pending: &mut Vec<(TermId, TermId)>,
    declaration_index: usize,
    pairs: &[(TermId, TermId)],
) -> Result<(), CheckError> {
    pending
        .try_reserve(pairs.len())
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    pending.extend_from_slice(pairs);
    Ok(())
}

fn append(arena: &mut Arena, declaration_index: usize, term: Term) -> Result<TermId, CheckError> {
    arena
        .push(term)
        .map_err(|error| from_format_error(error, declaration_index))
}

fn from_format_error(error: FormatError, declaration_index: usize) -> CheckError {
    match error.class() {
        FormatErrorClass::ResourceExhausted => CheckError::resource_exhausted(declaration_index),
        _ => unreachable!("normalizer-derived terms preserve the arena invariant"),
    }
}

#[derive(Clone, Copy)]
enum NormalizeFrame {
    Visit(TermId),
    Rebuild {
        source: TermId,
        exposed: TermId,
        composite: Composite,
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
    use super::{convert, normalize};
    use crate::checker::state::CheckedGlobals;
    use crate::checker::{CheckErrorClass, ReferenceKind};
    use crate::syntax::{Arena, Declaration, Natural, Term};

    fn natural(text: &str) -> Natural {
        Natural::from_decimal(text).expect("test natural is canonical")
    }

    #[test]
    fn normalization_reduces_below_visible_constructors_and_reuses_shared_results() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let identity = arena.push(Term::Lam(var_zero)).unwrap();
        let beta = arena.push(Term::App(identity, zero)).unwrap();
        let annotated_beta = arena.push(Term::Ann(beta, nat)).unwrap();
        let pair = arena
            .push(Term::Pair(annotated_beta, annotated_beta))
            .unwrap();
        let globals = CheckedGlobals::new();

        let normalized = normalize(&mut arena, &globals, pair).unwrap();
        let Term::Pair(first, second) = arena.get(normalized).unwrap() else {
            panic!("normalizing a visible pair must preserve its constructor");
        };
        assert_eq!((*first, *second), (zero, zero));
        assert_eq!(first, second, "shared children normalize to one arena node");
    }

    #[test]
    fn normalization_unfolds_transparent_globals_below_visible_constructors() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let body = arena.push(Term::Succ(zero)).unwrap();
        let transparent = Declaration::Transparent {
            name: "one".to_owned(),
            ty: nat,
            body,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&transparent).unwrap();
        let global = arena.push(Term::Global(natural("0"))).unwrap();
        let pair = arena.push(Term::Pair(global, zero)).unwrap();

        let normalized = normalize(&mut arena, &globals, pair).unwrap();
        let Term::Pair(first, second) = arena.get(normalized).unwrap() else {
            panic!("normalizing a visible pair must preserve its constructor");
        };
        assert_eq!((*first, *second), (body, zero));
    }

    #[test]
    fn conversion_matches_beta_delta_iota_normal_forms() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let identity = arena.push(Term::Lam(var_zero)).unwrap();
        let beta = arena.push(Term::App(identity, zero)).unwrap();
        let annotated_beta = arena.push(Term::Ann(beta, nat)).unwrap();
        let transparent = Declaration::Transparent {
            name: "z".to_owned(),
            ty: nat,
            body: annotated_beta,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&transparent).unwrap();
        let global = arena.push(Term::Global(natural("0"))).unwrap();
        let nat_elim = arena
            .push(Term::NatElim(nat, zero, var_zero, zero))
            .unwrap();

        assert!(convert(&mut arena, &globals, beta, zero).unwrap());
        assert!(convert(&mut arena, &globals, global, zero).unwrap());
        assert!(convert(&mut arena, &globals, nat_elim, zero).unwrap());
    }

    #[test]
    fn conversion_discards_derived_nodes_for_equal_and_unequal_results() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let star = arena.push(Term::Star).unwrap();
        let annotated = arena.push(Term::Ann(zero, nat)).unwrap();
        let reducible_pair = arena.push(Term::Pair(annotated, zero)).unwrap();
        let equal_pair = arena.push(Term::Pair(zero, zero)).unwrap();
        let unequal_pair = arena.push(Term::Pair(zero, star)).unwrap();
        let checkpoint = arena.len();
        let globals = CheckedGlobals::new();

        assert!(convert(&mut arena, &globals, reducible_pair, equal_pair).unwrap());
        assert_eq!(arena.len(), checkpoint);
        assert!(!convert(&mut arena, &globals, reducible_pair, unequal_pair).unwrap());
        assert_eq!(arena.len(), checkpoint);
    }

    #[test]
    fn conversion_respects_opacity_and_distinguishes_normal_forms() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let star = arena.push(Term::Star).unwrap();
        let opaque = Declaration::Opaque {
            name: "hidden".to_owned(),
            ty: nat,
            body: zero,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&opaque).unwrap();
        let global = arena.push(Term::Global(natural("0"))).unwrap();
        let left_pair = arena.push(Term::Pair(zero, zero)).unwrap();
        let right_pair = arena.push(Term::Pair(zero, star)).unwrap();

        assert!(!convert(&mut arena, &globals, global, zero).unwrap());
        assert!(!convert(&mut arena, &globals, left_pair, right_pair).unwrap());
        assert_eq!(normalize(&mut arena, &globals, global).unwrap(), global);
    }

    #[test]
    fn conversion_compares_structure_not_arena_identity() {
        let mut arena = Arena::new();
        let nat_left = arena.push(Term::Nat).unwrap();
        let zero_left = arena.push(Term::Zero).unwrap();
        let left = arena
            .push(Term::Id(nat_left, zero_left, zero_left))
            .unwrap();
        let nat_right = arena.push(Term::Nat).unwrap();
        let zero_right = arena.push(Term::Zero).unwrap();
        let right = arena
            .push(Term::Id(nat_right, zero_right, zero_right))
            .unwrap();
        let globals = CheckedGlobals::new();

        assert_ne!(left, right);
        assert!(convert(&mut arena, &globals, left, right).unwrap());
    }

    #[test]
    fn normalization_and_conversion_are_stack_safe_on_deep_terms() {
        const DEPTH: usize = 10_000;
        let mut arena = Arena::new();
        let zero_left = arena.push(Term::Zero).unwrap();
        let mut left = zero_left;
        for _ in 0..DEPTH {
            left = arena.push(Term::Succ(left)).unwrap();
        }
        let zero_right = arena.push(Term::Zero).unwrap();
        let mut right = zero_right;
        for _ in 0..DEPTH {
            right = arena.push(Term::Succ(right)).unwrap();
        }
        let globals = CheckedGlobals::new();

        assert_eq!(normalize(&mut arena, &globals, left).unwrap(), left);
        let checkpoint = arena.len();
        assert!(convert(&mut arena, &globals, left, right).unwrap());
        assert_eq!(arena.len(), checkpoint);
    }

    #[test]
    fn failed_conversion_rolls_back_nodes_derived_from_either_side() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let annotated = arena.push(Term::Ann(zero, nat)).unwrap();
        let left = arena.push(Term::Pair(annotated, zero)).unwrap();
        let right = arena.push(Term::Global(natural("0"))).unwrap();
        let checkpoint = arena.len();
        let globals = CheckedGlobals::new();

        let error = convert(&mut arena, &globals, left, right).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Global));
        assert_eq!(arena.len(), checkpoint);
    }
}

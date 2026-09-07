use super::CheckError;
use super::state::CheckedGlobals;
use super::transform::{TransformError, substitute_top};
use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Term, TermId};

/// Reduce an already validated term to weak-head normal form.
///
/// This implements only the deterministic Core v0.1 head strategy. It does
/// not recursively normalize constructor arguments, unfold opaque globals, or
/// perform eta expansion. Nodes appended during a failed reduction are rolled
/// back before returning.
pub(super) fn whnf(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<TermId, CheckError> {
    assert!(
        arena.get(root).is_some(),
        "WHNF root term id must belong to arena"
    );
    let checkpoint = arena.len();
    let result = whnf_inner(arena, globals, root);
    if result.is_err() {
        arena.truncate(checkpoint);
    }
    result
}

/// Expose a dependent-function type after weak-head reduction.
pub(super) fn expose_pi(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<Option<(TermId, TermId)>, CheckError> {
    let exposed = whnf(arena, globals, root)?;
    let result = match arena.get(exposed).expect("WHNF result belongs to arena") {
        Term::Pi(domain, codomain) => Some((*domain, *codomain)),
        _ => None,
    };
    Ok(result)
}

/// Expose a dependent-pair type after weak-head reduction.
pub(super) fn expose_sigma(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<Option<(TermId, TermId)>, CheckError> {
    let exposed = whnf(arena, globals, root)?;
    let result = match arena.get(exposed).expect("WHNF result belongs to arena") {
        Term::Sigma(domain, codomain) => Some((*domain, *codomain)),
        _ => None,
    };
    Ok(result)
}

fn whnf_inner(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    root: TermId,
) -> Result<TermId, CheckError> {
    let declaration_index = globals.next_declaration_index();
    let mut current = root;
    let mut frames = Vec::new();

    'evaluate: loop {
        let term = arena
            .get(current)
            .expect("WHNF current term belongs to arena")
            .clone();

        match term {
            Term::Ann(value, _) => {
                current = value;
                continue;
            }
            Term::Global(index) => {
                let entry = globals.lookup(current, &index)?;
                if let Some(body) = entry.unfolding_body() {
                    current = body;
                    continue;
                }
            }
            Term::App(function, argument) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::App {
                        source: current,
                        function,
                        argument,
                    },
                )?;
                current = function;
                continue;
            }
            Term::Fst(principal) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::Fst {
                        source: current,
                        principal,
                    },
                )?;
                current = principal;
                continue;
            }
            Term::Snd(principal) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::Snd {
                        source: current,
                        principal,
                    },
                )?;
                current = principal;
                continue;
            }
            Term::J(a, base, motive, branch, endpoint, path) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::J {
                        source: current,
                        a,
                        base,
                        motive,
                        branch,
                        endpoint,
                        path,
                    },
                )?;
                current = path;
                continue;
            }
            Term::EmptyElim(motive, scrutinee) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::EmptyElim {
                        source: current,
                        motive,
                        scrutinee,
                    },
                )?;
                current = scrutinee;
                continue;
            }
            Term::UnitElim(motive, branch, scrutinee) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::UnitElim {
                        source: current,
                        motive,
                        branch,
                        scrutinee,
                    },
                )?;
                current = scrutinee;
                continue;
            }
            Term::NatElim(motive, zero_branch, succ_branch, scrutinee) => {
                push_frame(
                    &mut frames,
                    declaration_index,
                    Frame::NatElim {
                        source: current,
                        motive,
                        zero_branch,
                        succ_branch,
                        scrutinee,
                    },
                )?;
                current = scrutinee;
                continue;
            }
            Term::Var(_)
            | Term::Universe(_)
            | Term::Pi(_, _)
            | Term::Lam(_)
            | Term::Sigma(_, _)
            | Term::Pair(_, _)
            | Term::Id(_, _, _)
            | Term::Refl(_)
            | Term::Empty
            | Term::Unit
            | Term::Star
            | Term::Nat
            | Term::Zero
            | Term::Succ(_) => {}
        }

        let mut value = current;
        loop {
            let Some(frame) = frames.pop() else {
                return Ok(value);
            };

            match frame {
                Frame::App {
                    source,
                    function,
                    argument,
                } => {
                    if let Term::Lam(body) = arena
                        .get(value)
                        .expect("exposed function belongs to arena")
                        .clone()
                    {
                        current = substitute_top(arena, body, argument)
                            .map_err(|error| from_transform_error(error, declaration_index))?;
                        continue 'evaluate;
                    }
                    value = rebuild_if_changed(
                        arena,
                        declaration_index,
                        source,
                        function,
                        value,
                        |principal| Term::App(principal, argument),
                    )?;
                }
                Frame::Fst { source, principal } => {
                    if let Term::Pair(first, _) = arena
                        .get(value)
                        .expect("exposed projection principal belongs to arena")
                        .clone()
                    {
                        current = first;
                        continue 'evaluate;
                    }
                    value = rebuild_if_changed(
                        arena,
                        declaration_index,
                        source,
                        principal,
                        value,
                        Term::Fst,
                    )?;
                }
                Frame::Snd { source, principal } => {
                    if let Term::Pair(_, second) = arena
                        .get(value)
                        .expect("exposed projection principal belongs to arena")
                        .clone()
                    {
                        current = second;
                        continue 'evaluate;
                    }
                    value = rebuild_if_changed(
                        arena,
                        declaration_index,
                        source,
                        principal,
                        value,
                        Term::Snd,
                    )?;
                }
                Frame::J {
                    source,
                    a,
                    base,
                    motive,
                    branch,
                    endpoint,
                    path,
                } => {
                    if matches!(
                        arena
                            .get(value)
                            .expect("exposed identity path belongs to arena"),
                        Term::Refl(_)
                    ) {
                        current = branch;
                        continue 'evaluate;
                    }
                    value = if value == path {
                        source
                    } else {
                        append(
                            arena,
                            declaration_index,
                            Term::J(a, base, motive, branch, endpoint, value),
                        )?
                    };
                }
                Frame::EmptyElim {
                    source,
                    motive,
                    scrutinee,
                } => {
                    // Empty has no constructor, so exposure can only rebuild a
                    // neutral eliminator; it never creates an iota step.
                    value = if value == scrutinee {
                        source
                    } else {
                        append(arena, declaration_index, Term::EmptyElim(motive, value))?
                    };
                }
                Frame::UnitElim {
                    source,
                    motive,
                    branch,
                    scrutinee,
                } => {
                    if matches!(
                        arena
                            .get(value)
                            .expect("exposed unit scrutinee belongs to arena"),
                        Term::Star
                    ) {
                        current = branch;
                        continue 'evaluate;
                    }
                    value = if value == scrutinee {
                        source
                    } else {
                        append(
                            arena,
                            declaration_index,
                            Term::UnitElim(motive, branch, value),
                        )?
                    };
                }
                Frame::NatElim {
                    source,
                    motive,
                    zero_branch,
                    succ_branch,
                    scrutinee,
                } => match arena
                    .get(value)
                    .expect("exposed natural scrutinee belongs to arena")
                    .clone()
                {
                    Term::Zero => {
                        current = zero_branch;
                        continue 'evaluate;
                    }
                    Term::Succ(predecessor) => {
                        let recursive = append(
                            arena,
                            declaration_index,
                            Term::NatElim(motive, zero_branch, succ_branch, predecessor),
                        )?;
                        let step = append(
                            arena,
                            declaration_index,
                            Term::App(succ_branch, predecessor),
                        )?;
                        current = append(arena, declaration_index, Term::App(step, recursive))?;
                        continue 'evaluate;
                    }
                    _ => {
                        value = if value == scrutinee {
                            source
                        } else {
                            append(
                                arena,
                                declaration_index,
                                Term::NatElim(motive, zero_branch, succ_branch, value),
                            )?
                        };
                    }
                },
            }
        }
    }
}

fn rebuild_if_changed(
    arena: &mut Arena,
    declaration_index: usize,
    source: TermId,
    original_principal: TermId,
    exposed_principal: TermId,
    rebuild: impl FnOnce(TermId) -> Term,
) -> Result<TermId, CheckError> {
    if original_principal == exposed_principal {
        Ok(source)
    } else {
        append(arena, declaration_index, rebuild(exposed_principal))
    }
}

fn push_frame(
    frames: &mut Vec<Frame>,
    declaration_index: usize,
    frame: Frame,
) -> Result<(), CheckError> {
    frames
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    frames.push(frame);
    Ok(())
}

fn append(arena: &mut Arena, declaration_index: usize, term: Term) -> Result<TermId, CheckError> {
    arena
        .push(term)
        .map_err(|error| from_format_error(error, declaration_index))
}

/// Checker transformations deliberately collapse only resource/allocation
/// failures into `TransformError`; impossible derived-format failures are
/// trapped inside `transform.rs` rather than returned. Keep that contract
/// explicit here instead of discarding the error with a blanket closure.
fn from_transform_error(_error: TransformError, declaration_index: usize) -> CheckError {
    CheckError::resource_exhausted(declaration_index)
}

fn from_format_error(error: FormatError, declaration_index: usize) -> CheckError {
    match error.class() {
        FormatErrorClass::ResourceExhausted => CheckError::resource_exhausted(declaration_index),
        _ => unreachable!("checker-derived terms preserve the arena invariant"),
    }
}

#[derive(Clone, Copy)]
enum Frame {
    App {
        source: TermId,
        function: TermId,
        argument: TermId,
    },
    Fst {
        source: TermId,
        principal: TermId,
    },
    Snd {
        source: TermId,
        principal: TermId,
    },
    J {
        source: TermId,
        a: TermId,
        base: TermId,
        motive: TermId,
        branch: TermId,
        endpoint: TermId,
        path: TermId,
    },
    EmptyElim {
        source: TermId,
        motive: TermId,
        scrutinee: TermId,
    },
    UnitElim {
        source: TermId,
        motive: TermId,
        branch: TermId,
        scrutinee: TermId,
    },
    NatElim {
        source: TermId,
        motive: TermId,
        zero_branch: TermId,
        succ_branch: TermId,
        scrutinee: TermId,
    },
}

#[cfg(test)]
mod tests {
    use super::{expose_pi, expose_sigma, whnf};
    use crate::checker::state::CheckedGlobals;
    use crate::checker::{CheckErrorClass, ReferenceKind};
    use crate::syntax::{Arena, Declaration, Natural, Term};

    fn natural(text: &str) -> Natural {
        Natural::from_decimal(text).expect("test natural is canonical")
    }

    #[test]
    fn whnf_performs_all_nine_frozen_head_reductions() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let star = arena.push(Term::Star).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let lam = arena.push(Term::Lam(var_zero)).unwrap();
        let beta = arena.push(Term::App(lam, zero)).unwrap();
        let pair = arena.push(Term::Pair(zero, star)).unwrap();
        let fst = arena.push(Term::Fst(pair)).unwrap();
        let snd = arena.push(Term::Snd(pair)).unwrap();
        let refl = arena.push(Term::Refl(zero)).unwrap();
        let j = arena
            .push(Term::J(nat, zero, nat, star, zero, refl))
            .unwrap();
        let unit_elim = arena.push(Term::UnitElim(nat, zero, star)).unwrap();
        let nat_zero = arena
            .push(Term::NatElim(nat, star, var_zero, zero))
            .unwrap();
        let succ_zero = arena.push(Term::Succ(zero)).unwrap();
        let nat_succ = arena
            .push(Term::NatElim(nat, star, var_zero, succ_zero))
            .unwrap();
        let annotation = arena.push(Term::Ann(zero, nat)).unwrap();

        let transparent = Declaration::Transparent {
            name: "z".to_owned(),
            ty: nat,
            body: annotation,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&transparent).unwrap();
        let global = arena.push(Term::Global(natural("0"))).unwrap();

        assert_eq!(whnf(&mut arena, &globals, beta).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, fst).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, snd).unwrap(), star);
        assert_eq!(whnf(&mut arena, &globals, j).unwrap(), star);
        assert_eq!(whnf(&mut arena, &globals, unit_elim).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, nat_zero).unwrap(), star);
        assert_eq!(whnf(&mut arena, &globals, annotation).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, global).unwrap(), zero);

        let succ_result = whnf(&mut arena, &globals, nat_succ).unwrap();
        let Term::App(step_application, recursive) = arena.get(succ_result).unwrap() else {
            panic!("successor iota must produce the recursive step application");
        };
        let Term::App(step, predecessor) = arena.get(*step_application).unwrap() else {
            panic!("successor iota must first apply the step to the predecessor");
        };
        assert_eq!(*step, var_zero);
        assert_eq!(*predecessor, zero);
        let Term::NatElim(motive, z, s, n) = arena.get(*recursive).unwrap() else {
            panic!("successor iota must contain the recursive eliminator");
        };
        assert_eq!((*motive, *z, *s, *n), (nat, star, var_zero, zero));
    }

    #[test]
    fn whnf_exposes_application_and_projection_principals() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let lam = arena.push(Term::Lam(var_zero)).unwrap();
        let annotated_lam = arena.push(Term::Ann(lam, nat)).unwrap();
        let application = arena.push(Term::App(annotated_lam, zero)).unwrap();

        let pair = arena.push(Term::Pair(zero, zero)).unwrap();
        let declaration = Declaration::Transparent {
            name: "pair".to_owned(),
            ty: nat,
            body: pair,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&declaration).unwrap();
        let global_pair = arena.push(Term::Global(natural("0"))).unwrap();
        let projection = arena.push(Term::Fst(global_pair)).unwrap();

        assert_eq!(whnf(&mut arena, &globals, application).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, projection).unwrap(), zero);
    }

    #[test]
    fn eliminators_expose_paths_and_scrutinees_before_deciding() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let unit = arena.push(Term::Unit).unwrap();
        let empty = arena.push(Term::Empty).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let star = arena.push(Term::Star).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();

        let identity = arena.push(Term::Id(nat, zero, zero)).unwrap();
        let refl = arena.push(Term::Refl(zero)).unwrap();
        let annotated_refl = arena.push(Term::Ann(refl, identity)).unwrap();
        let j = arena
            .push(Term::J(nat, zero, nat, star, zero, annotated_refl))
            .unwrap();

        let annotated_star = arena.push(Term::Ann(star, unit)).unwrap();
        let unit_elim = arena
            .push(Term::UnitElim(nat, zero, annotated_star))
            .unwrap();

        let annotated_zero = arena.push(Term::Ann(zero, nat)).unwrap();
        let nat_elim = arena
            .push(Term::NatElim(nat, star, var_zero, annotated_zero))
            .unwrap();

        let empty_postulate = Declaration::Postulate {
            name: "e".to_owned(),
            ty: empty,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&empty_postulate).unwrap();
        let global_empty = arena.push(Term::Global(natural("0"))).unwrap();
        let annotated_empty = arena.push(Term::Ann(global_empty, empty)).unwrap();
        let empty_elim = arena.push(Term::EmptyElim(nat, annotated_empty)).unwrap();

        assert_eq!(whnf(&mut arena, &globals, j).unwrap(), star);
        assert_eq!(whnf(&mut arena, &globals, unit_elim).unwrap(), zero);
        assert_eq!(whnf(&mut arena, &globals, nat_elim).unwrap(), star);

        let exposed_empty = whnf(&mut arena, &globals, empty_elim).unwrap();
        assert_ne!(exposed_empty, empty_elim);
        assert_eq!(
            arena.get(exposed_empty),
            Some(&Term::EmptyElim(nat, global_empty))
        );
        assert_eq!(
            whnf(&mut arena, &globals, exposed_empty).unwrap(),
            exposed_empty,
            "empty elimination is neutral only after exposing its scrutinee"
        );
    }

    #[test]
    fn opaque_and_postulate_globals_remain_neutral() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let declarations = [
            Declaration::Postulate {
                name: "p".to_owned(),
                ty: nat,
            },
            Declaration::Opaque {
                name: "o".to_owned(),
                ty: nat,
                body: zero,
            },
        ];
        let mut globals = CheckedGlobals::new();
        for declaration in &declarations {
            globals.commit(declaration).unwrap();
        }
        let postulate = arena.push(Term::Global(natural("0"))).unwrap();
        let opaque = arena.push(Term::Global(natural("1"))).unwrap();

        assert_eq!(whnf(&mut arena, &globals, postulate).unwrap(), postulate);
        assert_eq!(whnf(&mut arena, &globals, opaque).unwrap(), opaque);
    }

    #[test]
    fn expose_pi_and_sigma_only_accept_visible_type_formers() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let pi = arena.push(Term::Pi(nat, nat)).unwrap();
        let sigma = arena.push(Term::Sigma(nat, nat)).unwrap();
        let annotated_sigma = arena.push(Term::Ann(sigma, nat)).unwrap();
        let declaration = Declaration::Transparent {
            name: "P".to_owned(),
            ty: nat,
            body: pi,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&declaration).unwrap();
        let global_pi = arena.push(Term::Global(natural("0"))).unwrap();

        assert_eq!(
            expose_pi(&mut arena, &globals, global_pi).unwrap(),
            Some((nat, nat))
        );
        assert_eq!(
            expose_sigma(&mut arena, &globals, annotated_sigma).unwrap(),
            Some((nat, nat))
        );
        assert_eq!(expose_pi(&mut arena, &globals, nat).unwrap(), None);
        assert_eq!(expose_sigma(&mut arena, &globals, nat).unwrap(), None);
    }

    #[test]
    fn neutral_application_spines_are_stack_safe() {
        const DEPTH: usize = 10_000;
        let mut arena = Arena::new();
        let zero = arena.push(Term::Zero).unwrap();
        let mut root = arena.push(Term::Var(natural("0"))).unwrap();
        for _ in 0..DEPTH {
            root = arena.push(Term::App(root, zero)).unwrap();
        }
        let globals = CheckedGlobals::new();

        assert_eq!(whnf(&mut arena, &globals, root).unwrap(), root);
    }

    #[test]
    fn failed_whnf_rolls_back_derived_nodes() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let bad_global = arena.push(Term::Global(natural("0"))).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let body = arena.push(Term::Ann(bad_global, var_zero)).unwrap();
        let lam = arena.push(Term::Lam(body)).unwrap();
        let application = arena.push(Term::App(lam, zero)).unwrap();
        let checkpoint = arena.len();
        let globals = CheckedGlobals::new();

        let error = whnf(&mut arena, &globals, application).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Global));
        assert_eq!(arena.len(), checkpoint);
        assert_eq!(arena.get(nat), Some(&Term::Nat));
    }
}

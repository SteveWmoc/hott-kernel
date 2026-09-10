use super::CheckErrorClass;
use super::convert::convert;
use super::state::{CheckedGlobals, LocalContext};
use super::typecheck::synthesize_core;
use crate::syntax::{Arena, Declaration, Natural, Term, TermId};

fn natural(text: &str) -> Natural {
    Natural::from_decimal(text).expect("test natural is canonical")
}

fn commit_postulate(globals: &mut CheckedGlobals, name: &str, ty: TermId) {
    globals
        .commit(&Declaration::Postulate {
            name: name.to_owned(),
            ty,
        })
        .unwrap();
}

fn constant_motive(arena: &mut Arena, domain: TermId, result_type: TermId) -> TermId {
    let universe = arena.push(Term::Universe(natural("0"))).unwrap();
    let motive_type = arena.push(Term::Pi(domain, universe)).unwrap();
    let body = arena.push(Term::Lam(result_type)).unwrap();
    arena.push(Term::Ann(body, motive_type)).unwrap()
}

fn assert_convertible(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    actual: TermId,
    expected: TermId,
) {
    assert!(convert(arena, globals, actual, expected).unwrap());
}

#[test]
fn unary_eliminators_follow_exact_motive_protocols() {
    let mut arena = Arena::new();
    let empty = arena.push(Term::Empty).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let nat = arena.push(Term::Nat).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let zero = arena.push(Term::Zero).unwrap();

    let empty_motive = constant_motive(&mut arena, empty, nat);
    let unit_motive = constant_motive(&mut arena, unit, nat);
    let nat_motive = constant_motive(&mut arena, nat, unit);

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "absurd", empty);
    let absurd = arena.push(Term::Global(natural("0"))).unwrap();

    let empty_elim = arena.push(Term::EmptyElim(empty_motive, absurd)).unwrap();
    let unit_elim = arena.push(Term::UnitElim(unit_motive, zero, star)).unwrap();
    let step = arena.push(Term::Lam(star)).unwrap();
    let step = arena.push(Term::Lam(step)).unwrap();
    let nat_elim = arena
        .push(Term::NatElim(nat_motive, star, step, zero))
        .unwrap();

    let mut context = LocalContext::new();
    let empty_result = synthesize_core(&mut arena, &globals, &mut context, empty_elim).unwrap();
    assert_convertible(&mut arena, &globals, empty_result, nat);

    let unit_result = synthesize_core(&mut arena, &globals, &mut context, unit_elim).unwrap();
    assert_convertible(&mut arena, &globals, unit_result, nat);

    let nat_result = synthesize_core(&mut arena, &globals, &mut context, nat_elim).unwrap();
    assert_convertible(&mut arena, &globals, nat_result, unit);
    assert!(context.is_empty());
}

#[test]
fn j_recognizes_the_exact_dependent_path_domain() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let universe = arena.push(Term::Universe(natural("0"))).unwrap();
    let endpoint = arena.push(Term::Var(natural("0"))).unwrap();
    let path_domain = arena.push(Term::Id(nat, zero, endpoint)).unwrap();
    let path_family = arena.push(Term::Pi(path_domain, universe)).unwrap();
    let motive_type = arena.push(Term::Pi(nat, path_family)).unwrap();
    let motive_body = arena.push(Term::Lam(unit)).unwrap();
    let motive_body = arena.push(Term::Lam(motive_body)).unwrap();
    let motive = arena.push(Term::Ann(motive_body, motive_type)).unwrap();
    let path = arena.push(Term::Refl(zero)).unwrap();
    let term = arena
        .push(Term::J(nat, zero, motive, star, zero, path))
        .unwrap();

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    let result = synthesize_core(&mut arena, &globals, &mut context, term).unwrap();
    assert_convertible(&mut arena, &globals, result, unit);
    assert!(context.is_empty());
}

#[test]
fn j_shifts_the_base_when_recognizing_a_motive_under_an_outer_local() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let universe = arena.push(Term::Universe(natural("0"))).unwrap();
    let outer_base = arena.push(Term::Var(natural("0"))).unwrap();
    let shifted_base = arena.push(Term::Var(natural("1"))).unwrap();
    let motive_endpoint = arena.push(Term::Var(natural("0"))).unwrap();
    let path_domain = arena
        .push(Term::Id(nat, shifted_base, motive_endpoint))
        .unwrap();
    let path_family = arena.push(Term::Pi(path_domain, universe)).unwrap();
    let motive_type = arena.push(Term::Pi(nat, path_family)).unwrap();
    let motive_body = arena.push(Term::Lam(unit)).unwrap();
    let motive_body = arena.push(Term::Lam(motive_body)).unwrap();
    let motive = arena.push(Term::Ann(motive_body, motive_type)).unwrap();
    let path = arena.push(Term::Refl(outer_base)).unwrap();
    let term = arena
        .push(Term::J(nat, outer_base, motive, star, outer_base, path))
        .unwrap();

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    context.push_checked(0, nat).unwrap();
    let result = synthesize_core(&mut arena, &globals, &mut context, term).unwrap();
    assert_convertible(&mut arena, &globals, result, unit);
    assert_eq!(context.len(), 1);
}

#[test]
fn nat_step_type_shifts_a_motive_across_both_step_binders() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let universe = arena.push(Term::Universe(natural("0"))).unwrap();

    let outer_q = arena.push(Term::Var(natural("0"))).unwrap();
    let q_under_k = arena.push(Term::Var(natural("1"))).unwrap();
    let motive_result = arena.push(Term::Id(nat, q_under_k, q_under_k)).unwrap();
    let motive_type = arena.push(Term::Pi(nat, universe)).unwrap();
    let motive_body = arena.push(Term::Lam(motive_result)).unwrap();
    let motive = arena.push(Term::Ann(motive_body, motive_type)).unwrap();

    let zero_case = arena.push(Term::Refl(outer_q)).unwrap();
    let q_under_k_and_h = arena.push(Term::Var(natural("2"))).unwrap();
    let step_body = arena.push(Term::Refl(q_under_k_and_h)).unwrap();
    let step = arena.push(Term::Lam(step_body)).unwrap();
    let step = arena.push(Term::Lam(step)).unwrap();
    let term = arena
        .push(Term::NatElim(motive, zero_case, step, zero))
        .unwrap();
    let expected = arena.push(Term::Id(nat, outer_q, outer_q)).unwrap();

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    context.push_checked(0, nat).unwrap();
    let result = synthesize_core(&mut arena, &globals, &mut context, term).unwrap();
    assert_convertible(&mut arena, &globals, result, expected);
    assert_eq!(context.len(), 1);
}

#[test]
fn malformed_motives_are_rejected_transactionally() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let universe = arena.push(Term::Universe(natural("0"))).unwrap();

    let wrong_domain = constant_motive(&mut arena, unit, unit);
    let step = arena.push(Term::Lam(star)).unwrap();
    let step = arena.push(Term::Lam(step)).unwrap();
    let bad_nat = arena
        .push(Term::NatElim(wrong_domain, star, step, zero))
        .unwrap();

    let non_universe_type = arena.push(Term::Pi(nat, nat)).unwrap();
    let non_universe_body = arena.push(Term::Lam(zero)).unwrap();
    let non_universe_motive = arena
        .push(Term::Ann(non_universe_body, non_universe_type))
        .unwrap();
    let bad_codomain = arena
        .push(Term::NatElim(non_universe_motive, zero, step, zero))
        .unwrap();

    let motive_endpoint = arena.push(Term::Var(natural("0"))).unwrap();
    let wrong_path_family = arena.push(Term::Pi(unit, universe)).unwrap();
    let wrong_j_type = arena.push(Term::Pi(nat, wrong_path_family)).unwrap();
    let wrong_j_body = arena.push(Term::Lam(unit)).unwrap();
    let wrong_j_body = arena.push(Term::Lam(wrong_j_body)).unwrap();
    let wrong_j_motive = arena.push(Term::Ann(wrong_j_body, wrong_j_type)).unwrap();
    let _ = motive_endpoint;
    let path = arena.push(Term::Refl(zero)).unwrap();
    let bad_j = arena
        .push(Term::J(nat, zero, wrong_j_motive, star, zero, path))
        .unwrap();

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    let checkpoint = arena.len();
    for term in [bad_nat, bad_codomain, bad_j] {
        let error = synthesize_core(&mut arena, &globals, &mut context, term).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(arena.len(), checkpoint);
        assert!(context.is_empty());
    }
}

#[test]
fn deeply_nested_eliminators_use_the_explicit_task_stack() {
    const DEPTH: usize = 5_000;
    let mut arena = Arena::new();
    let unit = arena.push(Term::Unit).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let motive = constant_motive(&mut arena, unit, unit);
    let mut term = star;
    for _ in 0..DEPTH {
        term = arena.push(Term::UnitElim(motive, term, star)).unwrap();
    }

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    let result = synthesize_core(&mut arena, &globals, &mut context, term).unwrap();
    assert_convertible(&mut arena, &globals, result, unit);
    assert!(context.is_empty());
}

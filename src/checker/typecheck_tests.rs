use super::CheckErrorClass;
use super::state::{CheckedGlobals, LocalContext};
use super::typecheck::{check_motive_free, synthesize_motive_free};
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

fn assert_universe(arena: &Arena, term: TermId, level: &str) {
    let Term::Universe(actual) = arena.get(term).expect("universe term belongs to arena") else {
        panic!("expected a universe term");
    };
    assert_eq!(actual.as_str(), level);
}

#[test]
fn atomic_synthesis_uses_context_globals_and_unbounded_universe_arithmetic() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let local = arena.push(Term::Var(natural("0"))).unwrap();
    let huge_universe = arena
        .push(Term::Universe(natural("999999999999999999999999")))
        .unwrap();
    let global = arena.push(Term::Global(natural("0"))).unwrap();
    let empty = arena.push(Term::Empty).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let zero = arena.push(Term::Zero).unwrap();

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "u", unit);
    let mut context = LocalContext::new();
    context.push_checked(1, nat).unwrap();

    assert_eq!(
        synthesize_motive_free(&mut arena, &globals, &mut context, local).unwrap(),
        Some(nat)
    );
    assert_eq!(
        synthesize_motive_free(&mut arena, &globals, &mut context, global).unwrap(),
        Some(unit)
    );

    let universe_ty = synthesize_motive_free(&mut arena, &globals, &mut context, huge_universe)
        .unwrap()
        .unwrap();
    assert_universe(&arena, universe_ty, "1000000000000000000000000");

    let empty_ty = synthesize_motive_free(&mut arena, &globals, &mut context, empty)
        .unwrap()
        .unwrap();
    assert_universe(&arena, empty_ty, "0");

    let star_ty = synthesize_motive_free(&mut arena, &globals, &mut context, star)
        .unwrap()
        .unwrap();
    assert_eq!(arena.get(star_ty), Some(&Term::Unit));

    let zero_ty = synthesize_motive_free(&mut arena, &globals, &mut context, zero)
        .unwrap()
        .unwrap();
    assert_eq!(arena.get(zero_ty), Some(&Term::Nat));
    assert_eq!(context.len(), 1);
}

#[test]
fn products_application_and_projections_preserve_dependent_substitution() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
    let dependent = arena.push(Term::Id(nat, var_zero, var_zero)).unwrap();
    let function_type = arena.push(Term::Pi(nat, dependent)).unwrap();
    let pair_type = arena.push(Term::Sigma(nat, dependent)).unwrap();

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "f", function_type);
    commit_postulate(&mut globals, "p", pair_type);
    let function = arena.push(Term::Global(natural("0"))).unwrap();
    let pair = arena.push(Term::Global(natural("1"))).unwrap();
    let app = arena.push(Term::App(function, zero)).unwrap();
    let fst = arena.push(Term::Fst(pair)).unwrap();
    let snd = arena.push(Term::Snd(pair)).unwrap();
    let mut context = LocalContext::new();

    let function_universe =
        synthesize_motive_free(&mut arena, &globals, &mut context, function_type)
            .unwrap()
            .unwrap();
    assert_universe(&arena, function_universe, "0");
    let pair_universe = synthesize_motive_free(&mut arena, &globals, &mut context, pair_type)
        .unwrap()
        .unwrap();
    assert_universe(&arena, pair_universe, "0");

    let app_type = synthesize_motive_free(&mut arena, &globals, &mut context, app)
        .unwrap()
        .unwrap();
    let Term::Id(app_domain, app_left, app_right) = arena.get(app_type).unwrap() else {
        panic!("dependent application must synthesize an identity type");
    };
    assert_eq!((*app_domain, *app_left, *app_right), (nat, zero, zero));

    let fst_type = synthesize_motive_free(&mut arena, &globals, &mut context, fst)
        .unwrap()
        .unwrap();
    assert_eq!(fst_type, nat);

    let snd_type = synthesize_motive_free(&mut arena, &globals, &mut context, snd)
        .unwrap()
        .unwrap();
    let Term::Id(snd_domain, snd_left, snd_right) = arena.get(snd_type).unwrap() else {
        panic!("dependent second projection must synthesize an identity type");
    };
    assert_eq!(*snd_domain, nat);
    assert_eq!(snd_left, snd_right);
    assert!(matches!(arena.get(*snd_left), Some(Term::Fst(principal)) if *principal == pair));
}

#[test]
fn identity_refl_successor_and_annotation_follow_frozen_rules() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let identity = arena.push(Term::Id(nat, zero, zero)).unwrap();
    let refl = arena.push(Term::Refl(zero)).unwrap();
    let succ = arena.push(Term::Succ(zero)).unwrap();
    let annotation = arena.push(Term::Ann(zero, nat)).unwrap();
    let bad_annotation = arena.push(Term::Ann(star, nat)).unwrap();
    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();

    let identity_type = synthesize_motive_free(&mut arena, &globals, &mut context, identity)
        .unwrap()
        .unwrap();
    assert_universe(&arena, identity_type, "0");

    let refl_type = synthesize_motive_free(&mut arena, &globals, &mut context, refl)
        .unwrap()
        .unwrap();
    let Term::Id(refl_domain, refl_left, refl_right) = arena.get(refl_type).unwrap() else {
        panic!("reflexivity must synthesize an identity type");
    };
    assert!(matches!(arena.get(*refl_domain), Some(Term::Nat)));
    assert_eq!((*refl_left, *refl_right), (zero, zero));

    let succ_type = synthesize_motive_free(&mut arena, &globals, &mut context, succ)
        .unwrap()
        .unwrap();
    assert_eq!(arena.get(succ_type), Some(&Term::Nat));

    assert_eq!(
        synthesize_motive_free(&mut arena, &globals, &mut context, annotation).unwrap(),
        Some(nat)
    );
    let error =
        synthesize_motive_free(&mut arena, &globals, &mut context, bad_annotation).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.term_id(), Some(star));
    assert_eq!(error.reference_kind(), None);
}

#[test]
fn lambda_and_pair_checking_use_exposure_substitution_and_conversion() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
    let function_type = arena.push(Term::Pi(nat, nat)).unwrap();
    let lambda = arena.push(Term::Lam(var_zero)).unwrap();

    let dependent = arena.push(Term::Id(nat, var_zero, var_zero)).unwrap();
    let pair_type = arena.push(Term::Sigma(nat, dependent)).unwrap();
    let refl_zero = arena.push(Term::Refl(zero)).unwrap();
    let pair = arena.push(Term::Pair(zero, refl_zero)).unwrap();
    let bad_pair = arena.push(Term::Pair(zero, star)).unwrap();

    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    let checkpoint = arena.len();

    assert_eq!(
        check_motive_free(&mut arena, &globals, &mut context, lambda, function_type).unwrap(),
        Some(())
    );
    assert_eq!(
        arena.len(),
        checkpoint,
        "successful checking is arena-neutral"
    );
    assert!(context.is_empty());

    assert_eq!(
        check_motive_free(&mut arena, &globals, &mut context, pair, pair_type).unwrap(),
        Some(())
    );
    assert_eq!(arena.len(), checkpoint);

    let error =
        check_motive_free(&mut arena, &globals, &mut context, bad_pair, pair_type).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.term_id(), Some(star));
    assert!(context.is_empty());
    assert_eq!(arena.len(), checkpoint);

    for bare in [lambda, pair] {
        let error = synthesize_motive_free(&mut arena, &globals, &mut context, bare).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.term_id(), Some(bare));
    }
}

#[test]
fn transparent_type_aliases_are_exposed_but_noncumulativity_is_not_relaxed() {
    let mut arena = Arena::new();
    let universe_zero = arena.push(Term::Universe(natural("0"))).unwrap();
    let universe_one = arena.push(Term::Universe(natural("1"))).unwrap();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let function_type = arena.push(Term::Pi(nat, nat)).unwrap();
    let lambda = arena.push(Term::Lam(zero)).unwrap();

    let alias = Declaration::Transparent {
        name: "F".to_owned(),
        ty: universe_zero,
        body: function_type,
    };
    let mut globals = CheckedGlobals::new();
    globals.commit(&alias).unwrap();
    let alias_ref = arena.push(Term::Global(natural("0"))).unwrap();
    commit_postulate(&mut globals, "f", alias_ref);
    let function = arena.push(Term::Global(natural("1"))).unwrap();
    let app = arena.push(Term::App(function, zero)).unwrap();
    let mut context = LocalContext::new();

    assert_eq!(
        check_motive_free(&mut arena, &globals, &mut context, lambda, alias_ref).unwrap(),
        Some(())
    );
    let app_type = synthesize_motive_free(&mut arena, &globals, &mut context, app)
        .unwrap()
        .unwrap();
    assert_eq!(app_type, nat);

    let error =
        check_motive_free(&mut arena, &globals, &mut context, nat, universe_one).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.term_id(), Some(nat));
}

#[test]
fn malformed_motive_eliminators_fail_without_mutating_state() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let j = arena
        .push(Term::J(nat, zero, nat, zero, zero, zero))
        .unwrap();
    let product = arena.push(Term::Pi(nat, j)).unwrap();
    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();
    let checkpoint = arena.len();

    let error = synthesize_motive_free(&mut arena, &globals, &mut context, product).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(arena.len(), checkpoint);
    assert!(context.is_empty());

    let lambda = arena.push(Term::Lam(j)).unwrap();
    let expected = arena.push(Term::Pi(nat, nat)).unwrap();
    let checkpoint = arena.len();
    let error =
        check_motive_free(&mut arena, &globals, &mut context, lambda, expected).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(arena.len(), checkpoint);
    assert!(context.is_empty());
}

#[test]
fn deep_product_synthesis_and_lambda_checking_are_stack_safe() {
    const DEPTH: usize = 10_000;
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let mut expected = nat;
    let mut value = zero;
    for _ in 0..DEPTH {
        expected = arena.push(Term::Pi(nat, expected)).unwrap();
        value = arena.push(Term::Lam(value)).unwrap();
    }
    let globals = CheckedGlobals::new();
    let mut context = LocalContext::new();

    let synthesized = synthesize_motive_free(&mut arena, &globals, &mut context, expected)
        .unwrap()
        .unwrap();
    assert_universe(&arena, synthesized, "0");
    assert!(context.is_empty());

    let checkpoint = arena.len();
    assert_eq!(
        check_motive_free(&mut arena, &globals, &mut context, value, expected).unwrap(),
        Some(())
    );
    assert_eq!(arena.len(), checkpoint);
    assert!(context.is_empty());
}

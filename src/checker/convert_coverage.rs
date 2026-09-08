use super::convert::{convert, normalize};
use super::state::CheckedGlobals;
use crate::syntax::{Arena, Declaration, Natural, Term};

fn natural(text: &str) -> Natural {
    Natural::from_decimal(text).expect("test natural is canonical")
}

fn commit_postulate(globals: &mut CheckedGlobals, name: &str, ty: crate::syntax::TermId) {
    globals
        .commit(&Declaration::Postulate {
            name: name.to_owned(),
            ty,
        })
        .unwrap();
}

#[test]
fn normalization_rebuilds_remaining_composite_shapes() {
    let mut arena = Arena::new();
    let universe_zero = arena.push(Term::Universe(natural("0"))).unwrap();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let empty = arena.push(Term::Empty).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let succ_zero = arena.push(Term::Succ(zero)).unwrap();

    let annotated_nat = arena.push(Term::Ann(nat, universe_zero)).unwrap();
    let annotated_unit = arena.push(Term::Ann(unit, universe_zero)).unwrap();
    let annotated_zero = arena.push(Term::Ann(zero, nat)).unwrap();
    let annotated_star = arena.push(Term::Ann(star, unit)).unwrap();
    let annotated_succ = arena.push(Term::Ann(succ_zero, nat)).unwrap();

    let sigma_type = arena.push(Term::Sigma(nat, nat)).unwrap();
    let function_type = arena.push(Term::Pi(nat, sigma_type)).unwrap();
    let path_type = arena.push(Term::Id(nat, zero, zero)).unwrap();

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "f", function_type);
    commit_postulate(&mut globals, "e", empty);
    commit_postulate(&mut globals, "u", unit);
    commit_postulate(&mut globals, "p", path_type);

    let function = arena.push(Term::Global(natural("0"))).unwrap();
    let empty_neutral = arena.push(Term::Global(natural("1"))).unwrap();
    let unit_neutral = arena.push(Term::Global(natural("2"))).unwrap();
    let path_neutral = arena.push(Term::Global(natural("3"))).unwrap();
    let neutral_pair = arena.push(Term::App(function, annotated_zero)).unwrap();

    let sigma = arena
        .push(Term::Sigma(annotated_nat, annotated_nat))
        .unwrap();
    let refl = arena.push(Term::Refl(annotated_zero)).unwrap();
    let fst = arena.push(Term::Fst(neutral_pair)).unwrap();
    let snd = arena.push(Term::Snd(neutral_pair)).unwrap();
    let empty_elim = arena
        .push(Term::EmptyElim(annotated_nat, empty_neutral))
        .unwrap();
    let unit_elim = arena
        .push(Term::UnitElim(annotated_nat, annotated_zero, unit_neutral))
        .unwrap();
    let j = arena
        .push(Term::J(
            annotated_nat,
            annotated_zero,
            annotated_unit,
            annotated_star,
            annotated_succ,
            path_neutral,
        ))
        .unwrap();

    let normalized_sigma = normalize(&mut arena, &globals, sigma).unwrap();
    assert_eq!(arena.get(normalized_sigma), Some(&Term::Sigma(nat, nat)));

    let normalized_refl = normalize(&mut arena, &globals, refl).unwrap();
    assert_eq!(arena.get(normalized_refl), Some(&Term::Refl(zero)));

    for (projection, is_fst) in [(fst, true), (snd, false)] {
        let normalized = normalize(&mut arena, &globals, projection).unwrap();
        let principal = match arena.get(normalized).unwrap() {
            Term::Fst(principal) if is_fst => *principal,
            Term::Snd(principal) if !is_fst => *principal,
            other => panic!("projection constructor changed during normalization: {other:?}"),
        };
        assert_eq!(arena.get(principal), Some(&Term::App(function, zero)));
    }

    let normalized_empty = normalize(&mut arena, &globals, empty_elim).unwrap();
    assert_eq!(
        arena.get(normalized_empty),
        Some(&Term::EmptyElim(nat, empty_neutral))
    );

    let normalized_unit = normalize(&mut arena, &globals, unit_elim).unwrap();
    assert_eq!(
        arena.get(normalized_unit),
        Some(&Term::UnitElim(nat, zero, unit_neutral))
    );

    let normalized_j = normalize(&mut arena, &globals, j).unwrap();
    assert_eq!(
        arena.get(normalized_j),
        Some(&Term::J(nat, zero, unit, star, succ_zero, path_neutral,))
    );
}

#[test]
fn conversion_exercises_remaining_structural_arms_and_all_j_children() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let unit = arena.push(Term::Unit).unwrap();
    let empty = arena.push(Term::Empty).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let star = arena.push(Term::Star).unwrap();
    let succ_zero = arena.push(Term::Succ(zero)).unwrap();
    let sigma_type = arena.push(Term::Sigma(nat, nat)).unwrap();
    let path_type = arena.push(Term::Id(nat, zero, zero)).unwrap();

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "pair", sigma_type);
    commit_postulate(&mut globals, "e", empty);
    commit_postulate(&mut globals, "u", unit);
    commit_postulate(&mut globals, "p", path_type);
    commit_postulate(&mut globals, "q", path_type);

    let pair_neutral = arena.push(Term::Global(natural("0"))).unwrap();
    let empty_neutral = arena.push(Term::Global(natural("1"))).unwrap();
    let unit_neutral = arena.push(Term::Global(natural("2"))).unwrap();
    let path_p = arena.push(Term::Global(natural("3"))).unwrap();
    let path_q = arena.push(Term::Global(natural("4"))).unwrap();

    let sigma_left = arena.push(Term::Sigma(nat, nat)).unwrap();
    let sigma_right = arena.push(Term::Sigma(nat, nat)).unwrap();
    let refl_left = arena.push(Term::Refl(zero)).unwrap();
    let refl_right = arena.push(Term::Refl(zero)).unwrap();
    let fst_left = arena.push(Term::Fst(pair_neutral)).unwrap();
    let fst_right = arena.push(Term::Fst(pair_neutral)).unwrap();
    let snd_left = arena.push(Term::Snd(pair_neutral)).unwrap();
    let snd_right = arena.push(Term::Snd(pair_neutral)).unwrap();
    let empty_left = arena.push(Term::EmptyElim(nat, empty_neutral)).unwrap();
    let empty_right = arena.push(Term::EmptyElim(nat, empty_neutral)).unwrap();
    let unit_left = arena.push(Term::UnitElim(nat, zero, unit_neutral)).unwrap();
    let unit_right = arena.push(Term::UnitElim(nat, zero, unit_neutral)).unwrap();

    let j_left = arena
        .push(Term::J(nat, zero, unit, star, succ_zero, path_p))
        .unwrap();
    let j_equal = arena
        .push(Term::J(nat, zero, unit, star, succ_zero, path_p))
        .unwrap();
    let j_variants = [
        arena
            .push(Term::J(unit, zero, unit, star, succ_zero, path_p))
            .unwrap(),
        arena
            .push(Term::J(nat, star, unit, star, succ_zero, path_p))
            .unwrap(),
        arena
            .push(Term::J(nat, zero, nat, star, succ_zero, path_p))
            .unwrap(),
        arena
            .push(Term::J(nat, zero, unit, zero, succ_zero, path_p))
            .unwrap(),
        arena
            .push(Term::J(nat, zero, unit, star, zero, path_p))
            .unwrap(),
        arena
            .push(Term::J(nat, zero, unit, star, succ_zero, path_q))
            .unwrap(),
    ];

    let checkpoint = arena.len();
    for (left, right) in [
        (sigma_left, sigma_right),
        (refl_left, refl_right),
        (fst_left, fst_right),
        (snd_left, snd_right),
        (empty_left, empty_right),
        (unit_left, unit_right),
        (j_left, j_equal),
    ] {
        assert!(convert(&mut arena, &globals, left, right).unwrap());
        assert_eq!(arena.len(), checkpoint);
    }

    for variant in j_variants {
        assert!(!convert(&mut arena, &globals, j_left, variant).unwrap());
        assert_eq!(arena.len(), checkpoint);
    }
}

#[test]
fn conversion_distinguishes_atomic_payloads() {
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let universe_zero = arena.push(Term::Universe(natural("0"))).unwrap();
    let universe_one = arena.push(Term::Universe(natural("1"))).unwrap();
    let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
    let var_one = arena.push(Term::Var(natural("1"))).unwrap();

    let mut globals = CheckedGlobals::new();
    commit_postulate(&mut globals, "a", nat);
    commit_postulate(&mut globals, "b", nat);
    let global_zero = arena.push(Term::Global(natural("0"))).unwrap();
    let global_one = arena.push(Term::Global(natural("1"))).unwrap();
    let checkpoint = arena.len();

    // The variable pair represents syntax already validated under a local
    // context of depth at least two; conversion itself intentionally carries
    // no local-context parameter.
    for (left, right) in [
        (var_zero, var_one),
        (global_zero, global_one),
        (universe_zero, universe_one),
    ] {
        assert!(!convert(&mut arena, &globals, left, right).unwrap());
        assert_eq!(arena.len(), checkpoint);
    }
}

#[test]
fn normalization_and_conversion_are_stack_safe_through_deep_head_exposure() {
    const DEPTH: usize = 10_000;
    let mut arena = Arena::new();
    let nat = arena.push(Term::Nat).unwrap();
    let zero = arena.push(Term::Zero).unwrap();
    let mut annotated = zero;
    for _ in 0..DEPTH {
        annotated = arena.push(Term::Ann(annotated, nat)).unwrap();
    }
    let globals = CheckedGlobals::new();

    // Unlike the deep Succ regression, this forces WHNF itself to erase ten
    // thousand nested head annotations before normalization can see Zero.
    assert_eq!(normalize(&mut arena, &globals, annotated).unwrap(), zero);
    let checkpoint = arena.len();
    assert!(convert(&mut arena, &globals, annotated, zero).unwrap());
    assert_eq!(arena.len(), checkpoint);
}

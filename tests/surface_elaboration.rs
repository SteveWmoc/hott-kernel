use hott_kernel::{SurfaceErrorClass, Term, elaborate_surface, parse_surface, print_canonical};

fn elaborate(source: &[u8]) -> hott_kernel::Module {
    let surface = parse_surface(source).expect("surface source must parse");
    elaborate_surface(&surface).expect("surface source must elaborate")
}

#[test]
fn resolves_locals_and_earlier_globals_exactly() {
    let core = elaborate(
        br#"
(surface 0 1)
(module Basics)
(postulate A (universe 0))
(transparent identity_map
  (pi x (ref A) (ref A))
  (lam x (ref x)))
"#,
    );

    assert_eq!(
        print_canonical(&core).unwrap(),
        b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"A\" (universe 0)) (transparent \"identity_map\" (pi (global 0) (global 0)) (lam (var 0)))))\n"
    );
}

#[test]
fn local_binders_shadow_globals_and_nearest_local_wins() {
    let core = elaborate(
        br#"
(surface 0 1)
(module Shadow)
(postulate A (universe 0))
(transparent choose_outer
  (pi A (universe 0)
    (pi x (ref A) (ref A)))
  (lam A
    (lam x
      (ref A))))
"#,
    );

    assert_eq!(
        print_canonical(&core).unwrap(),
        b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"A\" (universe 0)) (transparent \"choose_outer\" (pi (universe 0) (pi (var 0) (var 1))) (lam (lam (var 1))))))\n"
    );

    let nearest = elaborate(
        br#"
(surface 0 1)
(module Nearest)
(postulate p
  (pi x nat
    (pi x nat
      (ref x))))
"#,
    );
    assert_eq!(
        print_canonical(&nearest).unwrap(),
        b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"p\" (pi nat (pi nat (var 0))))))\n"
    );
}

#[test]
fn global_indices_follow_successful_declaration_order() {
    let core = elaborate(
        br#"
(surface 0 1)
(module Globals)
(postulate A (universe 0))
(postulate B (ref A))
(postulate C (ref B))
"#,
    );

    assert_eq!(
        print_canonical(&core).unwrap(),
        b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"A\" (universe 0)) (postulate \"B\" (global 0)) (postulate \"C\" (global 1))))\n"
    );
}

#[test]
fn current_declaration_and_forward_declarations_are_unavailable() {
    for source in [
        br#"
(surface 0 1)
(module SelfRef)
(transparent loop nat (ref loop))
"#
        .as_slice(),
        br#"
(surface 0 1)
(module Forward)
(postulate first (ref later))
(postulate later (universe 0))
"#
        .as_slice(),
    ] {
        let surface = parse_surface(source).unwrap();
        assert_eq!(
            elaborate_surface(&surface).unwrap_err().class(),
            SurfaceErrorClass::UnknownName
        );
    }
}

#[test]
fn rejects_duplicate_global_declaration_names() {
    let surface = parse_surface(
        br#"
(surface 0 1)
(module Duplicate)
(postulate A (universe 0))
(postulate A unit)
"#,
    )
    .unwrap();

    let error = elaborate_surface(&surface).unwrap_err();
    assert_eq!(
        error.class(),
        SurfaceErrorClass::DuplicateGlobalDeclarationName
    );
}

#[test]
fn unknown_name_is_distinct_from_parse_failure() {
    let surface = parse_surface(b"(surface 0 1) (module M) (postulate p (ref missing))")
        .expect("unresolved references are valid parser output");

    let error = elaborate_surface(&surface).unwrap_err();
    assert_eq!(error.class(), SurfaceErrorClass::UnknownName);
}

#[test]
fn translates_every_surface_term_constructor() {
    let core = elaborate(
        br#"
(surface 0 1)
(module Forms)
(postulate g nat)
(postulate t_ref (ref g))
(postulate t_universe (universe 2))
(postulate t_pi (pi x nat (ref x)))
(postulate t_lam (lam x (ref x)))
(postulate t_app (app zero zero))
(postulate t_sigma (sigma x nat (ref x)))
(postulate t_pair (pair zero zero))
(postulate t_fst (fst zero))
(postulate t_snd (snd zero))
(postulate t_id (id nat zero zero))
(postulate t_refl (refl zero))
(postulate t_j (j nat zero nat zero zero zero))
(postulate t_empty empty)
(postulate t_empty_elim (empty-elim nat zero))
(postulate t_unit unit)
(postulate t_star star)
(postulate t_unit_elim (unit-elim nat zero star))
(postulate t_nat nat)
(postulate t_zero zero)
(postulate t_succ (succ zero))
(postulate t_nat_elim (nat-elim nat zero zero zero))
(postulate t_ann (ann zero nat))
"#,
    );

    let declarations = core.declarations();
    assert_eq!(declarations.len(), 23);

    let expected = [
        "nat",
        "global",
        "universe",
        "pi",
        "lam",
        "app",
        "sigma",
        "pair",
        "fst",
        "snd",
        "id",
        "refl",
        "j",
        "empty",
        "empty-elim",
        "unit",
        "star",
        "unit-elim",
        "nat",
        "zero",
        "succ",
        "nat-elim",
        "ann",
    ];

    for (declaration, expected) in declarations.iter().zip(expected) {
        let term = core.arena().get(declaration.ty()).unwrap();
        let actual = match term {
            Term::Var(_) => "var",
            Term::Global(_) => "global",
            Term::Universe(_) => "universe",
            Term::Pi(_, _) => "pi",
            Term::Lam(_) => "lam",
            Term::App(_, _) => "app",
            Term::Sigma(_, _) => "sigma",
            Term::Pair(_, _) => "pair",
            Term::Fst(_) => "fst",
            Term::Snd(_) => "snd",
            Term::Id(_, _, _) => "id",
            Term::Refl(_) => "refl",
            Term::J(_, _, _, _, _, _) => "j",
            Term::Empty => "empty",
            Term::EmptyElim(_, _) => "empty-elim",
            Term::Unit => "unit",
            Term::Star => "star",
            Term::UnitElim(_, _, _) => "unit-elim",
            Term::Nat => "nat",
            Term::Zero => "zero",
            Term::Succ(_) => "succ",
            Term::NatElim(_, _, _, _) => "nat-elim",
            Term::Ann(_, _) => "ann",
        };
        assert_eq!(actual, expected, "declaration {}", declaration.name());
    }
}

#[test]
fn module_and_binder_names_do_not_survive_core_translation() {
    let first = elaborate(b"(surface 0 1) (module First) (postulate p (pi x nat (ref x)))");
    let second = elaborate(b"(surface 0 1) (module Second) (postulate p (pi y nat (ref y)))");

    assert_eq!(
        print_canonical(&first).unwrap(),
        print_canonical(&second).unwrap()
    );
}

#[test]
fn deep_binding_translation_does_not_use_the_rust_call_stack() {
    const DEPTH: usize = 10_000;

    let mut source = String::from("(surface 0 1) (module Deep) (postulate p ");
    for index in 0..DEPTH {
        source.push_str("(pi x");
        source.push_str(&index.to_string());
        source.push_str(" nat ");
    }
    source.push_str("(ref x0)");
    for _ in 0..DEPTH {
        source.push(')');
    }
    source.push(')');

    let core = elaborate(source.as_bytes());
    let mut id = core.declarations()[0].ty();

    for _ in 0..DEPTH {
        let Term::Pi(_, codomain) = core.arena().get(id).unwrap() else {
            panic!("expected nested pi");
        };
        id = *codomain;
    }

    let Term::Var(index) = core.arena().get(id).unwrap() else {
        panic!("deep reference should resolve to a local variable");
    };
    assert_eq!(index.as_str(), (DEPTH - 1).to_string());
}

#[test]
fn elaboration_does_not_run_the_core_checker() {
    let core = elaborate(b"(surface 0 1) (module InvalidButExplicit) (transparent bad nat unit)");
    assert_eq!(core.declarations().len(), 1);
}

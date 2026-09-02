use hott_kernel::{CheckErrorClass, ReferenceKind, check_references, parse_canonical};

const ACCEPTED: &[&[u8]] = &[
    include_bytes!("conformance/accepted/annotation-erasure.core"),
    include_bytes!("conformance/accepted/beta.core"),
    include_bytes!("conformance/accepted/declaration-kinds.core"),
    include_bytes!("conformance/accepted/dependent-lookup.core"),
    include_bytes!("conformance/accepted/empty-neutral.core"),
    include_bytes!("conformance/accepted/j-refl.core"),
    include_bytes!("conformance/accepted/mixed-universes.core"),
    include_bytes!("conformance/accepted/nat-elim-zero.core"),
    include_bytes!("conformance/accepted/nat-elim-succ.core"),
    include_bytes!("conformance/accepted/nested-substitution.core"),
    include_bytes!("conformance/accepted/pair-projections.core"),
    include_bytes!("conformance/accepted/transparent-delta.core"),
    include_bytes!("conformance/accepted/unit-elim-star.core"),
];

#[test]
fn accepted_conformance_modules_have_available_references() {
    for input in ACCEPTED {
        let module = parse_canonical(input).expect("accepted fixture must parse");
        check_references(&module).expect("accepted fixture references must be available");
    }
}

#[test]
fn typing_rejections_can_still_have_available_references() {
    for input in [
        include_bytes!("conformance/rejected/bad-body.core").as_slice(),
        include_bytes!("conformance/rejected/false-refl.core").as_slice(),
        include_bytes!("conformance/rejected/no-uip.core").as_slice(),
        include_bytes!("conformance/rejected/universe-noncumulative.core").as_slice(),
    ] {
        let module = parse_canonical(input).expect("typing rejection must parse");
        check_references(&module).expect("typing belongs to a later checker layer");
    }
}

#[test]
fn out_of_scope_local_reference_is_an_invalid_judgment() {
    let module = parse_canonical(include_bytes!(
        "conformance/rejected/out-of-scope-variable.core"
    ))
    .unwrap();
    let error = check_references(&module).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.declaration_index(), 0);
    assert_eq!(error.reference_kind(), Some(ReferenceKind::Local));
    assert!(error.term_id().is_some());
}

#[test]
fn forward_self_and_out_of_range_globals_are_invalid_judgments() {
    let self_reference = canonical("(postulate \"self\" (global 0))");
    let out_of_range = canonical("(postulate \"first\" unit) (postulate \"too-far\" (global 2))");
    let cases = [
        (
            0,
            include_bytes!("conformance/rejected/forward-reference.core").as_slice(),
        ),
        (0, self_reference.as_bytes()),
        (1, out_of_range.as_bytes()),
    ];

    for (declaration_index, input) in cases {
        let module = parse_canonical(input).unwrap();
        let error = check_references(&module).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.declaration_index(), declaration_index);
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Global));
    }
}

#[test]
fn only_pi_sigma_and_lam_extend_local_scope() {
    let valid = canonical(
        "(postulate \"binders\" (pi unit (sigma (var 0) (lam (pair (var 2) (pair (var 1) (var 0)))))))",
    );
    let module = parse_canonical(valid.as_bytes()).unwrap();
    check_references(&module).unwrap();

    for declaration in [
        "(postulate \"pi-domain-does-not-bind\" (pi (var 0) unit))",
        "(postulate \"sigma-domain-does-not-bind\" (sigma (var 0) unit))",
        "(postulate \"application-does-not-bind\" (app unit (var 0)))",
    ] {
        let invalid = canonical(declaration);
        let module = parse_canonical(invalid.as_bytes()).unwrap();
        let error = check_references(&module).unwrap_err();
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Local));
    }
}

#[test]
fn declaration_types_and_bodies_each_start_with_empty_local_scope() {
    let cases = [
        (
            0,
            canonical("(transparent \"body-is-closed\" (pi unit (var 0)) (var 0))"),
        ),
        (
            1,
            canonical("(postulate \"first\" (pi unit (var 0))) (postulate \"second\" (var 0))"),
        ),
    ];

    for (declaration_index, input) in cases {
        let module = parse_canonical(input.as_bytes()).unwrap();
        let error = check_references(&module).unwrap_err();
        assert_eq!(error.declaration_index(), declaration_index);
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Local));
    }
}

#[test]
fn enormous_unrepresentable_indices_are_out_of_scope() {
    let index = "9".repeat(512);
    let input = canonical(&format!("(postulate \"huge\" (lam (var {index})))"));
    let module = parse_canonical(input.as_bytes()).unwrap();
    let error = check_references(&module).unwrap_err();
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.reference_kind(), Some(ReferenceKind::Local));
}

#[test]
fn deeply_nested_scope_validation_does_not_use_the_rust_call_stack() {
    const DEPTH: usize = 10_000;
    let mut declaration = String::from("(postulate \"deep\" ");
    for _ in 0..DEPTH {
        declaration.push_str("(lam ");
    }
    declaration.push_str("(var 9999)");
    for _ in 0..DEPTH {
        declaration.push(')');
    }
    declaration.push(')');

    let input = canonical(&declaration);
    let module = parse_canonical(input.as_bytes()).unwrap();
    check_references(&module).unwrap();
}

fn canonical(declarations: &str) -> String {
    format!("(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations {declarations}))\n")
}

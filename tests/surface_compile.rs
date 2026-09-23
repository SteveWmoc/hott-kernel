use hott_kernel::{
    SurfaceCompileError, SurfaceErrorClass, check_module, compile_surface, parse_canonical,
    print_semantic,
};

const FIXTURES: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("surface/empty.surface"),
        include_bytes!("surface/empty.core"),
    ),
    (
        include_bytes!("surface/identity.surface"),
        include_bytes!("surface/identity.core"),
    ),
    (
        include_bytes!("surface/shadow.surface"),
        include_bytes!("surface/shadow.core"),
    ),
    (
        include_bytes!("surface/invalid-but-explicit.surface"),
        include_bytes!("surface/invalid-but-explicit.core"),
    ),
];

#[test]
fn byte_level_fixtures_compile_exactly() {
    for (source, expected) in FIXTURES {
        let actual = compile_surface(source).expect("fixture must compile");
        assert_eq!(actual.as_slice(), *expected);

        let reparsed = parse_canonical(&actual).expect("compiler output must be canonical Core");
        assert_eq!(
            hott_kernel::print_canonical(&reparsed).unwrap().as_slice(),
            *expected
        );
    }
}

#[test]
fn comments_layout_module_names_and_binder_names_do_not_affect_core_bytes() {
    let first = br#"
; first spelling
(surface 0 1)
(module First)
(postulate p
  (pi x nat
    (ref x)))
"#;
    let second = br#"(surface 0 1);header
(module Second)
(postulate p (pi renamed nat (ref renamed)))"#;

    assert_eq!(
        compile_surface(first).unwrap(),
        compile_surface(second).unwrap()
    );
}

#[test]
fn consistent_global_rename_changes_artifact_but_not_semantic_projection() {
    let first = br#"
(surface 0 1)
(module M)
(postulate A (universe 0))
(postulate use_A (ref A))
"#;
    let second = br#"
(surface 0 1)
(module M)
(postulate B (universe 0))
(postulate use_B (ref B))
"#;

    let first_bytes = compile_surface(first).unwrap();
    let second_bytes = compile_surface(second).unwrap();
    assert_ne!(first_bytes, second_bytes);

    let first_core = parse_canonical(&first_bytes).unwrap();
    let second_core = parse_canonical(&second_bytes).unwrap();
    assert_eq!(
        print_semantic(&first_core).unwrap(),
        print_semantic(&second_core).unwrap()
    );
}

#[test]
fn compiler_preserves_surface_error_classes() {
    let cases: &[(&[u8], SurfaceErrorClass)] = &[
        (
            b"(surface 0 1) (module M) (postulate p (succ))",
            SurfaceErrorClass::MalformedSyntax,
        ),
        (
            b"(surface 0 2) (module M)",
            SurfaceErrorClass::UnsupportedVersion,
        ),
        (
            b"(surface 0 1) (module M) (postulate id nat)",
            SurfaceErrorClass::ReservedIdentifier,
        ),
        (
            b"(surface 0 1) (module M) (postulate p (ref missing))",
            SurfaceErrorClass::UnknownName,
        ),
        (
            b"(surface 0 1) (module M) (postulate p nat) (postulate p unit)",
            SurfaceErrorClass::DuplicateGlobalDeclarationName,
        ),
    ];

    for (source, expected) in cases {
        let error = compile_surface(source).unwrap_err();
        let SurfaceCompileError::Surface(error) = error else {
            panic!("surface failure must remain a Surface error");
        };
        assert_eq!(error.class(), *expected);
    }
}

#[test]
fn logically_invalid_surface_still_compiles_without_checker_authority() {
    let bytes = compile_surface(include_bytes!("surface/invalid-but-explicit.surface"))
        .expect("structurally explicit source must compile");

    let mut core = parse_canonical(&bytes).expect("compiler output must be canonical");
    assert!(
        check_module(&mut core).is_err(),
        "logical rejection belongs to the Core checker, not the Surface compiler"
    );
}

#[test]
fn empty_surface_module_compiles_to_the_frozen_empty_core_artifact() {
    assert_eq!(
        compile_surface(include_bytes!("surface/empty.surface")).unwrap(),
        include_bytes!("format/canonical/empty.core")
    );
}

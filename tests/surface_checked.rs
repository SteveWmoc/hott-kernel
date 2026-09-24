use hott_kernel::{
    CheckErrorClass, SurfaceCheckError, SurfaceErrorClass, compile_and_check_surface,
    compile_surface,
};

#[test]
fn accepted_surface_emits_the_same_bytes_as_unchecked_compilation() {
    for source in [
        include_bytes!("surface/empty.surface").as_slice(),
        include_bytes!("surface/identity.surface").as_slice(),
        include_bytes!("surface/shadow.surface").as_slice(),
    ] {
        assert_eq!(
            compile_and_check_surface(source).expect("valid Surface must check"),
            compile_surface(source).expect("valid Surface must compile")
        );
    }
}

#[test]
fn accepted_surface_emits_canonical_fixture_bytes() {
    assert_eq!(
        compile_and_check_surface(include_bytes!("surface/identity.surface")).unwrap(),
        include_bytes!("surface/identity.core")
    );
}

#[test]
fn logical_rejection_remains_a_checker_error() {
    let error = compile_and_check_surface(include_bytes!("surface/invalid-but-explicit.surface"))
        .expect_err("invalid Core judgment must be rejected by checker");

    let SurfaceCheckError::Check(error) = error else {
        panic!("logical rejection must remain a checker error");
    };
    assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
    assert_eq!(error.declaration_index(), 0);
}

#[test]
fn surface_failures_remain_surface_errors() {
    let cases: &[(&[u8], SurfaceErrorClass)] = &[
        (
            b"(surface 0 2) (module M)",
            SurfaceErrorClass::UnsupportedVersion,
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
        let error = compile_and_check_surface(source).unwrap_err();
        let SurfaceCheckError::Surface(error) = error else {
            panic!("Surface rejection must remain a Surface error");
        };
        assert_eq!(error.class(), *expected);
    }
}

#[test]
fn unchecked_compiler_still_accepts_logically_invalid_surface() {
    assert!(compile_surface(include_bytes!("surface/invalid-but-explicit.surface")).is_ok());
    assert!(matches!(
        compile_and_check_surface(include_bytes!("surface/invalid-but-explicit.surface")),
        Err(SurfaceCheckError::Check(_))
    ));
}

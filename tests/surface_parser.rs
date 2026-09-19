use hott_kernel::{
    SURFACE_FORMAT, SurfaceDeclaration, SurfaceErrorClass, SurfaceTerm, parse_surface,
};

fn malformed(input: &[u8]) {
    assert_eq!(
        parse_surface(input).unwrap_err().class(),
        SurfaceErrorClass::MalformedSyntax
    );
}

#[test]
fn parses_named_module_with_layout_and_comments() {
    let input = br#"
        ; leading comment
        ( surface 0 1 )
        ( module Basics )

        ( postulate A ( universe 0 ) )
        ; comments are layout
        ( transparent identity_map
            ( pi x ( ref A ) ( ref A ) )
            ( lam x ( ref x ) ) )
        ; trailing comment
    "#;

    let module = parse_surface(input).expect("valid Surface v0.1 source");
    assert_eq!(module.format(), SURFACE_FORMAT);
    assert_eq!(module.name().as_str(), "Basics");
    assert_eq!(module.declarations().len(), 2);

    let SurfaceDeclaration::Transparent { name, ty, body } = &module.declarations()[1] else {
        panic!("second declaration should be transparent");
    };
    assert_eq!(name.as_str(), "identity_map");

    let SurfaceTerm::Pi {
        binder,
        domain,
        codomain,
    } = module.arena().get(*ty).unwrap()
    else {
        panic!("transparent type should be pi");
    };
    assert_eq!(binder.as_str(), "x");
    assert!(matches!(
        module.arena().get(*domain),
        Some(SurfaceTerm::Ref(_))
    ));
    assert!(matches!(
        module.arena().get(*codomain),
        Some(SurfaceTerm::Ref(_))
    ));

    let SurfaceTerm::Lam {
        binder,
        body: lambda_body,
    } = module.arena().get(*body).unwrap()
    else {
        panic!("transparent body should be lambda");
    };
    assert_eq!(binder.as_str(), "x");
    let Some(SurfaceTerm::Ref(reference)) = module.arena().get(*lambda_body) else {
        panic!("lambda body should be a named reference");
    };
    assert_eq!(reference.as_str(), "x");
}

#[test]
fn parses_every_surface_term_form() {
    let input = br#"
(surface 0 1)
(module Forms)
(postulate t_ref (ref missing))
(postulate t_universe (universe 2))
(postulate t_pi (pi x nat (ref x)))
(postulate t_lam (lam x (ref x)))
(postulate t_app (app (ref f) (ref a)))
(postulate t_sigma (sigma x nat (ref x)))
(postulate t_pair (pair zero zero))
(postulate t_fst (fst (ref p)))
(postulate t_snd (snd (ref p)))
(postulate t_id (id nat zero zero))
(postulate t_refl (refl zero))
(postulate t_j (j nat zero (ref C) zero zero (refl zero)))
(postulate t_empty empty)
(postulate t_empty_elim (empty-elim (ref C) (ref e)))
(postulate t_unit unit)
(postulate t_star star)
(postulate t_unit_elim (unit-elim (ref C) zero star))
(postulate t_nat nat)
(postulate t_zero zero)
(postulate t_succ (succ zero))
(postulate t_nat_elim (nat-elim (ref C) zero (ref s) zero))
(postulate t_ann (ann (lam x (ref x)) (pi x nat nat)))
"#;

    let module = parse_surface(input).expect("all Surface term forms must parse");
    assert_eq!(module.declarations().len(), 22);
}

#[test]
fn version_envelope_distinguishes_unsupported_from_malformed() {
    assert_eq!(
        parse_surface(b"(surface 0 2) (module M)")
            .unwrap_err()
            .class(),
        SurfaceErrorClass::UnsupportedVersion
    );
    assert_eq!(
        parse_surface(b"(surface 1 0) (module M)")
            .unwrap_err()
            .class(),
        SurfaceErrorClass::UnsupportedVersion
    );
    malformed(b"(surface 00 1) (module M)");
    malformed(b"(surface x 1) (module M)");
    malformed(b"(surface 0) (module M)");
}

#[test]
fn rejects_bom_and_invalid_utf8() {
    malformed(b"\\xef\\xbb\\xbf(surface 0 1) (module M)");
    malformed(&[b'(', b's', b'u', 0xff, b')']);
}

#[test]
fn reserved_words_are_rejected_in_every_name_position() {
    for input in [
        b"(surface 0 1) (module ref)".as_slice(),
        b"(surface 0 1) (module M) (postulate id nat)".as_slice(),
        b"(surface 0 1) (module M) (postulate p (pi lam nat nat))".as_slice(),
        b"(surface 0 1) (module M) (postulate p (ref pi))".as_slice(),
    ] {
        assert_eq!(
            parse_surface(input).unwrap_err().class(),
            SurfaceErrorClass::ReservedIdentifier
        );
    }
}

#[test]
fn rejects_non_identifiers_in_name_positions() {
    malformed(b"(surface 0 1) (module bad-name)");
    malformed(b"(surface 0 1) (module M) (postulate 9x nat)");
    malformed(b"(surface 0 1) (module M) (postulate p (lam x-y zero))");
}

#[test]
fn comments_never_concatenate_token_fragments() {
    malformed(
        b"(surface 0 1) (module Ba; this comment separates tokens
sics)",
    );

    let module = parse_surface(
        b"(surface 0 1);header
(module M);module
; eof comment",
    )
    .expect("comments at token boundaries are valid layout");
    assert_eq!(module.name().as_str(), "M");
}

#[test]
fn unknown_names_are_parser_input_not_parser_errors() {
    let module = parse_surface(b"(surface 0 1) (module M) (postulate p (ref definitely_missing))")
        .expect("name resolution belongs to the elaboration slice");

    let ty = module.declarations()[0].ty();
    let Some(SurfaceTerm::Ref(name)) = module.arena().get(ty) else {
        panic!("expected named reference");
    };
    assert_eq!(name.as_str(), "definitely_missing");
}

#[test]
fn duplicate_global_names_are_deferred_to_name_resolution() {
    let module = parse_surface(b"(surface 0 1) (module M) (postulate A nat) (postulate A unit)")
        .expect("duplicate-global rejection belongs to the elaboration slice");
    assert_eq!(module.declarations().len(), 2);
}

#[test]
fn rejects_unknown_tags_wrong_arities_and_trailing_tokens() {
    malformed(b"(surface 0 1) (module M) (postulate p (mystery zero))");
    malformed(b"(surface 0 1) (module M) (postulate p (succ))");
    malformed(b"(surface 0 1) (module M) (postulate p (succ zero zero))");
    malformed(b"(surface 0 1) (module M) surprise");
}

#[test]
fn naturals_are_canonical_and_unbounded() {
    malformed(b"(surface 0 1) (module M) (postulate p (universe 01))");

    let huge = "9".repeat(512);
    let input = format!("(surface 0 1) (module M) (postulate p (universe {huge}))");
    let module = parse_surface(input.as_bytes()).expect("unbounded natural must parse");
    let ty = module.declarations()[0].ty();
    let Some(SurfaceTerm::Universe(level)) = module.arena().get(ty) else {
        panic!("expected universe");
    };
    assert_eq!(level.as_str(), huge);
}

#[test]
fn deep_terms_do_not_use_the_rust_call_stack() {
    const DEPTH: usize = 10_000;
    let mut input = String::from("(surface 0 1) (module Deep) (postulate p ");
    for _ in 0..DEPTH {
        input.push_str("(succ ");
    }
    input.push_str("zero");
    for _ in 0..DEPTH {
        input.push(')');
    }
    input.push(')');

    let module = parse_surface(input.as_bytes()).expect("deep term must parse iteratively");
    assert_eq!(module.declarations().len(), 1);
    assert_eq!(module.arena().len(), DEPTH + 1);
}

#[test]
fn all_three_declaration_kinds_parse() {
    let module = parse_surface(
        b"(surface 0 1) (module M)           (postulate A (universe 0))           (transparent t nat zero)           (opaque o unit star)",
    )
    .unwrap();

    assert!(matches!(
        &module.declarations()[0],
        SurfaceDeclaration::Postulate { .. }
    ));
    assert!(matches!(
        &module.declarations()[1],
        SurfaceDeclaration::Transparent { .. }
    ));
    assert!(matches!(
        &module.declarations()[2],
        SurfaceDeclaration::Opaque { .. }
    ));
}

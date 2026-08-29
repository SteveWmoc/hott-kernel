use hott_kernel::{
    FormatErrorClass, parse_canonical, parse_transport, print_canonical, print_semantic,
};

const CANONICAL: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("format/canonical/empty.core"),
        include_bytes!("format/semantic/empty.semantic"),
    ),
    (
        include_bytes!("format/canonical/identity-u0.core"),
        include_bytes!("format/semantic/identity-u0.semantic"),
    ),
    (
        include_bytes!("format/canonical/identity-u0-renamed.core"),
        include_bytes!("format/semantic/identity-u0.semantic"),
    ),
    (
        include_bytes!("format/canonical/unit-transparent.core"),
        include_bytes!("format/semantic/unit-transparent.semantic"),
    ),
    (
        include_bytes!("format/canonical/unit-opaque.core"),
        include_bytes!("format/semantic/unit-opaque.semantic"),
    ),
];

#[test]
fn canonical_fixtures_round_trip_byte_for_byte() {
    for (artifact, semantic) in CANONICAL {
        let module = parse_canonical(artifact).expect("canonical fixture must parse");
        assert_eq!(print_canonical(&module).unwrap().as_slice(), *artifact);
        assert_eq!(print_semantic(&module).unwrap().as_slice(), *semantic);
    }
}

#[test]
fn renamed_identity_has_the_same_semantic_projection() {
    let first = parse_canonical(include_bytes!("format/canonical/identity-u0.core")).unwrap();
    let renamed =
        parse_canonical(include_bytes!("format/canonical/identity-u0-renamed.core")).unwrap();
    assert_eq!(
        print_semantic(&first).unwrap(),
        print_semantic(&renamed).unwrap()
    );
    assert_ne!(
        print_canonical(&first).unwrap(),
        print_canonical(&renamed).unwrap()
    );
}

#[test]
fn noncanonical_transport_is_accepted_only_by_transport_parser() {
    let input = include_bytes!("format/noncanonical/identity-whitespace.input");
    let expected = include_bytes!("format/canonical/identity-u0.core");
    let module = parse_transport(input).expect("transport whitespace is valid");
    assert_eq!(print_canonical(&module).unwrap().as_slice(), expected);
    assert_eq!(
        parse_canonical(input).unwrap_err().class(),
        FormatErrorClass::NoncanonicalArtifact
    );
}

#[test]
fn malformed_fixtures_are_format_errors() {
    let malformed: &[&[u8]] = &[
        include_bytes!("format/malformed/duplicate-name.core"),
        include_bytes!("format/malformed/forbidden-escape.core"),
        include_bytes!("format/malformed/leading-zero.core"),
        include_bytes!("format/malformed/trailing-token.core"),
        include_bytes!("format/malformed/wrong-arity.core"),
    ];
    for input in malformed {
        assert_eq!(
            parse_transport(input).unwrap_err().class(),
            FormatErrorClass::MalformedEncoding
        );
    }

    let bytes = decode_hex(include_str!("format/malformed/invalid-utf8.hex"));
    assert_eq!(
        parse_transport(&bytes).unwrap_err().class(),
        FormatErrorClass::MalformedEncoding
    );
}

#[test]
fn unsupported_version_is_not_malformed() {
    assert_eq!(
        parse_transport(include_bytes!("format/unsupported/format-0.2.core"))
            .unwrap_err()
            .class(),
        FormatErrorClass::UnsupportedVersion
    );
}

#[test]
fn logically_invalid_format_fixtures_still_parse() {
    for input in [
        include_bytes!("format/invalid-judgment/forward-reference.core").as_slice(),
        include_bytes!("format/invalid-judgment/out-of-scope-variable.core").as_slice(),
    ] {
        let module = parse_canonical(input).expect("logical rejection belongs to the checker");
        assert_eq!(print_canonical(&module).unwrap().as_slice(), input);
    }
}

#[test]
fn every_conformance_module_passes_the_format_layer() {
    let accepted: &[&[u8]] = &[
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
    let rejected: &[&[u8]] = &[
        include_bytes!("conformance/rejected/bad-body.core"),
        include_bytes!("conformance/rejected/bare-lambda-synthesis.core"),
        include_bytes!("conformance/rejected/bare-motive.core"),
        include_bytes!("conformance/rejected/bare-pair-synthesis.core"),
        include_bytes!("conformance/rejected/empty-no-computation.core"),
        include_bytes!("conformance/rejected/equality-reflection.core"),
        include_bytes!("conformance/rejected/false-refl.core"),
        include_bytes!("conformance/rejected/forward-reference.core"),
        include_bytes!("conformance/rejected/j-motive-path-domain.core"),
        include_bytes!("conformance/rejected/j-path-mismatch.core"),
        include_bytes!("conformance/rejected/j-wrong-branch.core"),
        include_bytes!("conformance/rejected/no-pi-eta.core"),
        include_bytes!("conformance/rejected/no-sigma-eta.core"),
        include_bytes!("conformance/rejected/no-uip.core"),
        include_bytes!("conformance/rejected/opaque-no-delta.core"),
        include_bytes!("conformance/rejected/out-of-scope-variable.core"),
        include_bytes!("conformance/rejected/unit-motive-domain.core"),
        include_bytes!("conformance/rejected/unit-no-uniqueness.core"),
        include_bytes!("conformance/rejected/universe-noncumulative.core"),
    ];

    for input in accepted.iter().chain(rejected.iter()) {
        let module = parse_canonical(input).expect("conformance files are canonical format");
        assert_eq!(print_canonical(&module).unwrap().as_slice(), *input);
    }
}

#[test]
fn deep_terms_do_not_use_the_rust_call_stack() {
    const DEPTH: usize = 10_000;
    let mut input = String::from(
        "(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (transparent \"deep\" nat ",
    );
    for _ in 0..DEPTH {
        input.push_str("(succ ");
    }
    input.push_str("zero");
    for _ in 0..DEPTH {
        input.push(')');
    }
    input.push_str(")))\n");

    let module = parse_canonical(input.as_bytes()).expect("deep canonical term must parse");
    assert_eq!(
        print_canonical(&module).unwrap().as_slice(),
        input.as_bytes()
    );
}

#[test]
fn naturals_are_not_limited_to_machine_words() {
    let level = "9".repeat(512);
    let input = format!(
        "(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"huge\" (universe {level}))))\n"
    );
    let module = parse_canonical(input.as_bytes()).expect("unbounded decimal level must parse");
    assert_eq!(
        print_canonical(&module).unwrap().as_slice(),
        input.as_bytes()
    );
}

fn decode_hex(text: &str) -> Vec<u8> {
    let compact: String = text
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .collect();
    assert_eq!(compact.len() % 2, 0);
    compact
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let hi = hex_digit(pair[0]);
            let lo = hex_digit(pair[1]);
            (hi << 4) | lo
        })
        .collect()
}

fn hex_digit(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => panic!("non-hex fixture byte"),
    }
}

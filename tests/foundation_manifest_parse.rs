use hott_kernel::{
    AuditDeclarationKind, FormatErrorClass, KernelFeature, parse_foundation_manifest,
};

fn identity_fixture() -> &'static [u8] {
    include_bytes!("format/manifests/identity-u0.manifest.json")
}

fn malformed(input: &[u8]) {
    let error = parse_foundation_manifest(input).unwrap_err();
    assert_eq!(error.class(), FormatErrorClass::MalformedEncoding);
}

fn resource_exhausted(input: &[u8]) {
    let error = parse_foundation_manifest(input).unwrap_err();
    assert_eq!(error.class(), FormatErrorClass::ResourceExhausted);
}

#[test]
fn positive_exponent_overflow_is_resource_exhausted() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let input = source.replace(
        "\"index\": 0,",
        "\"index\": 1e999999999999999999999999999999999999999999999999999999999999,",
    );
    resource_exhausted(input.as_bytes());
}

#[test]
fn integer_magnitude_overflow_is_resource_exhausted() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let input = source.replace(
        "\"index\": 0,",
        "\"index\": 999999999999999999999999999999999999999999999999999999999999,",
    );
    resource_exhausted(input.as_bytes());
}

#[test]
fn parses_frozen_identity_manifest() {
    let manifest = parse_foundation_manifest(identity_fixture()).expect("fixture must parse");

    assert_eq!(manifest.schema(), "hott-foundation-manifest/0.1");
    assert_eq!(manifest.artifact_format(), "hott-core/0.1");
    assert_eq!(
        manifest.artifact_sha256(),
        "c24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea"
    );
    assert_eq!(manifest.semantic_projection(), "hott-semantic/0.1");
    assert_eq!(
        manifest.semantic_sha256(),
        "043e467fc7e2ff8c1ee847e2453acbe8e490c27fe47ade2a81361e50dce772aa"
    );
    assert_eq!(manifest.kernel_theory(), "mltt-core");
    assert_eq!(manifest.kernel_version(), "0.1");
    assert_eq!(manifest.feature_vocabulary(), "mltt-core-features/0.1");
    assert!(manifest.asserted_provenance().is_empty());

    let declarations = manifest.declarations();
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0].index(), 0);
    assert_eq!(declarations[0].display_name(), "id-U0");
    assert_eq!(declarations[0].kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declarations[0].direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert!(declarations[0].direct().extensions().is_empty());
    assert!(declarations[0].direct().postulates().is_empty());
    assert!(declarations[0].direct().declarations().is_empty());
}

#[test]
fn accepts_json_key_reordering_and_insignificant_whitespace() {
    let input = br#"{
      "asserted_provenance" : [],
      "audit" : { "declarations" : [], "feature_vocabulary" : "mltt-core-features/0.1" },
      "kernel" : { "version" : "0.1", "theory" : "mltt-core" },
      "artifact" : {
        "semantic_sha256" : "af4c8b526f9a1fe4a03efe2cfe60b0744f0a7f6bcca227f762d3e1c6d9de5deb",
        "semantic_projection" : "hott-semantic/0.1",
        "sha256" : "d15f02d6d077829b1133995f8f81b3d8e404bb7bef976368eb86fd7a98d22834",
        "format" : "hott-core/0.1"
      },
      "schema" : "hott-foundation-manifest/0.1"
    }"#;

    let manifest = parse_foundation_manifest(input).expect("reordered manifest must parse");
    assert!(manifest.declarations().is_empty());
}

#[test]
fn accepts_noncanonical_integer_spelling_and_surrogate_pair() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let source = source.replace("\"index\": 0,", "\"index\": 0e0,");
    let source = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [
    {
      "declaration": -0.0,
      "generated_by": [
        {
          "kind": "ai-system",
          "name": "\uD83D\uDE80"
        }
      ]
    }
  ]"#,
    );

    let manifest =
        parse_foundation_manifest(source.as_bytes()).expect("valid JSON forms must parse");
    assert_eq!(manifest.declarations()[0].index(), 0);
    assert_eq!(manifest.asserted_provenance()[0].declaration(), 0);
    assert_eq!(
        manifest.asserted_provenance()[0].generated_by()[0].name(),
        "🚀"
    );
}

#[test]
fn rejects_bom_invalid_utf8_and_lone_surrogate() {
    malformed(b"\xef\xbb\xbf{}");
    malformed(&[b'{', b'"', 0xff, b'"', b':', b'0', b'}']);
    malformed(include_bytes!(
        "format/manifests/invalid-lone-surrogate.json"
    ));
}

#[test]
fn rejects_duplicate_and_additional_object_keys() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let duplicate = source.replacen(
        "{\n  \"schema\":",
        "{\n  \"schema\": \"hott-foundation-manifest/0.1\",\n  \"sche\\u006da\":",
        1,
    );
    malformed(duplicate.as_bytes());

    let additional = source.replacen("{\n  \"schema\":", "{\n  \"extra\": 0,\n  \"schema\":", 1);
    malformed(additional.as_bytes());
}

#[test]
fn unsupported_manifest_identity_is_not_malformed() {
    let source = core::str::from_utf8(identity_fixture()).unwrap().replace(
        "hott-foundation-manifest/0.1",
        "hott-foundation-manifest/9.9",
    );
    let error = parse_foundation_manifest(source.as_bytes()).unwrap_err();
    assert_eq!(error.class(), FormatErrorClass::UnsupportedVersion);
}

#[test]
fn rejects_schema_shape_and_hash_violations() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    malformed(
        source
            .replace("\"asserted_provenance\": []", "\"asserted_provenance\": {}")
            .as_bytes(),
    );
    malformed(
        source
            .replace(
                "c24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea",
                "C24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea",
            )
            .as_bytes(),
    );
    malformed(
        source
            .replace("\"kind\": \"transparent\"", "\"kind\": \"mystery\"")
            .as_bytes(),
    );
}

#[test]
fn rejects_noncanonical_set_order_and_nonempty_extensions() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let reversed = source.replace(
        "\"pi\",\n            \"universe\"",
        "\"universe\",\n            \"pi\"",
    );
    malformed(reversed.as_bytes());

    let extension = source.replacen("\"extensions\": []", "\"extensions\": [\"cubical\"]", 1);
    malformed(extension.as_bytes());
}

#[test]
fn rejects_wrong_record_index_and_non_backward_dependency() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    malformed(source.replace("\"index\": 0,", "\"index\": 1,").as_bytes());

    let forward = source.replacen("\"declarations\": []", "\"declarations\": [0]", 1);
    malformed(forward.as_bytes());
}

#[test]
fn rejects_transitive_features_that_omit_direct_features() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let invalid = source.replacen(
        "\"transitive\": {\n          \"kernel_features\": [\n            \"pi\",\n            \"universe\"\n          ]",
        "\"transitive\": {\n          \"kernel_features\": []",
        1,
    );
    malformed(invalid.as_bytes());
}

#[test]
fn rejects_transitive_declarations_that_omit_direct_declarations() {
    let source = br#"{
      "schema": "hott-foundation-manifest/0.1",
      "artifact": {
        "format": "hott-core/0.1",
        "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "semantic_projection": "hott-semantic/0.1",
        "semantic_sha256": "1111111111111111111111111111111111111111111111111111111111111111"
      },
      "kernel": { "theory": "mltt-core", "version": "0.1" },
      "audit": {
        "feature_vocabulary": "mltt-core-features/0.1",
        "declarations": [
          {
            "index": 0,
            "display_name": "a",
            "kind": "postulate",
            "direct": {
              "kernel_features": [], "extensions": [], "postulates": [], "declarations": []
            },
            "transitive": {
              "kernel_features": [], "extensions": [], "postulates": [], "declarations": []
            }
          },
          {
            "index": 1,
            "display_name": "b",
            "kind": "transparent",
            "direct": {
              "kernel_features": [], "extensions": [], "postulates": [0], "declarations": [0]
            },
            "transitive": {
              "kernel_features": [], "extensions": [], "postulates": [], "declarations": []
            }
          }
        ]
      },
      "asserted_provenance": []
    }"#;
    malformed(source);
}

#[test]
fn rejects_postulate_dependencies_that_name_non_postulate_records() {
    let template = r#"{
      "schema": "hott-foundation-manifest/0.1",
      "artifact": {
        "format": "hott-core/0.1",
        "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "semantic_projection": "hott-semantic/0.1",
        "semantic_sha256": "1111111111111111111111111111111111111111111111111111111111111111"
      },
      "kernel": { "theory": "mltt-core", "version": "0.1" },
      "audit": {
        "feature_vocabulary": "mltt-core-features/0.1",
        "declarations": [
          {
            "index": 0,
            "display_name": "a",
            "kind": "KIND",
            "direct": {
              "kernel_features": [], "extensions": [], "postulates": [], "declarations": []
            },
            "transitive": {
              "kernel_features": [], "extensions": [], "postulates": [], "declarations": []
            }
          },
          {
            "index": 1,
            "display_name": "b",
            "kind": "transparent",
            "direct": {
              "kernel_features": [], "extensions": [], "postulates": [0], "declarations": [0]
            },
            "transitive": {
              "kernel_features": [], "extensions": [], "postulates": [0], "declarations": [0]
            }
          }
        ]
      },
      "asserted_provenance": []
    }"#;

    for kind in ["transparent", "opaque"] {
        let source = template.replace("KIND", kind);
        malformed(source.as_bytes());
    }
}

#[test]
fn rejects_postulates_not_subset_of_declarations() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let invalid = source.replacen(
        "\"postulates\": [],\n          \"declarations\": []",
        "\"postulates\": [0],\n          \"declarations\": []",
        1,
    );
    malformed(invalid.as_bytes());
}

#[test]
fn parses_and_orders_asserted_provenance() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();
    let source = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [
    {
      "declaration": 0,
      "generated_by": [
        { "kind": "ai-system", "name": "A", "version": "1" },
        { "kind": "ai-system", "name": "B", "details": "reviewed" }
      ]
    }
  ]"#,
    );
    let manifest = parse_foundation_manifest(source.as_bytes()).expect("provenance must parse");
    let provenance = &manifest.asserted_provenance()[0];
    assert_eq!(provenance.declaration(), 0);
    assert_eq!(provenance.generated_by()[0].version(), Some("1"));
    assert_eq!(provenance.generated_by()[1].details(), Some("reviewed"));
}

#[test]
fn rejects_invalid_provenance_shape_order_and_controls() {
    let source = core::str::from_utf8(identity_fixture()).unwrap();

    let empty_generators = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [{ "declaration": 0, "generated_by": [] }]"#,
    );
    malformed(empty_generators.as_bytes());

    let bad_identifier = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [{ "declaration": 0, "generated_by": [{ "kind": "AI", "name": "x" }] }]"#,
    );
    malformed(bad_identifier.as_bytes());

    let control = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [{ "declaration": 0, "generated_by": [{ "kind": "ai", "name": "line\nfeed" }] }]"#,
    );
    malformed(control.as_bytes());

    let reversed = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [{ "declaration": 0, "generated_by": [
      { "kind": "ai", "name": "B" },
      { "kind": "ai", "name": "A" }
    ] }]"#,
    );
    malformed(reversed.as_bytes());
}

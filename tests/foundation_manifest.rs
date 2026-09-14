use hott_kernel::{
    CheckErrorClass, ManifestBuildError, build_foundation_manifest, parse_canonical,
    print_canonical, print_foundation_manifest,
};

#[test]
fn identity_manifest_matches_frozen_fixture_byte_for_byte() {
    let mut module = parse_canonical(include_bytes!("format/canonical/identity-u0.core"))
        .expect("canonical identity fixture must parse");
    let manifest = build_foundation_manifest(&mut module).expect("identity fixture must check");
    let json = print_foundation_manifest(&manifest).expect("manifest must serialize");

    assert_eq!(
        json.as_slice(),
        include_bytes!("format/manifests/identity-u0.manifest.json")
    );
}

#[test]
fn empty_manifest_matches_frozen_spec_byte_for_byte() {
    let mut module = parse_canonical(include_bytes!("format/canonical/empty.core"))
        .expect("canonical empty fixture must parse");
    let manifest = build_foundation_manifest(&mut module).expect("empty fixture must check");
    let json = print_foundation_manifest(&manifest).expect("manifest must serialize");

    let expected = b"{\n  \"schema\": \"hott-foundation-manifest/0.1\",\n  \"artifact\": {\n    \"format\": \"hott-core/0.1\",\n    \"sha256\": \"d15f02d6d077829b1133995f8f81b3d8e404bb7bef976368eb86fd7a98d22834\",\n    \"semantic_projection\": \"hott-semantic/0.1\",\n    \"semantic_sha256\": \"af4c8b526f9a1fe4a03efe2cfe60b0744f0a7f6bcca227f762d3e1c6d9de5deb\"\n  },\n  \"kernel\": {\n    \"theory\": \"mltt-core\",\n    \"version\": \"0.1\"\n  },\n  \"audit\": {\n    \"feature_vocabulary\": \"mltt-core-features/0.1\",\n    \"declarations\": []\n  },\n  \"asserted_provenance\": []\n}\n";

    assert_eq!(json.as_slice(), expected);
    assert!(manifest.audit().declarations().is_empty());
}

#[test]
fn rejected_module_never_produces_a_manifest() {
    let mut module = parse_canonical(include_bytes!("conformance/rejected/bad-body.core"))
        .expect("rejected conformance fixture is canonical format");

    let error = build_foundation_manifest(&mut module).unwrap_err();
    match error {
        ManifestBuildError::Check(error) => {
            assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        }
        ManifestBuildError::Format(error) => {
            panic!("logical rejection was relabeled as format failure: {error}");
        }
    }
}

#[test]
fn generation_preserves_the_source_module() {
    let mut module = parse_canonical(include_bytes!("format/canonical/unit-transparent.core"))
        .expect("canonical fixture must parse");
    let before = print_canonical(&module).unwrap();

    let manifest = build_foundation_manifest(&mut module).expect("fixture must check");
    let _ = print_foundation_manifest(&manifest).unwrap();

    assert_eq!(print_canonical(&module).unwrap(), before);
}

#[test]
fn display_names_are_escaped_as_json_strings() {
    let input = b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (transparent \"quote\\\"slash\\\\name\" unit star)))\n";
    let mut module = parse_canonical(input).expect("escaped Core display name must parse");
    let manifest = build_foundation_manifest(&mut module).expect("fixture must check");
    let json = print_foundation_manifest(&manifest).expect("manifest must serialize");
    let text = core::str::from_utf8(&json).unwrap();

    assert!(text.contains("\"display_name\": \"quote\\\"slash\\\\name\""));
    assert!(text.ends_with("  \"asserted_provenance\": []\n}\n"));
}

#[test]
fn multi_record_dependency_arrays_have_deterministic_layout() {
    let input = b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"p\" unit) (transparent \"a\" unit (global 0)) (transparent \"b\" unit (global 1))))\n";
    let mut module = parse_canonical(input).expect("dependency-chain fixture must parse");
    let manifest =
        build_foundation_manifest(&mut module).expect("dependency-chain fixture must check");
    let json = print_foundation_manifest(&manifest).expect("manifest must serialize");
    let text = core::str::from_utf8(&json).unwrap();

    let expected = r#"    "declarations": [
      {
        "index": 0,
        "display_name": "p",
        "kind": "postulate",
        "direct": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [],
          "declarations": []
        },
        "transitive": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [],
          "declarations": []
        }
      },
      {
        "index": 1,
        "display_name": "a",
        "kind": "transparent",
        "direct": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [
            0
          ],
          "declarations": [
            0
          ]
        },
        "transitive": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [
            0
          ],
          "declarations": [
            0
          ]
        }
      },
      {
        "index": 2,
        "display_name": "b",
        "kind": "transparent",
        "direct": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [],
          "declarations": [
            1
          ]
        },
        "transitive": {
          "kernel_features": [
            "unit"
          ],
          "extensions": [],
          "postulates": [
            0
          ],
          "declarations": [
            0,
            1
          ]
        }
      }
    ]"#;

    assert!(text.contains(expected));
    assert_eq!(
        manifest.audit().declarations()[2].direct().declarations(),
        &[1]
    );
    assert_eq!(
        manifest.audit().declarations()[2]
            .transitive()
            .declarations(),
        &[0, 1]
    );
    assert_eq!(
        manifest.audit().declarations()[2].transitive().postulates(),
        &[0]
    );
}

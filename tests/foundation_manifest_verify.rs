use hott_kernel::{ManifestVerifyErrorClass, verify_foundation_manifest};

fn identity_core() -> &'static [u8] {
    include_bytes!("format/canonical/identity-u0.core")
}

fn identity_manifest() -> &'static [u8] {
    include_bytes!("format/manifests/identity-u0.manifest.json")
}

fn verify_class(core: &[u8], manifest: &[u8], expected: ManifestVerifyErrorClass) {
    let error = verify_foundation_manifest(core, manifest).unwrap_err();
    assert_eq!(error.class(), expected);
}

#[test]
fn verifies_frozen_identity_manifest() {
    let manifest = verify_foundation_manifest(identity_core(), identity_manifest())
        .expect("frozen identity manifest must verify");
    assert_eq!(manifest.declarations().len(), 1);
    assert!(manifest.asserted_provenance().is_empty());
}

#[test]
fn asserted_provenance_is_preserved_but_excluded_from_comparison() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace(
        "\"asserted_provenance\": []",
        r#""asserted_provenance": [
    {
      "declaration": 0,
      "generated_by": [
        { "kind": "ai-system", "name": "Madam Flash", "version": "5p3" }
      ]
    }
  ]"#,
    );

    let manifest = verify_foundation_manifest(identity_core(), supplied.as_bytes())
        .expect("provenance does not participate in deterministic comparison");
    assert_eq!(manifest.asserted_provenance().len(), 1);
    assert_eq!(
        manifest.asserted_provenance()[0].generated_by()[0].name(),
        "Madam Flash"
    );
}

#[test]
fn reports_artifact_hash_mismatch_before_later_deterministic_fields() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace(
        "c24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );
    verify_class(
        identity_core(),
        supplied.as_bytes(),
        ManifestVerifyErrorClass::ArtifactHashMismatch,
    );
}

#[test]
fn renamed_artifact_reports_artifact_hash_mismatch_even_when_semantic_hash_matches() {
    let renamed = include_bytes!("format/canonical/identity-u0-renamed.core");
    verify_class(
        renamed,
        identity_manifest(),
        ManifestVerifyErrorClass::ArtifactHashMismatch,
    );
}

#[test]
fn reports_semantic_hash_mismatch_after_artifact_hash_matches() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace(
        "043e467fc7e2ff8c1ee847e2453acbe8e490c27fe47ade2a81361e50dce772aa",
        "1111111111111111111111111111111111111111111111111111111111111111",
    );
    verify_class(
        identity_core(),
        supplied.as_bytes(),
        ManifestVerifyErrorClass::SemanticHashMismatch,
    );
}

#[test]
fn reports_manifest_mismatch_for_display_name() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace("\"display_name\": \"id-U0\"", "\"display_name\": \"wrong\"");
    verify_class(
        identity_core(),
        supplied.as_bytes(),
        ManifestVerifyErrorClass::ManifestMismatch,
    );
}

#[test]
fn reports_manifest_mismatch_for_declaration_kind() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace("\"kind\": \"transparent\"", "\"kind\": \"opaque\"");
    verify_class(
        identity_core(),
        supplied.as_bytes(),
        ManifestVerifyErrorClass::ManifestMismatch,
    );
}

#[test]
fn reports_manifest_mismatch_for_dependency_sets() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let supplied = source.replace("\"pi\",\n            \"universe\"", "\"universe\"");
    verify_class(
        identity_core(),
        supplied.as_bytes(),
        ManifestVerifyErrorClass::ManifestMismatch,
    );
}

#[test]
fn malformed_manifest_remains_malformed_encoding() {
    verify_class(
        identity_core(),
        b"{}",
        ManifestVerifyErrorClass::MalformedEncoding,
    );
}

#[test]
fn unsupported_manifest_identities_remain_unsupported_version() {
    let source = core::str::from_utf8(identity_manifest()).unwrap();
    let mutations = [
        (
            "\"schema\": \"hott-foundation-manifest/0.1\"",
            "\"schema\": \"hott-foundation-manifest/9.9\"",
        ),
        (
            "\"format\": \"hott-core/0.1\"",
            "\"format\": \"hott-core/9.9\"",
        ),
        (
            "\"semantic_projection\": \"hott-semantic/0.1\"",
            "\"semantic_projection\": \"hott-semantic/9.9\"",
        ),
        ("\"theory\": \"mltt-core\"", "\"theory\": \"foreign-core\""),
        ("\"version\": \"0.1\"", "\"version\": \"9.9\""),
        (
            "\"feature_vocabulary\": \"mltt-core-features/0.1\"",
            "\"feature_vocabulary\": \"mltt-core-features/9.9\"",
        ),
    ];

    for (supported, unsupported) in mutations {
        assert!(
            source.contains(supported),
            "fixture must contain {supported}"
        );
        let supplied = source.replacen(supported, unsupported, 1);
        verify_class(
            identity_core(),
            supplied.as_bytes(),
            ManifestVerifyErrorClass::UnsupportedVersion,
        );
    }
}

#[test]
fn noncanonical_core_is_rejected_before_manifest_parsing() {
    let mut core = Vec::with_capacity(identity_core().len() + 1);
    core.push(b' ');
    core.extend_from_slice(identity_core());
    verify_class(&core, b"{}", ManifestVerifyErrorClass::NoncanonicalArtifact);
}

#[test]
fn invalid_judgment_is_reported_before_manifest_parsing() {
    let invalid = include_bytes!("format/invalid-judgment/out-of-scope-variable.core");
    verify_class(invalid, b"{}", ManifestVerifyErrorClass::InvalidJudgment);
}

#[test]
fn malformed_core_is_reported_before_manifest_parsing() {
    verify_class(
        b"(",
        identity_manifest(),
        ManifestVerifyErrorClass::MalformedEncoding,
    );
}

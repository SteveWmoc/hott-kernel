use hott_kernel::{compute_module_hashes, parse_canonical, parse_transport};

struct Fixture {
    artifact: &'static [u8],
    artifact_sha256: &'static str,
    semantic_sha256: &'static str,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        artifact: include_bytes!("format/canonical/empty.core"),
        artifact_sha256: "d15f02d6d077829b1133995f8f81b3d8e404bb7bef976368eb86fd7a98d22834",
        semantic_sha256: "af4c8b526f9a1fe4a03efe2cfe60b0744f0a7f6bcca227f762d3e1c6d9de5deb",
    },
    Fixture {
        artifact: include_bytes!("format/canonical/identity-u0.core"),
        artifact_sha256: "c24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea",
        semantic_sha256: "043e467fc7e2ff8c1ee847e2453acbe8e490c27fe47ade2a81361e50dce772aa",
    },
    Fixture {
        artifact: include_bytes!("format/canonical/identity-u0-renamed.core"),
        artifact_sha256: "1fde31a8426a8f4088bec41b598ecc87e67ccfee04aa2cbc4317e19cc5998282",
        semantic_sha256: "043e467fc7e2ff8c1ee847e2453acbe8e490c27fe47ade2a81361e50dce772aa",
    },
    Fixture {
        artifact: include_bytes!("format/canonical/unit-transparent.core"),
        artifact_sha256: "c01e40b607870cae7d392f603059cf38535de9b5417d82a4ea808e4dcc4ed458",
        semantic_sha256: "90e074556d0e9d1540b7c8ebe58e25955e52237899578c57e560efc0eb8c3ad4",
    },
    Fixture {
        artifact: include_bytes!("format/canonical/unit-opaque.core"),
        artifact_sha256: "08b30d97048374f591cedff9a4722267a7997ac857af0dafb25362c744468b26",
        semantic_sha256: "3f7bf145e6e64f915f0a54683496a0fac5c671992c8acf4b303a776885049ef2",
    },
];

#[test]
fn frozen_artifact_and_semantic_hashes_match() {
    for fixture in FIXTURES {
        let module = parse_canonical(fixture.artifact).expect("frozen fixture must parse");
        let hashes = compute_module_hashes(&module).unwrap();
        assert_eq!(hashes.artifact_sha256().as_str(), fixture.artifact_sha256);
        assert_eq!(hashes.semantic_sha256().as_str(), fixture.semantic_sha256);
    }
}

#[test]
fn display_name_changes_only_the_artifact_identity() {
    let original = parse_canonical(include_bytes!("format/canonical/identity-u0.core")).unwrap();
    let renamed =
        parse_canonical(include_bytes!("format/canonical/identity-u0-renamed.core")).unwrap();

    let original_hashes = compute_module_hashes(&original).unwrap();
    let renamed_hashes = compute_module_hashes(&renamed).unwrap();

    assert_ne!(
        original_hashes.artifact_sha256(),
        renamed_hashes.artifact_sha256()
    );
    assert_eq!(
        original_hashes.semantic_sha256(),
        renamed_hashes.semantic_sha256()
    );
}

#[test]
fn noncanonical_transport_hashes_its_canonical_artifact() {
    let module = parse_transport(include_bytes!(
        "format/noncanonical/identity-whitespace.input"
    ))
    .expect("transport whitespace is valid");
    let hashes = compute_module_hashes(&module).unwrap();

    assert_eq!(
        hashes.artifact_sha256().as_str(),
        "c24d88214220b2d9d24fabc523601e67a5c716a414053c988f73c3b80d9475ea"
    );
    assert_eq!(
        hashes.semantic_sha256().as_str(),
        "043e467fc7e2ff8c1ee847e2453acbe8e490c27fe47ade2a81361e50dce772aa"
    );
}

#[test]
fn hashing_does_not_perform_logical_validation() {
    let module = parse_canonical(include_bytes!(
        "format/invalid-judgment/forward-reference.core"
    ))
    .expect("logical rejection belongs to the checker");

    compute_module_hashes(&module).expect("format identity does not type-check declarations");
}

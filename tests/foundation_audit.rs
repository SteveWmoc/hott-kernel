use hott_kernel::{
    AuditDeclarationKind, CheckErrorClass, FEATURE_VOCABULARY, KernelFeature,
    check_and_extract_audit, parse_canonical,
};
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_paths(kind: &str) -> Vec<PathBuf> {
    let directory = Path::new("tests/conformance").join(kind);
    let mut paths = fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("conformance directory entry is readable")
                .path()
        })
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "core")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

#[test]
fn declaration_kinds_report_exact_direct_dependencies() {
    let bytes = fs::read("tests/conformance/accepted/declaration-kinds.core").unwrap();
    let mut module = parse_canonical(&bytes).unwrap();
    let audit = check_and_extract_audit(&mut module).unwrap();

    assert_eq!(audit.feature_vocabulary(), FEATURE_VOCABULARY);
    let records = audit.declarations();
    assert_eq!(records.len(), 3);

    assert_eq!(records[0].index(), 0);
    assert_eq!(records[0].display_name(), "assumed-unit");
    assert_eq!(records[0].kind(), AuditDeclarationKind::Postulate);
    assert_eq!(
        records[0].direct().kernel_features(),
        &[KernelFeature::Unit]
    );
    assert!(records[0].direct().extensions().is_empty());
    assert!(records[0].direct().postulates().is_empty());
    assert!(records[0].direct().declarations().is_empty());

    assert_eq!(records[1].display_name(), "sealed-unit");
    assert_eq!(records[1].kind(), AuditDeclarationKind::Opaque);
    assert_eq!(
        records[1].direct().kernel_features(),
        &[KernelFeature::Unit]
    );
    assert!(records[1].direct().declarations().is_empty());

    assert_eq!(records[2].display_name(), "copied-assumption");
    assert_eq!(records[2].kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        records[2].direct().kernel_features(),
        &[KernelFeature::Unit]
    );
    assert_eq!(records[2].direct().declarations(), &[0]);
    assert_eq!(records[2].direct().postulates(), &[0]);
    assert_eq!(records[2].transitive().declarations(), &[0]);
    assert_eq!(records[2].transitive().postulates(), &[0]);
    assert_eq!(
        records[2].transitive().kernel_features(),
        &[KernelFeature::Unit]
    );
}

#[test]
fn transitive_closure_includes_indirect_postulates_and_declarations() {
    let bytes = b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"p\" unit) (transparent \"a\" unit (global 0)) (transparent \"b\" unit (global 1))))\n";
    let mut module = parse_canonical(bytes).unwrap();
    let audit = check_and_extract_audit(&mut module).unwrap();
    let b = &audit.declarations()[2];

    assert_eq!(b.direct().declarations(), &[1]);
    assert!(b.direct().postulates().is_empty());
    assert_eq!(b.transitive().declarations(), &[0, 1]);
    assert_eq!(b.transitive().postulates(), &[0]);
    assert_eq!(b.transitive().kernel_features(), &[KernelFeature::Unit]);
}

#[test]
fn overlapping_direct_dependencies_are_unioned_once() {
    let bytes = b"(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"p\" unit) (transparent \"a\" unit (global 0)) (transparent \"b\" unit (global 1)) (transparent \"c\" unit (unit-elim (ann (lam unit) (pi unit (universe 0))) (global 0) (unit-elim (ann (lam unit) (pi unit (universe 0))) (global 1) (global 2))))))\n";
    let mut module = parse_canonical(bytes).unwrap();
    let audit = check_and_extract_audit(&mut module).unwrap();
    let c = &audit.declarations()[3];

    assert_eq!(c.direct().declarations(), &[0, 1, 2]);
    assert_eq!(c.direct().postulates(), &[0]);
    assert_eq!(c.transitive().declarations(), &[0, 1, 2]);
    assert_eq!(c.transitive().postulates(), &[0]);
    assert_eq!(
        c.transitive().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Unit,
            KernelFeature::Universe,
        ]
    );
}

#[test]
fn accepted_corpus_covers_all_features_and_obeys_canonical_ordering() {
    let mut observed = [false; 7];
    let paths = fixture_paths("accepted");
    assert_eq!(paths.len(), 13);

    for path in paths {
        let bytes = fs::read(&path).unwrap();
        let mut module = parse_canonical(&bytes).unwrap();
        let audit = check_and_extract_audit(&mut module)
            .unwrap_or_else(|error| panic!("{} failed audit extraction: {error}", path.display()));

        for record in audit.declarations() {
            for dependencies in [record.direct(), record.transitive()] {
                assert!(dependencies.extensions().is_empty());
                assert!(strictly_increasing(dependencies.postulates()));
                assert!(strictly_increasing(dependencies.declarations()));
                assert!(
                    dependencies
                        .postulates()
                        .iter()
                        .all(|postulate| dependencies
                            .declarations()
                            .binary_search(postulate)
                            .is_ok())
                );
                assert!(
                    dependencies
                        .declarations()
                        .iter()
                        .all(|dependency| *dependency < record.index())
                );
                assert!(
                    dependencies
                        .kernel_features()
                        .windows(2)
                        .all(|window| window[0].as_str() < window[1].as_str())
                );
            }
            for feature in record.direct().kernel_features() {
                observed[feature_slot(*feature)] = true;
            }
        }
    }

    assert_eq!(observed, [true; 7]);
}

#[test]
fn rejected_modules_never_produce_audit_records() {
    let paths = fixture_paths("rejected");
    assert_eq!(paths.len(), 19);

    for path in paths {
        let bytes = fs::read(&path).unwrap();
        let mut module = parse_canonical(&bytes).unwrap();
        let error = match check_and_extract_audit(&mut module) {
            Ok(_) => panic!("{} unexpectedly produced an audit", path.display()),
            Err(error) => error,
        };
        assert_eq!(
            error.class(),
            CheckErrorClass::InvalidJudgment,
            "{} failed with the wrong class",
            path.display()
        );
    }
}

#[test]
fn deep_audit_walk_does_not_use_the_rust_call_stack() {
    const DEPTH: usize = 5_000;
    let mut bytes = String::from(
        "(hott-core (format 0 1) (theory \"mltt-core\" 0 1) (declarations (postulate \"deep\" ",
    );
    bytes.push_str(&"(ann ".repeat(DEPTH));
    bytes.push_str("unit");
    bytes.push_str(&" (universe 0))".repeat(DEPTH));
    bytes.push_str(")))\n");

    let mut module = parse_canonical(bytes.as_bytes()).unwrap();
    let audit = check_and_extract_audit(&mut module).unwrap();
    assert_eq!(
        audit.declarations()[0].direct().kernel_features(),
        &[KernelFeature::Unit, KernelFeature::Universe]
    );
}

fn strictly_increasing(values: &[usize]) -> bool {
    values.windows(2).all(|window| window[0] < window[1])
}

fn feature_slot(feature: KernelFeature) -> usize {
    match feature {
        KernelFeature::Empty => 0,
        KernelFeature::Identity => 1,
        KernelFeature::NaturalNumbers => 2,
        KernelFeature::Pi => 3,
        KernelFeature::Sigma => 4,
        KernelFeature::Unit => 5,
        KernelFeature::Universe => 6,
    }
}

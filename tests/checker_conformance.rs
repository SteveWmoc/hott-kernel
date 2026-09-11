use hott_kernel::{CheckErrorClass, check_module, parse_canonical, print_canonical};
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
fn accepted_conformance_modules_check_and_remain_unchanged() {
    let paths = fixture_paths("accepted");
    assert_eq!(paths.len(), 13, "frozen accepted fixture inventory changed");

    for path in paths {
        let bytes = fs::read(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let mut module = parse_canonical(&bytes).unwrap_or_else(|error| {
            panic!("{} did not parse canonically: {error}", path.display())
        });
        let before = print_canonical(&module).expect("parsed fixture reprints canonically");

        check_module(&mut module)
            .unwrap_or_else(|error| panic!("{} was rejected: {error}", path.display()));

        let after = print_canonical(&module).expect("checked fixture reprints canonically");
        assert_eq!(after, before, "checking mutated {}", path.display());
    }
}

#[test]
fn rejected_conformance_modules_fail_logically_and_remain_unchanged() {
    let paths = fixture_paths("rejected");
    assert_eq!(paths.len(), 19, "frozen rejected fixture inventory changed");

    for path in paths {
        let bytes = fs::read(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let mut module = parse_canonical(&bytes).unwrap_or_else(|error| {
            panic!("{} did not parse canonically: {error}", path.display())
        });
        let before = print_canonical(&module).expect("parsed fixture reprints canonically");

        let error = match check_module(&mut module) {
            Ok(()) => panic!("{} unexpectedly checked", path.display()),
            Err(error) => error,
        };
        assert_eq!(
            error.class(),
            CheckErrorClass::InvalidJudgment,
            "{} failed with the wrong class",
            path.display()
        );

        let after = print_canonical(&module).expect("rejected fixture reprints canonically");
        assert_eq!(after, before, "failed checking mutated {}", path.display());
    }
}

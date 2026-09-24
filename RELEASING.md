# Release process

This checklist governs tagged GitHub source releases of `hott-kernel`. It does
not publish the crate to crates.io; registry publication requires a separate,
explicit decision.

## Prepare the release candidate

1. Merge all intended release changes to `main`.
2. Confirm `Cargo.toml` and `Cargo.lock` carry the intended crate version.
3. Confirm `rust-toolchain.toml`, `Cargo.toml`'s `rust-version`, and CI all
   agree on the supported release toolchain.
4. Confirm the README compatibility table and `CHANGELOG.md` agree with the
   checked-in constants and versioned specifications.
5. On the exact candidate commit, require green CI for formatting, Clippy,
   tests, rustdoc with warnings denied, and `cargo package --locked`.
6. Resolve any release-candidate review findings. Adversarial model review is
   advisory and never substitutes for deterministic checks.
7. Record the exact 40-character `main` commit SHA to be released.

## Tag and publish

1. Create tag `vX.Y.Z` at the recorded commit. Never move an existing release
   tag to a different commit.
2. Create the GitHub Release from that exact tag.
3. Use `hott-kernel vX.Y.Z` as the release title.
4. Use the matching `CHANGELOG.md` entry as the basis for release notes,
   including the versioned-contract compatibility envelope.
5. Do not attach a binary and call it authoritative unless a later release
   process explicitly defines reproducible binary artifacts. GitHub's source
   archives are sufficient for the initial library-only release.

## Verify after publication

1. Confirm the release tag resolves to the recorded commit SHA.
2. Confirm the tagged source contains the expected crate version, Rust pin,
   frozen specifications, conformance fixtures, and manifest schema.
3. Leave the released tag immutable. Subsequent fixes receive a new version and
   tag.
4. Record new development under the `[Unreleased]` section of
   `CHANGELOG.md`.

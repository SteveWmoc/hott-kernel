use core::fmt;

use crate::checker::{
    AuditDependencies, CheckError, CheckErrorClass, DeclarationAudit, FoundationAudit,
    check_and_extract_audit,
};
use crate::error::{FormatError, FormatErrorClass};
use crate::format::{compute_module_hashes, parse_canonical};

use super::{ParsedAuditDependencies, ParsedFoundationManifest, parse_foundation_manifest};

/// Stable public classification of complete Foundation Manifest verification failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ManifestVerifyErrorClass {
    MalformedEncoding,
    UnsupportedVersion,
    NoncanonicalArtifact,
    InvalidJudgment,
    ArtifactHashMismatch,
    SemanticHashMismatch,
    ManifestMismatch,
    ResourceExhausted,
}

/// Failure while verifying a supplied Foundation Manifest against Core artifact bytes.
///
/// Format and checker failures retain their lower-layer diagnostics. The three
/// mismatch variants are nonlogical integrity/audit failures from the frozen
/// Core v0.1 result vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestVerifyError {
    Format(FormatError),
    Check(CheckError),
    ArtifactHashMismatch,
    SemanticHashMismatch,
    ManifestMismatch,
}

impl ManifestVerifyError {
    pub const fn class(&self) -> ManifestVerifyErrorClass {
        match self {
            Self::Format(error) => match error.class() {
                FormatErrorClass::MalformedEncoding => ManifestVerifyErrorClass::MalformedEncoding,
                FormatErrorClass::UnsupportedVersion => {
                    ManifestVerifyErrorClass::UnsupportedVersion
                }
                FormatErrorClass::NoncanonicalArtifact => {
                    ManifestVerifyErrorClass::NoncanonicalArtifact
                }
                FormatErrorClass::ResourceExhausted => ManifestVerifyErrorClass::ResourceExhausted,
            },
            Self::Check(error) => match error.class() {
                CheckErrorClass::InvalidJudgment => ManifestVerifyErrorClass::InvalidJudgment,
                CheckErrorClass::ResourceExhausted => ManifestVerifyErrorClass::ResourceExhausted,
            },
            Self::ArtifactHashMismatch => ManifestVerifyErrorClass::ArtifactHashMismatch,
            Self::SemanticHashMismatch => ManifestVerifyErrorClass::SemanticHashMismatch,
            Self::ManifestMismatch => ManifestVerifyErrorClass::ManifestMismatch,
        }
    }
}

impl fmt::Display for ManifestVerifyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(error) => error.fmt(formatter),
            Self::Check(error) => error.fmt(formatter),
            Self::ArtifactHashMismatch => formatter.write_str("artifact-hash-mismatch"),
            Self::SemanticHashMismatch => formatter.write_str("semantic-hash-mismatch"),
            Self::ManifestMismatch => formatter.write_str("manifest-mismatch"),
        }
    }
}

impl std::error::Error for ManifestVerifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Format(error) => Some(error),
            Self::Check(error) => Some(error),
            Self::ArtifactHashMismatch | Self::SemanticHashMismatch | Self::ManifestMismatch => {
                None
            }
        }
    }
}

impl From<FormatError> for ManifestVerifyError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

impl From<CheckError> for ManifestVerifyError {
    fn from(error: CheckError) -> Self {
        Self::Check(error)
    }
}

/// Verify a supplied Foundation Manifest v0.1 against canonical Core v0.1 bytes.
///
/// The operation follows the frozen verification order: parse and require a
/// canonical Core artifact, recompute both hashes, check every declaration and
/// recompute the structural audit, parse/validate the supplied manifest, then
/// compare deterministic fields. Asserted provenance is structurally validated
/// by the manifest parser but is excluded from deterministic comparison.
///
/// On success the parsed manifest is returned so callers can report asserted
/// provenance separately without treating those historical claims as verified.
pub fn verify_foundation_manifest(
    core_bytes: &[u8],
    manifest_bytes: &[u8],
) -> Result<ParsedFoundationManifest, ManifestVerifyError> {
    let mut module = parse_canonical(core_bytes)?;
    let hashes = compute_module_hashes(&module)?;
    let audit = check_and_extract_audit(&mut module)?;
    let supplied = parse_foundation_manifest(manifest_bytes)?;

    if supplied.artifact_sha256() != hashes.artifact_sha256().as_str() {
        return Err(ManifestVerifyError::ArtifactHashMismatch);
    }
    if supplied.semantic_sha256() != hashes.semantic_sha256().as_str() {
        return Err(ManifestVerifyError::SemanticHashMismatch);
    }
    if !audit_matches(&audit, &supplied) {
        return Err(ManifestVerifyError::ManifestMismatch);
    }

    Ok(supplied)
}

fn audit_matches(expected: &FoundationAudit, supplied: &ParsedFoundationManifest) -> bool {
    let expected = expected.declarations();
    let supplied = supplied.declarations();
    expected.len() == supplied.len()
        && expected
            .iter()
            .zip(supplied)
            .all(|(expected, supplied)| declaration_matches(expected, supplied))
}

fn declaration_matches(
    expected: &DeclarationAudit,
    supplied: &super::ParsedDeclarationAudit,
) -> bool {
    expected.index() == supplied.index()
        && expected.display_name() == supplied.display_name()
        && expected.kind() == supplied.kind()
        && dependencies_match(expected.direct(), supplied.direct())
        && dependencies_match(expected.transitive(), supplied.transitive())
}

fn dependencies_match(expected: &AuditDependencies, supplied: &ParsedAuditDependencies) -> bool {
    expected.kernel_features() == supplied.kernel_features()
        && expected.extensions() == supplied.extensions()
        && expected.postulates() == supplied.postulates()
        && expected.declarations() == supplied.declarations()
}

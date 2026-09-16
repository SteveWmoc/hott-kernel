use core::fmt;

use crate::checker::{
    AuditDependencies, CheckError, FoundationAudit, KernelFeature, check_and_extract_audit,
};
use crate::error::FormatError;
use crate::format::{ARTIFACT_FORMAT, ModuleHashes, SEMANTIC_PROJECTION, compute_module_hashes};
use crate::syntax::Module;

mod parser;
mod verify;
pub use parser::{
    ParsedAuditDependencies, ParsedDeclarationAudit, ParsedFoundationManifest,
    ParsedManifestGenerator, ParsedProvenanceRecord, parse_foundation_manifest,
};
pub use verify::{ManifestVerifyError, ManifestVerifyErrorClass, verify_foundation_manifest};

pub const FOUNDATION_MANIFEST_SCHEMA: &str = "hott-foundation-manifest/0.1";
pub const KERNEL_THEORY: &str = "mltt-core";
pub const KERNEL_VERSION: &str = "0.1";

/// A complete generated Foundation Manifest v0.1 with no asserted provenance.
///
/// The deterministic artifact, kernel, and audit fields are recomputed from a
/// fully checked module. `asserted_provenance` is serialized as the empty array,
/// meaning that no historical production claim accompanies the artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationManifest {
    hashes: ModuleHashes,
    audit: FoundationAudit,
}

impl FoundationManifest {
    pub const fn schema(&self) -> &'static str {
        FOUNDATION_MANIFEST_SCHEMA
    }

    pub const fn artifact_format(&self) -> &'static str {
        ARTIFACT_FORMAT
    }

    pub const fn semantic_projection(&self) -> &'static str {
        SEMANTIC_PROJECTION
    }

    pub const fn kernel_theory(&self) -> &'static str {
        KERNEL_THEORY
    }

    pub const fn kernel_version(&self) -> &'static str {
        KERNEL_VERSION
    }

    pub const fn hashes(&self) -> &ModuleHashes {
        &self.hashes
    }

    pub const fn audit(&self) -> &FoundationAudit {
        &self.audit
    }
}

/// Failure while constructing deterministic manifest data from a module.
///
/// Logical rejection remains a checker error. Canonical-printing or hashing
/// failures remain format errors. The manifest layer does not relabel either.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestBuildError {
    Check(CheckError),
    Format(FormatError),
}

impl fmt::Display for ManifestBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Check(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ManifestBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Check(error) => Some(error),
            Self::Format(error) => Some(error),
        }
    }
}

impl From<CheckError> for ManifestBuildError {
    fn from(error: CheckError) -> Self {
        Self::Check(error)
    }
}

impl From<FormatError> for ManifestBuildError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

/// Check a complete Core v0.1 module and construct its deterministic manifest.
///
/// No manifest is produced for a logically invalid module. Hashing occurs only
/// after the complete checker and structural audit have succeeded. This
/// operation does not add any judgment beyond the frozen Core checker.
pub fn build_foundation_manifest(
    module: &mut Module,
) -> Result<FoundationManifest, ManifestBuildError> {
    let audit = check_and_extract_audit(module)?;
    let hashes = compute_module_hashes(module)?;
    Ok(FoundationManifest { hashes, audit })
}

/// Serialize a generated manifest as deterministic, schema-shaped UTF-8 JSON.
///
/// JSON object order and whitespace are not semantically significant in
/// Foundation Manifest v0.1, and manifest bytes are not hashed. This printer
/// nevertheless emits one stable representation for reproducible tooling.
pub fn print_foundation_manifest(manifest: &FoundationManifest) -> Result<Vec<u8>, FormatError> {
    let mut out = JsonBuffer::new();

    out.push_str("{\n")?;
    out.push_str("  \"schema\": \"")?;
    out.push_str(FOUNDATION_MANIFEST_SCHEMA)?;
    out.push_str("\",\n")?;

    out.push_str("  \"artifact\": {\n")?;
    out.push_str("    \"format\": \"")?;
    out.push_str(ARTIFACT_FORMAT)?;
    out.push_str("\",\n")?;
    out.push_str("    \"sha256\": \"")?;
    out.push_str(manifest.hashes.artifact_sha256().as_str())?;
    out.push_str("\",\n")?;
    out.push_str("    \"semantic_projection\": \"")?;
    out.push_str(SEMANTIC_PROJECTION)?;
    out.push_str("\",\n")?;
    out.push_str("    \"semantic_sha256\": \"")?;
    out.push_str(manifest.hashes.semantic_sha256().as_str())?;
    out.push_str("\"\n")?;
    out.push_str("  },\n")?;

    out.push_str("  \"kernel\": {\n")?;
    out.push_str("    \"theory\": \"")?;
    out.push_str(KERNEL_THEORY)?;
    out.push_str("\",\n")?;
    out.push_str("    \"version\": \"")?;
    out.push_str(KERNEL_VERSION)?;
    out.push_str("\"\n")?;
    out.push_str("  },\n")?;

    out.push_str("  \"audit\": {\n")?;
    out.push_str("    \"feature_vocabulary\": \"")?;
    out.push_str(manifest.audit.feature_vocabulary())?;
    out.push_str("\",\n")?;
    write_declarations(&mut out, manifest.audit.declarations())?;
    out.push_str("  },\n")?;
    out.push_str("  \"asserted_provenance\": []\n")?;
    out.push_str("}\n")?;

    Ok(out.into_bytes())
}

fn write_declarations(
    out: &mut JsonBuffer,
    declarations: &[crate::checker::DeclarationAudit],
) -> Result<(), FormatError> {
    if declarations.is_empty() {
        out.push_str("    \"declarations\": []\n")?;
        return Ok(());
    }

    out.push_str("    \"declarations\": [\n")?;
    for (position, declaration) in declarations.iter().enumerate() {
        out.push_str("      {\n")?;
        out.push_str("        \"index\": ")?;
        out.push_usize(declaration.index())?;
        out.push_str(",\n")?;
        out.push_str("        \"display_name\": ")?;
        write_json_string(out, declaration.display_name())?;
        out.push_str(",\n")?;
        out.push_str("        \"kind\": \"")?;
        out.push_str(declaration.kind().as_str())?;
        out.push_str("\",\n")?;
        out.push_str("        \"direct\": ")?;
        write_dependencies(out, declaration.direct(), 8)?;
        out.push_str(",\n")?;
        out.push_str("        \"transitive\": ")?;
        write_dependencies(out, declaration.transitive(), 8)?;
        out.push_str("\n      }")?;
        if position + 1 != declarations.len() {
            out.push_str(",")?;
        }
        out.push_str("\n")?;
    }
    out.push_str("    ]\n")
}

fn write_dependencies(
    out: &mut JsonBuffer,
    dependencies: &AuditDependencies,
    indent: usize,
) -> Result<(), FormatError> {
    out.push_str("{\n")?;
    push_indent(out, indent + 2)?;
    out.push_str("\"kernel_features\": ")?;
    write_feature_array(out, dependencies.kernel_features(), indent + 2)?;
    out.push_str(",\n")?;

    push_indent(out, indent + 2)?;
    out.push_str("\"extensions\": ")?;
    write_string_array(out, dependencies.extensions(), indent + 2)?;
    out.push_str(",\n")?;

    push_indent(out, indent + 2)?;
    out.push_str("\"postulates\": ")?;
    write_index_array(out, dependencies.postulates(), indent + 2)?;
    out.push_str(",\n")?;

    push_indent(out, indent + 2)?;
    out.push_str("\"declarations\": ")?;
    write_index_array(out, dependencies.declarations(), indent + 2)?;
    out.push_str("\n")?;
    push_indent(out, indent)?;
    out.push_str("}")
}

fn write_feature_array(
    out: &mut JsonBuffer,
    values: &[KernelFeature],
    indent: usize,
) -> Result<(), FormatError> {
    if values.is_empty() {
        return out.push_str("[]");
    }

    out.push_str("[\n")?;
    for (position, value) in values.iter().enumerate() {
        push_indent(out, indent + 2)?;
        out.push_str("\"")?;
        out.push_str(value.as_str())?;
        out.push_str("\"")?;
        if position + 1 != values.len() {
            out.push_str(",")?;
        }
        out.push_str("\n")?;
    }
    push_indent(out, indent)?;
    out.push_str("]")
}

fn write_string_array(
    out: &mut JsonBuffer,
    values: &[String],
    indent: usize,
) -> Result<(), FormatError> {
    if values.is_empty() {
        return out.push_str("[]");
    }

    out.push_str("[\n")?;
    for (position, value) in values.iter().enumerate() {
        push_indent(out, indent + 2)?;
        write_json_string(out, value)?;
        if position + 1 != values.len() {
            out.push_str(",")?;
        }
        out.push_str("\n")?;
    }
    push_indent(out, indent)?;
    out.push_str("]")
}

fn write_index_array(
    out: &mut JsonBuffer,
    values: &[usize],
    indent: usize,
) -> Result<(), FormatError> {
    if values.is_empty() {
        return out.push_str("[]");
    }

    out.push_str("[\n")?;
    for (position, value) in values.iter().copied().enumerate() {
        push_indent(out, indent + 2)?;
        out.push_usize(value)?;
        if position + 1 != values.len() {
            out.push_str(",")?;
        }
        out.push_str("\n")?;
    }
    push_indent(out, indent)?;
    out.push_str("]")
}

fn write_json_string(out: &mut JsonBuffer, value: &str) -> Result<(), FormatError> {
    out.push_str("\"")?;
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\"")?,
            '\\' => out.push_str("\\\\")?,
            _ => out.push_char(ch)?,
        }
    }
    out.push_str("\"")
}

fn push_indent(out: &mut JsonBuffer, count: usize) -> Result<(), FormatError> {
    for _ in 0..count {
        out.push_str(" ")?;
    }
    Ok(())
}

struct JsonBuffer(String);

impl JsonBuffer {
    const fn new() -> Self {
        Self(String::new())
    }

    fn push_str(&mut self, text: &str) -> Result<(), FormatError> {
        self.0
            .try_reserve(text.len())
            .map_err(|_| FormatError::resource_exhausted())?;
        self.0.push_str(text);
        Ok(())
    }

    fn push_char(&mut self, ch: char) -> Result<(), FormatError> {
        self.0
            .try_reserve(ch.len_utf8())
            .map_err(|_| FormatError::resource_exhausted())?;
        self.0.push(ch);
        Ok(())
    }

    fn push_usize(&mut self, mut value: usize) -> Result<(), FormatError> {
        if value == 0 {
            return self.push_str("0");
        }

        let mut digits = [0_u8; 3 * core::mem::size_of::<usize>()];
        let mut used = 0;
        while value != 0 {
            digits[used] = (value % 10) as u8;
            value /= 10;
            used += 1;
        }
        for digit in digits[..used].iter().rev() {
            self.push_char(char::from(b'0' + *digit))?;
        }
        Ok(())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }
}

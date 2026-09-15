use core::cmp::Ordering;

use crate::checker::{AuditDeclarationKind, FEATURE_VOCABULARY, KernelFeature};
use crate::error::FormatError;
use crate::format::{ARTIFACT_FORMAT, SEMANTIC_PROJECTION};

use super::{FOUNDATION_MANIFEST_SCHEMA, KERNEL_THEORY, KERNEL_VERSION};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFoundationManifest {
    artifact_sha256: String,
    semantic_sha256: String,
    declarations: Vec<ParsedDeclarationAudit>,
    asserted_provenance: Vec<ParsedProvenanceRecord>,
}

impl ParsedFoundationManifest {
    pub const fn schema(&self) -> &'static str {
        FOUNDATION_MANIFEST_SCHEMA
    }

    pub const fn artifact_format(&self) -> &'static str {
        ARTIFACT_FORMAT
    }

    pub fn artifact_sha256(&self) -> &str {
        &self.artifact_sha256
    }

    pub const fn semantic_projection(&self) -> &'static str {
        SEMANTIC_PROJECTION
    }

    pub fn semantic_sha256(&self) -> &str {
        &self.semantic_sha256
    }

    pub const fn kernel_theory(&self) -> &'static str {
        KERNEL_THEORY
    }

    pub const fn kernel_version(&self) -> &'static str {
        KERNEL_VERSION
    }

    pub const fn feature_vocabulary(&self) -> &'static str {
        FEATURE_VOCABULARY
    }

    pub fn declarations(&self) -> &[ParsedDeclarationAudit] {
        &self.declarations
    }

    pub fn asserted_provenance(&self) -> &[ParsedProvenanceRecord] {
        &self.asserted_provenance
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedDeclarationAudit {
    index: usize,
    display_name: String,
    kind: AuditDeclarationKind,
    direct: ParsedAuditDependencies,
    transitive: ParsedAuditDependencies,
}

impl ParsedDeclarationAudit {
    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn kind(&self) -> AuditDeclarationKind {
        self.kind
    }

    pub const fn direct(&self) -> &ParsedAuditDependencies {
        &self.direct
    }

    pub const fn transitive(&self) -> &ParsedAuditDependencies {
        &self.transitive
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedAuditDependencies {
    kernel_features: Vec<KernelFeature>,
    extensions: Vec<String>,
    postulates: Vec<usize>,
    declarations: Vec<usize>,
}

impl ParsedAuditDependencies {
    pub fn kernel_features(&self) -> &[KernelFeature] {
        &self.kernel_features
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    pub fn postulates(&self) -> &[usize] {
        &self.postulates
    }

    pub fn declarations(&self) -> &[usize] {
        &self.declarations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedProvenanceRecord {
    declaration: usize,
    generated_by: Vec<ParsedManifestGenerator>,
}

impl ParsedProvenanceRecord {
    pub const fn declaration(&self) -> usize {
        self.declaration
    }

    pub fn generated_by(&self) -> &[ParsedManifestGenerator] {
        &self.generated_by
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedManifestGenerator {
    kind: String,
    name: String,
    version: Option<String>,
    details: Option<String>,
}

impl ParsedManifestGenerator {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
}

/// Parse and structurally validate Foundation Manifest v0.1 JSON.
///
/// This operation validates UTF-8/JSON encoding, the checked-in v0.1 schema,
/// duplicate-key prohibition, Unicode-scalar decoding, and the additional
/// ordering/subsetting constraints of the frozen manifest specification. It
/// does not compare deterministic fields with a Core artifact; that is a
/// separate verification operation.
pub fn parse_foundation_manifest(bytes: &[u8]) -> Result<ParsedFoundationManifest, FormatError> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(FormatError::malformed(0));
    }
    let input =
        core::str::from_utf8(bytes).map_err(|error| FormatError::malformed(error.valid_up_to()))?;
    let mut parser = Parser::new(input);
    let manifest = parser.parse_manifest()?;
    parser.skip_whitespace();
    if !parser.is_end() {
        return Err(FormatError::malformed(parser.position()));
    }
    Ok(manifest)
}

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            position: 0,
        }
    }

    const fn position(&self) -> usize {
        self.position
    }

    fn is_end(&self) -> bool {
        self.position == self.bytes.len()
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn consume(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), FormatError> {
        if self.consume(byte) {
            Ok(())
        } else {
            Err(FormatError::malformed(self.position))
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.position += 1;
        }
    }

    fn parse_manifest(&mut self) -> Result<ParsedFoundationManifest, FormatError> {
        self.skip_whitespace();
        self.expect(b'{')?;
        self.skip_whitespace();

        let mut schema_seen = false;
        let mut artifact = None;
        let mut kernel_seen = false;
        let mut declarations = None;
        let mut asserted_provenance = None;

        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }

        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();

            match key.as_str() {
                "schema" => {
                    reject_duplicate(schema_seen, key_offset)?;
                    schema_seen = true;
                    self.parse_supported_string(FOUNDATION_MANIFEST_SCHEMA)?;
                }
                "artifact" => {
                    reject_duplicate(artifact.is_some(), key_offset)?;
                    artifact = Some(self.parse_artifact()?);
                }
                "kernel" => {
                    reject_duplicate(kernel_seen, key_offset)?;
                    kernel_seen = true;
                    self.parse_kernel()?;
                }
                "audit" => {
                    reject_duplicate(declarations.is_some(), key_offset)?;
                    declarations = Some(self.parse_audit()?);
                }
                "asserted_provenance" => {
                    reject_duplicate(asserted_provenance.is_some(), key_offset)?;
                    asserted_provenance = Some(self.parse_provenance_array()?);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }

            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }

        if !schema_seen || !kernel_seen {
            return Err(FormatError::malformed(self.position));
        }
        let (artifact_sha256, semantic_sha256) =
            artifact.ok_or_else(|| FormatError::malformed(self.position))?;
        let declarations = declarations.ok_or_else(|| FormatError::malformed(self.position))?;
        let asserted_provenance =
            asserted_provenance.ok_or_else(|| FormatError::malformed(self.position))?;

        validate_provenance_declarations(&asserted_provenance, declarations.len(), self.position)?;

        Ok(ParsedFoundationManifest {
            artifact_sha256,
            semantic_sha256,
            declarations,
            asserted_provenance,
        })
    }

    fn parse_artifact(&mut self) -> Result<(String, String), FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut format_seen = false;
        let mut sha256 = None;
        let mut projection_seen = false;
        let mut semantic_sha256 = None;

        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "format" => {
                    reject_duplicate(format_seen, key_offset)?;
                    format_seen = true;
                    self.parse_supported_string(ARTIFACT_FORMAT)?;
                }
                "sha256" => {
                    reject_duplicate(sha256.is_some(), key_offset)?;
                    let value_offset = self.position;
                    let value = self.parse_string()?;
                    validate_sha256(&value, value_offset)?;
                    sha256 = Some(value);
                }
                "semantic_projection" => {
                    reject_duplicate(projection_seen, key_offset)?;
                    projection_seen = true;
                    self.parse_supported_string(SEMANTIC_PROJECTION)?;
                }
                "semantic_sha256" => {
                    reject_duplicate(semantic_sha256.is_some(), key_offset)?;
                    let value_offset = self.position;
                    let value = self.parse_string()?;
                    validate_sha256(&value, value_offset)?;
                    semantic_sha256 = Some(value);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }

        if !format_seen || !projection_seen {
            return Err(FormatError::malformed(self.position));
        }
        Ok((
            sha256.ok_or_else(|| FormatError::malformed(self.position))?,
            semantic_sha256.ok_or_else(|| FormatError::malformed(self.position))?,
        ))
    }

    fn parse_kernel(&mut self) -> Result<(), FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut theory_seen = false;
        let mut version_seen = false;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "theory" => {
                    reject_duplicate(theory_seen, key_offset)?;
                    theory_seen = true;
                    self.parse_supported_string(KERNEL_THEORY)?;
                }
                "version" => {
                    reject_duplicate(version_seen, key_offset)?;
                    version_seen = true;
                    self.parse_supported_string(KERNEL_VERSION)?;
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }
        if theory_seen && version_seen {
            Ok(())
        } else {
            Err(FormatError::malformed(self.position))
        }
    }

    fn parse_audit(&mut self) -> Result<Vec<ParsedDeclarationAudit>, FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut vocabulary_seen = false;
        let mut declarations = None;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "feature_vocabulary" => {
                    reject_duplicate(vocabulary_seen, key_offset)?;
                    vocabulary_seen = true;
                    self.parse_supported_string(FEATURE_VOCABULARY)?;
                }
                "declarations" => {
                    reject_duplicate(declarations.is_some(), key_offset)?;
                    declarations = Some(self.parse_declaration_array()?);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }
        if !vocabulary_seen {
            return Err(FormatError::malformed(self.position));
        }
        declarations.ok_or_else(|| FormatError::malformed(self.position))
    }

    fn parse_declaration_array(&mut self) -> Result<Vec<ParsedDeclarationAudit>, FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut records = Vec::new();
        if self.consume(b']') {
            return Ok(records);
        }
        loop {
            let expected_index = records.len();
            let record = self.parse_declaration_record(expected_index, &records)?;
            push_vec(&mut records, record)?;
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b']')?;
            break;
        }
        Ok(records)
    }

    fn parse_declaration_record(
        &mut self,
        expected_index: usize,
        prior_records: &[ParsedDeclarationAudit],
    ) -> Result<ParsedDeclarationAudit, FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut index = None;
        let mut display_name = None;
        let mut kind = None;
        let mut direct = None;
        let mut transitive = None;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "index" => {
                    reject_duplicate(index.is_some(), key_offset)?;
                    index = Some(self.parse_index()?);
                }
                "display_name" => {
                    reject_duplicate(display_name.is_some(), key_offset)?;
                    let value_offset = self.position;
                    let value = self.parse_string()?;
                    validate_human_string(&value, value_offset)?;
                    display_name = Some(value);
                }
                "kind" => {
                    reject_duplicate(kind.is_some(), key_offset)?;
                    let value_offset = self.position;
                    let value = self.parse_string()?;
                    kind = Some(parse_declaration_kind(&value, value_offset)?);
                }
                "direct" => {
                    reject_duplicate(direct.is_some(), key_offset)?;
                    direct = Some(self.parse_dependencies()?);
                }
                "transitive" => {
                    reject_duplicate(transitive.is_some(), key_offset)?;
                    transitive = Some(self.parse_dependencies()?);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }

        let index = index.ok_or_else(|| FormatError::malformed(self.position))?;
        if index != expected_index {
            return Err(FormatError::malformed(self.position));
        }
        let direct = direct.ok_or_else(|| FormatError::malformed(self.position))?;
        let transitive = transitive.ok_or_else(|| FormatError::malformed(self.position))?;
        validate_dependencies_are_earlier(&direct, index, self.position)?;
        validate_dependencies_are_earlier(&transitive, index, self.position)?;
        validate_transitive_includes_direct(&direct, &transitive, self.position)?;
        validate_postulate_kinds(&direct, prior_records, self.position)?;
        validate_postulate_kinds(&transitive, prior_records, self.position)?;

        Ok(ParsedDeclarationAudit {
            index,
            display_name: display_name.ok_or_else(|| FormatError::malformed(self.position))?,
            kind: kind.ok_or_else(|| FormatError::malformed(self.position))?,
            direct,
            transitive,
        })
    }

    fn parse_dependencies(&mut self) -> Result<ParsedAuditDependencies, FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut kernel_features = None;
        let mut extensions_seen = false;
        let mut postulates = None;
        let mut declarations = None;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "kernel_features" => {
                    reject_duplicate(kernel_features.is_some(), key_offset)?;
                    kernel_features = Some(self.parse_feature_array()?);
                }
                "extensions" => {
                    reject_duplicate(extensions_seen, key_offset)?;
                    extensions_seen = true;
                    self.parse_empty_array()?;
                }
                "postulates" => {
                    reject_duplicate(postulates.is_some(), key_offset)?;
                    postulates = Some(self.parse_index_array()?);
                }
                "declarations" => {
                    reject_duplicate(declarations.is_some(), key_offset)?;
                    declarations = Some(self.parse_index_array()?);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }

        if !extensions_seen {
            return Err(FormatError::malformed(self.position));
        }
        let postulates = postulates.ok_or_else(|| FormatError::malformed(self.position))?;
        let declarations = declarations.ok_or_else(|| FormatError::malformed(self.position))?;
        if !is_sorted_subset(&postulates, &declarations) {
            return Err(FormatError::malformed(self.position));
        }
        Ok(ParsedAuditDependencies {
            kernel_features: kernel_features
                .ok_or_else(|| FormatError::malformed(self.position))?,
            extensions: Vec::new(),
            postulates,
            declarations,
        })
    }

    fn parse_feature_array(&mut self) -> Result<Vec<KernelFeature>, FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume(b']') {
            return Ok(values);
        }
        loop {
            let offset = self.position;
            let name = self.parse_string()?;
            let feature = parse_kernel_feature(&name, offset)?;
            if let Some(previous) = values.last().copied()
                && previous.as_str() >= feature.as_str()
            {
                return Err(FormatError::malformed(offset));
            }
            push_vec(&mut values, feature)?;
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b']')?;
            break;
        }
        Ok(values)
    }

    fn parse_index_array(&mut self) -> Result<Vec<usize>, FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume(b']') {
            return Ok(values);
        }
        loop {
            let offset = self.position;
            let value = self.parse_index()?;
            if values.last().is_some_and(|previous| *previous >= value) {
                return Err(FormatError::malformed(offset));
            }
            push_vec(&mut values, value)?;
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b']')?;
            break;
        }
        Ok(values)
    }

    fn parse_empty_array(&mut self) -> Result<(), FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        if self.consume(b']') {
            Ok(())
        } else {
            Err(FormatError::malformed(self.position))
        }
    }

    fn parse_provenance_array(&mut self) -> Result<Vec<ParsedProvenanceRecord>, FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut records = Vec::new();
        if self.consume(b']') {
            return Ok(records);
        }
        loop {
            let offset = self.position;
            let record = self.parse_provenance_record()?;
            if records
                .last()
                .is_some_and(|previous: &ParsedProvenanceRecord| {
                    previous.declaration >= record.declaration
                })
            {
                return Err(FormatError::malformed(offset));
            }
            push_vec(&mut records, record)?;
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b']')?;
            break;
        }
        Ok(records)
    }

    fn parse_provenance_record(&mut self) -> Result<ParsedProvenanceRecord, FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut declaration = None;
        let mut generated_by = None;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "declaration" => {
                    reject_duplicate(declaration.is_some(), key_offset)?;
                    declaration = Some(self.parse_index()?);
                }
                "generated_by" => {
                    reject_duplicate(generated_by.is_some(), key_offset)?;
                    generated_by = Some(self.parse_generator_array()?);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }
        Ok(ParsedProvenanceRecord {
            declaration: declaration.ok_or_else(|| FormatError::malformed(self.position))?,
            generated_by: generated_by.ok_or_else(|| FormatError::malformed(self.position))?,
        })
    }

    fn parse_generator_array(&mut self) -> Result<Vec<ParsedManifestGenerator>, FormatError> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut generators = Vec::new();
        if self.consume(b']') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let offset = self.position;
            let generator = self.parse_generator()?;
            if let Some(previous) = generators.last()
                && compare_generators(previous, &generator) != Ordering::Less
            {
                return Err(FormatError::malformed(offset));
            }
            push_vec(&mut generators, generator)?;
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b']')?;
            break;
        }
        Ok(generators)
    }

    fn parse_generator(&mut self) -> Result<ParsedManifestGenerator, FormatError> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut kind = None;
        let mut name = None;
        let mut version = None;
        let mut details = None;
        let mut version_seen = false;
        let mut details_seen = false;
        if self.consume(b'}') {
            return Err(FormatError::malformed(self.position.saturating_sub(1)));
        }
        loop {
            let key_offset = self.position;
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            match key.as_str() {
                "kind" => {
                    reject_duplicate(kind.is_some(), key_offset)?;
                    let offset = self.position;
                    let value = self.parse_string()?;
                    validate_identifier(&value, offset)?;
                    kind = Some(value);
                }
                "name" => {
                    reject_duplicate(name.is_some(), key_offset)?;
                    let offset = self.position;
                    let value = self.parse_string()?;
                    validate_human_string(&value, offset)?;
                    name = Some(value);
                }
                "version" => {
                    reject_duplicate(version_seen, key_offset)?;
                    version_seen = true;
                    let offset = self.position;
                    let value = self.parse_string()?;
                    validate_human_string(&value, offset)?;
                    version = Some(value);
                }
                "details" => {
                    reject_duplicate(details_seen, key_offset)?;
                    details_seen = true;
                    let offset = self.position;
                    let value = self.parse_string()?;
                    validate_human_string(&value, offset)?;
                    details = Some(value);
                }
                _ => return Err(FormatError::malformed(key_offset)),
            }
            self.skip_whitespace();
            if self.consume(b',') {
                self.skip_whitespace();
                continue;
            }
            self.expect(b'}')?;
            break;
        }
        Ok(ParsedManifestGenerator {
            kind: kind.ok_or_else(|| FormatError::malformed(self.position))?,
            name: name.ok_or_else(|| FormatError::malformed(self.position))?,
            version,
            details,
        })
    }

    fn parse_supported_string(&mut self, expected: &str) -> Result<(), FormatError> {
        let value = self.parse_string()?;
        if value == expected {
            Ok(())
        } else {
            Err(FormatError::unsupported())
        }
    }

    fn parse_string(&mut self) -> Result<String, FormatError> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let byte = self
                .peek()
                .ok_or_else(|| FormatError::malformed(self.position))?;
            match byte {
                b'"' => {
                    self.position += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.position += 1;
                    self.parse_escape(&mut out)?;
                }
                0x00..=0x1f => return Err(FormatError::malformed(self.position)),
                0x20..=0x7f => {
                    self.position += 1;
                    push_char(&mut out, char::from(byte))?;
                }
                _ => {
                    let ch = self.input[self.position..]
                        .chars()
                        .next()
                        .ok_or_else(|| FormatError::malformed(self.position))?;
                    self.position += ch.len_utf8();
                    push_char(&mut out, ch)?;
                }
            }
        }
    }

    fn parse_escape(&mut self, out: &mut String) -> Result<(), FormatError> {
        let escape_offset = self.position.saturating_sub(1);
        let escaped = self
            .peek()
            .ok_or_else(|| FormatError::malformed(self.position))?;
        self.position += 1;
        let ch = match escaped {
            b'"' => '"',
            b'\\' => '\\',
            b'/' => '/',
            b'b' => '\u{0008}',
            b'f' => '\u{000c}',
            b'n' => '\n',
            b'r' => '\r',
            b't' => '\t',
            b'u' => {
                let high = self.parse_hex_quad()?;
                if (0xd800..=0xdbff).contains(&high) {
                    if !self.consume(b'\\') || !self.consume(b'u') {
                        return Err(FormatError::malformed(escape_offset));
                    }
                    let low = self.parse_hex_quad()?;
                    if !(0xdc00..=0xdfff).contains(&low) {
                        return Err(FormatError::malformed(escape_offset));
                    }
                    let scalar =
                        0x1_0000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(low) - 0xdc00);
                    char::from_u32(scalar).ok_or_else(|| FormatError::malformed(escape_offset))?
                } else if (0xdc00..=0xdfff).contains(&high) {
                    return Err(FormatError::malformed(escape_offset));
                } else {
                    char::from_u32(u32::from(high))
                        .ok_or_else(|| FormatError::malformed(escape_offset))?
                }
            }
            _ => return Err(FormatError::malformed(escape_offset)),
        };
        push_char(out, ch)
    }

    fn parse_hex_quad(&mut self) -> Result<u16, FormatError> {
        let mut value = 0_u16;
        for _ in 0..4 {
            let byte = self
                .peek()
                .ok_or_else(|| FormatError::malformed(self.position))?;
            let digit = match byte {
                b'0'..=b'9' => u16::from(byte - b'0'),
                b'a'..=b'f' => u16::from(byte - b'a' + 10),
                b'A'..=b'F' => u16::from(byte - b'A' + 10),
                _ => return Err(FormatError::malformed(self.position)),
            };
            value = value * 16 + digit;
            self.position += 1;
        }
        Ok(value)
    }

    fn parse_index(&mut self) -> Result<usize, FormatError> {
        let start = self.position;
        let negative = self.consume(b'-');
        let int_start = self.position;
        match self.peek() {
            Some(b'0') => {
                self.position += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(FormatError::malformed(self.position));
                }
            }
            Some(b'1'..=b'9') => {
                self.position += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.position += 1;
                }
            }
            _ => return Err(FormatError::malformed(self.position)),
        }
        let int_end = self.position;

        let mut frac_start = self.position;
        let mut frac_end = self.position;
        if self.consume(b'.') {
            frac_start = self.position;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(FormatError::malformed(self.position));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.position += 1;
            }
            frac_end = self.position;
        }

        let mut exponent_negative = false;
        let mut exponent = 0_usize;
        let mut exponent_overflow = false;
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.position += 1;
            if self.consume(b'+') {
                // explicit positive exponent
            } else if self.consume(b'-') {
                exponent_negative = true;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(FormatError::malformed(self.position));
            }
            while let Some(byte @ b'0'..=b'9') = self.peek() {
                if !exponent_overflow {
                    if let Some(next) = exponent
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(usize::from(byte - b'0')))
                    {
                        exponent = next;
                    } else {
                        exponent_overflow = true;
                    }
                }
                self.position += 1;
            }
        }

        let integer = &self.bytes[int_start..int_end];
        let fraction = &self.bytes[frac_start..frac_end];
        decode_nonnegative_integer(
            integer,
            fraction,
            negative,
            exponent_negative,
            exponent,
            exponent_overflow,
            start,
        )
    }
}

fn reject_duplicate(duplicate: bool, offset: usize) -> Result<(), FormatError> {
    if duplicate {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn push_char(out: &mut String, ch: char) -> Result<(), FormatError> {
    out.try_reserve(ch.len_utf8())
        .map_err(|_| FormatError::resource_exhausted())?;
    out.push(ch);
    Ok(())
}

fn push_vec<T>(values: &mut Vec<T>, value: T) -> Result<(), FormatError> {
    values
        .try_reserve(1)
        .map_err(|_| FormatError::resource_exhausted())?;
    values.push(value);
    Ok(())
}

fn validate_sha256(value: &str, offset: usize) -> Result<(), FormatError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(FormatError::malformed(offset))
    }
}

fn validate_human_string(value: &str, offset: usize) -> Result<(), FormatError> {
    if value.is_empty()
        || value
            .chars()
            .any(|ch| matches!(ch as u32, 0x0000..=0x001f | 0x007f..=0x009f))
    {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn validate_identifier(value: &str, offset: usize) -> Result<(), FormatError> {
    let bytes = value.as_bytes();
    if bytes.first().is_none_or(|byte| !byte.is_ascii_lowercase()) {
        return Err(FormatError::malformed(offset));
    }
    let mut previous_separator = false;
    for byte in bytes.iter().copied().skip(1) {
        match byte {
            b'a'..=b'z' | b'0'..=b'9' => previous_separator = false,
            b'.' | b'-' if !previous_separator => previous_separator = true,
            _ => return Err(FormatError::malformed(offset)),
        }
    }
    if previous_separator {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn parse_kernel_feature(value: &str, offset: usize) -> Result<KernelFeature, FormatError> {
    match value {
        "empty" => Ok(KernelFeature::Empty),
        "identity" => Ok(KernelFeature::Identity),
        "natural-numbers" => Ok(KernelFeature::NaturalNumbers),
        "pi" => Ok(KernelFeature::Pi),
        "sigma" => Ok(KernelFeature::Sigma),
        "unit" => Ok(KernelFeature::Unit),
        "universe" => Ok(KernelFeature::Universe),
        _ => Err(FormatError::malformed(offset)),
    }
}

fn parse_declaration_kind(value: &str, offset: usize) -> Result<AuditDeclarationKind, FormatError> {
    match value {
        "postulate" => Ok(AuditDeclarationKind::Postulate),
        "transparent" => Ok(AuditDeclarationKind::Transparent),
        "opaque" => Ok(AuditDeclarationKind::Opaque),
        _ => Err(FormatError::malformed(offset)),
    }
}

fn is_sorted_subset<T: Ord>(subset: &[T], superset: &[T]) -> bool {
    let mut cursor = 0;
    for value in subset {
        while cursor < superset.len() && superset[cursor].cmp(value) == Ordering::Less {
            cursor += 1;
        }
        if cursor == superset.len() || superset[cursor].cmp(value) != Ordering::Equal {
            return false;
        }
    }
    true
}

fn validate_transitive_includes_direct(
    direct: &ParsedAuditDependencies,
    transitive: &ParsedAuditDependencies,
    offset: usize,
) -> Result<(), FormatError> {
    if !is_sorted_subset(&direct.kernel_features, &transitive.kernel_features)
        || !is_sorted_subset(&direct.extensions, &transitive.extensions)
        || !is_sorted_subset(&direct.postulates, &transitive.postulates)
        || !is_sorted_subset(&direct.declarations, &transitive.declarations)
    {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn validate_postulate_kinds(
    dependencies: &ParsedAuditDependencies,
    prior_records: &[ParsedDeclarationAudit],
    offset: usize,
) -> Result<(), FormatError> {
    for dependency in &dependencies.postulates {
        if prior_records
            .get(*dependency)
            .is_none_or(|record| record.kind != AuditDeclarationKind::Postulate)
        {
            return Err(FormatError::malformed(offset));
        }
    }
    Ok(())
}

fn validate_dependencies_are_earlier(
    dependencies: &ParsedAuditDependencies,
    declaration: usize,
    offset: usize,
) -> Result<(), FormatError> {
    if dependencies
        .declarations
        .last()
        .is_some_and(|dependency| *dependency >= declaration)
        || dependencies
            .postulates
            .last()
            .is_some_and(|dependency| *dependency >= declaration)
    {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn validate_provenance_declarations(
    records: &[ParsedProvenanceRecord],
    declaration_count: usize,
    offset: usize,
) -> Result<(), FormatError> {
    if records
        .last()
        .is_some_and(|record| record.declaration >= declaration_count)
    {
        Err(FormatError::malformed(offset))
    } else {
        Ok(())
    }
}

fn compare_generators(left: &ParsedManifestGenerator, right: &ParsedManifestGenerator) -> Ordering {
    (
        left.kind.as_str(),
        left.name.as_str(),
        left.version.as_deref().unwrap_or(""),
        left.details.as_deref().unwrap_or(""),
    )
        .cmp(&(
            right.kind.as_str(),
            right.name.as_str(),
            right.version.as_deref().unwrap_or(""),
            right.details.as_deref().unwrap_or(""),
        ))
}

fn decode_nonnegative_integer(
    integer: &[u8],
    fraction: &[u8],
    negative: bool,
    exponent_negative: bool,
    exponent: usize,
    exponent_overflow: bool,
    offset: usize,
) -> Result<usize, FormatError> {
    let total_digits = integer.len() + fraction.len();
    let all_zero = digits_all_zero(integer, fraction);
    if all_zero {
        return Ok(0);
    }
    if negative {
        return Err(FormatError::malformed(offset));
    }

    if exponent_negative {
        if exponent_overflow {
            return Err(FormatError::malformed(offset));
        }
        let Some(discard) = fraction.len().checked_add(exponent) else {
            return Err(FormatError::malformed(offset));
        };
        if discard > total_digits || !trailing_digits_zero(integer, fraction, discard) {
            return Err(FormatError::malformed(offset));
        }
        parse_digit_prefix(integer, fraction, total_digits - discard)
    } else if exponent_overflow {
        Err(FormatError::resource_exhausted())
    } else if exponent >= fraction.len() {
        let scale = exponent - fraction.len();
        let mut value = parse_digit_prefix(integer, fraction, total_digits)?;
        if scale > 3 * core::mem::size_of::<usize>() {
            return Err(FormatError::resource_exhausted());
        }
        for _ in 0..scale {
            value = value
                .checked_mul(10)
                .ok_or_else(FormatError::resource_exhausted)?;
        }
        Ok(value)
    } else {
        let discard = fraction.len() - exponent;
        if !trailing_digits_zero(integer, fraction, discard) {
            return Err(FormatError::malformed(offset));
        }
        parse_digit_prefix(integer, fraction, total_digits - discard)
    }
}

fn digits_all_zero(integer: &[u8], fraction: &[u8]) -> bool {
    integer.iter().chain(fraction).all(|digit| *digit == b'0')
}

fn trailing_digits_zero(integer: &[u8], fraction: &[u8], count: usize) -> bool {
    let total = integer.len() + fraction.len();
    if count > total {
        return false;
    }
    (total - count..total).all(|index| combined_digit(integer, fraction, index) == b'0')
}

fn parse_digit_prefix(integer: &[u8], fraction: &[u8], count: usize) -> Result<usize, FormatError> {
    let mut value = 0_usize;
    for index in 0..count {
        let digit = usize::from(combined_digit(integer, fraction, index) - b'0');
        value = value
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or_else(FormatError::resource_exhausted)?;
    }
    Ok(value)
}

fn combined_digit(integer: &[u8], fraction: &[u8], index: usize) -> u8 {
    if index < integer.len() {
        integer[index]
    } else {
        fraction[index - integer.len()]
    }
}

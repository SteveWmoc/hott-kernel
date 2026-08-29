use core::fmt;

/// Stable public classification of format-layer failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FormatErrorClass {
    MalformedEncoding,
    UnsupportedVersion,
    NoncanonicalArtifact,
    ResourceExhausted,
}

/// A format-layer failure.
///
/// Diagnostic detail is intentionally not part of the stable interchange
/// vocabulary. Consumers should branch on [`FormatError::class`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatError {
    class: FormatErrorClass,
    offset: Option<usize>,
}

impl FormatError {
    pub const fn class(&self) -> FormatErrorClass {
        self.class
    }

    /// Byte offset of the detected encoding problem, when meaningful.
    ///
    /// The offset is diagnostic only and is not serialized by Core v0.1.
    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }

    pub(crate) const fn malformed(offset: usize) -> Self {
        Self {
            class: FormatErrorClass::MalformedEncoding,
            offset: Some(offset),
        }
    }

    pub(crate) const fn unsupported() -> Self {
        Self {
            class: FormatErrorClass::UnsupportedVersion,
            offset: None,
        }
    }

    pub(crate) const fn noncanonical() -> Self {
        Self {
            class: FormatErrorClass::NoncanonicalArtifact,
            offset: None,
        }
    }

    pub(crate) const fn resource_exhausted() -> Self {
        Self {
            class: FormatErrorClass::ResourceExhausted,
            offset: None,
        }
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.class {
            FormatErrorClass::MalformedEncoding => "malformed-encoding",
            FormatErrorClass::UnsupportedVersion => "unsupported-version",
            FormatErrorClass::NoncanonicalArtifact => "noncanonical-artifact",
            FormatErrorClass::ResourceExhausted => "resource-exhausted",
        };
        f.write_str(label)
    }
}

impl std::error::Error for FormatError {}

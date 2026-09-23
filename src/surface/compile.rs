use core::fmt;

use crate::error::FormatError;
use crate::format::print_canonical;
use crate::surface::{SurfaceError, elaborate_surface, parse_surface};

/// Failure while compiling Surface v0.1 source to canonical Core v0.1 bytes.
///
/// Surface parsing and name-resolution failures remain Surface errors.
/// Canonical Core serialization failures remain format errors; the compiler
/// does not relabel failures from either layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceCompileError {
    Surface(SurfaceError),
    Format(FormatError),
}

impl fmt::Display for SurfaceCompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SurfaceCompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Surface(error) => Some(error),
            Self::Format(error) => Some(error),
        }
    }
}

impl From<SurfaceError> for SurfaceCompileError {
    fn from(error: SurfaceError) -> Self {
        Self::Surface(error)
    }
}

impl From<FormatError> for SurfaceCompileError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

/// Compile one Surface v0.1 source file to canonical Core v0.1 bytes.
///
/// This operation performs exactly three untrusted/tooling steps:
///
/// 1. strict Surface parsing;
/// 2. deterministic name resolution and structural elaboration;
/// 3. canonical Core serialization.
///
/// It does not invoke the Core checker and therefore does not claim that the
/// emitted artifact is a valid Core judgment.
pub fn compile_surface(input: &[u8]) -> Result<Vec<u8>, SurfaceCompileError> {
    let surface = parse_surface(input)?;
    let core = elaborate_surface(&surface)?;
    Ok(print_canonical(&core)?)
}

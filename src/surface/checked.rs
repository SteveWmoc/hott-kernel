use core::fmt;

use crate::checker::{CheckError, check_module};
use crate::error::FormatError;
use crate::format::print_canonical;
use crate::surface::{SurfaceError, elaborate_surface, parse_surface};

/// Failure while compiling and checking Surface v0.1 source.
///
/// The wrapper preserves the layer that rejected the input:
///
/// - `SurfaceCheckError::Surface` for Surface parsing/name-resolution failures;
/// - `SurfaceCheckError::Check` for logical Core rejection;
/// - `SurfaceCheckError::Format` for canonical Core serialization failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceCheckError {
    Surface(SurfaceError),
    Check(CheckError),
    Format(FormatError),
}

impl fmt::Display for SurfaceCheckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface(error) => error.fmt(formatter),
            Self::Check(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SurfaceCheckError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Surface(error) => Some(error),
            Self::Check(error) => Some(error),
            Self::Format(error) => Some(error),
        }
    }
}

impl From<SurfaceError> for SurfaceCheckError {
    fn from(error: SurfaceError) -> Self {
        Self::Surface(error)
    }
}

impl From<CheckError> for SurfaceCheckError {
    fn from(error: CheckError) -> Self {
        Self::Check(error)
    }
}

impl From<FormatError> for SurfaceCheckError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

/// Compile Surface v0.1 source, require Core logical validity, and return
/// canonical Core v0.1 bytes.
///
/// This convenience wrapper adds no typing or conversion rule. It delegates
/// logical validity exactly to the existing frozen Core checker:
///
/// 1. parse Surface v0.1;
/// 2. resolve names and elaborate structurally to Core;
/// 3. call `check_module` on that Core module;
/// 4. serialize the same module with the canonical Core printer.
///
/// `check_module` restores all checker scratch nodes before returning, so the
/// bytes emitted after a successful check are the same canonical artifact that
/// the unchecked Surface compiler would emit.
pub fn compile_and_check_surface(input: &[u8]) -> Result<Vec<u8>, SurfaceCheckError> {
    let surface = parse_surface(input)?;
    let mut core = elaborate_surface(&surface)?;
    check_module(&mut core)?;
    Ok(print_canonical(&core)?)
}

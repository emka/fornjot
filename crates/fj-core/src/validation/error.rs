use std::{convert::Infallible, fmt};

use crate::validate::{
    EdgeValidationError, FaceValidationError, ShellValidationError,
    SketchValidationError, SolidValidationError,
};

use super::checks::AdjacentHalfEdgesNotConnected;

/// An error that can occur during a validation
#[derive(Clone, Debug, thiserror::Error)]
pub enum ValidationError {
    /// `HalfEdge`s in `Cycle` not connected
    #[error(transparent)]
    HalfEdgesInCycleNotConnected(#[from] AdjacentHalfEdgesNotConnected),

    /// `Edge` validation error
    #[error("`Edge` validation error")]
    Edge(#[from] EdgeValidationError),

    /// `Face` validation error
    #[error("`Face` validation error")]
    Face(#[from] FaceValidationError),

    /// `Shell` validation error
    #[error("`Shell` validation error")]
    Shell(#[from] ShellValidationError),

    /// `Solid` validation error
    #[error("`Solid` validation error")]
    Solid(#[from] SolidValidationError),

    /// `Sketch` validation error
    #[error("`Sketch` validation error")]
    Sketch(#[from] SketchValidationError),
}

impl From<Infallible> for ValidationError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}

/// A collection of validation errors
#[derive(Debug, thiserror::Error)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_errors = self.0.len();

        writeln!(f, "{num_errors} unhandled validation errors:")?;

        for err in &self.0 {
            writeln!(f, "{err}")?;
        }

        Ok(())
    }
}

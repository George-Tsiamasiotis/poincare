//! Custom error types.

use std::path::PathBuf;

#[derive(thiserror::Error)]
pub enum NcError {
    /// Catches errors from the wrapped netcdf library.
    /// Lower level error: netcdf::Error::Netcdf.
    #[error("Wrapped library Error: {reason} ({source}).")]
    LibraryError {
        #[source]
        source: netcdf::Error,
        reason: Box<str>,
    },

    /// Given path is not a file.
    #[error("Error: '{0}': File not found.")]
    FileNotFound(PathBuf),

    /// Variable does not exist.
    #[error("Error: '{0}' variable not found.")]
    VariableNotFound(Box<str>),

    /// Variable exists, but has no data.
    #[error("Error: '{0}' variable is empty.")]
    EmptyVariable(Box<str>),

    /// Expected scalar value, found array.
    #[error("Error: '{0}' variable is not scalar.")]
    NotScalar(Box<str>),

    /// Supplied wrong `Extends` dimensionality to `get_values()`.
    /// Lower level error: netcdf::Error::DimensionalityMismatch.
    #[error("Error extracting '{field}': {source}.")]
    DimensionError {
        #[source]
        source: netcdf::Error,
        field: Box<str>,
    },
}

impl std::fmt::Debug for NcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}", self)
    }
}

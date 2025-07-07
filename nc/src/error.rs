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
}

impl std::fmt::Debug for NcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}", self)
    }
}

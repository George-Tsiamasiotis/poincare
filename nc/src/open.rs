//! Handles NetCDF file opening.

use std::path::PathBuf;

use crate::NcError;
use crate::scalars::Scalars;

#[derive(Debug)]
/// NetCDF equilibrium data.
pub struct NcData {
    /// Path to NetCDF file.
    pub path: PathBuf,
    pub scalars: Scalars,
}

impl NcData {
    /// Creates an NcData from a NetCDF file.
    pub fn open(path: PathBuf) -> Result<Self, NcError> {
        use NcError::*;

        if !path.exists() {
            return Err(FileNotFound(path));
        }

        // If this fails, its due to an underlying library error.
        let nc_file = match netcdf::open(&path) {
            Ok(nc_file) => nc_file,
            Err(liberror) => {
                return Err(LibraryError {
                    source: liberror, // Error::Netcdf
                    reason: "Error opening NetCDF file".into(),
                });
            }
        };

        let scalars = Scalars::build(&nc_file)?;

        let rec = NcData { path, scalars };

        Ok(rec)
    }
}

#[cfg(test)]
mod test {
    use crate::NcData;
    use crate::NcError;

    #[test]
    fn file_not_found() {
        let x: Result<NcData, NcError> = NcData::open("not_an_existing_path".into());
        assert!(x.is_err());
        assert!(matches!(x.err().unwrap(), NcError::FileNotFound { .. }));
    }
}

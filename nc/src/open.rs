//! Handles NetCDF file opening and `NcData` creation.

use std::path::PathBuf;

use crate::NcError;
use crate::bfield::Bfield;
use crate::coords::Coords;
use crate::currents::Currents;
use crate::scalars::Scalars;

#[derive(Debug)]
/// NetCDF equilibrium data.
pub struct NcData {
    /// Path to NetCDF file.
    pub path: PathBuf,
    /// Equilibrium's scalar values.
    pub scalars: Scalars,
    /// Equilibrium's coordinate variables.
    pub coords: Coords,
    /// Plasma toroidal (I) and poloidal (g) currents.
    pub currents: Currents,
    /// Magnetic field strength.
    pub bfield: Bfield,
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
        let coords = Coords::build(&nc_file)?;
        let currents = Currents::build(&nc_file)?;
        let bfield = Bfield::build(&nc_file)?;

        let rec = NcData {
            path,
            scalars,
            coords,
            currents,
            bfield,
        };

        Ok(rec)
    }
}

impl std::fmt::Display for NcData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NcData:")?;
        write!(f, "{}", self.scalars)?;
        write!(f, "{}", self.coords)?;
        write!(f, "{}", self.currents)
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
        assert!(matches!(x.unwrap_err(), NcError::FileNotFound { .. }));
    }
}

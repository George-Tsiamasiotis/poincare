//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices

mod vars;

use crate::vars::coords::Coords;
use crate::vars::scalars::Scalars;
use std::path::PathBuf;

#[derive(Debug)]
/// The reconstructed equilibria data
pub struct Nc {
    pub path: PathBuf,
    pub scalars: Scalars,
    pub coords: Coords,
}

impl Nc {
    /// Creates a `Scalars` by extracting data from `f`.
    pub fn open(path: PathBuf) -> Result<Nc, netcdf::Error> {
        let netcdf_file = match netcdf::open(path.to_str().expect("path not found")) {
            Ok(nc_file) => nc_file,
            Err(error) => return Err(error),
        };

        let nc = Nc {
            path,
            scalars: Scalars::from_netcdf_file(&netcdf_file),
            coords: Coords::from_netcdf_file(&netcdf_file),
        };

        Ok(nc)
    }
}

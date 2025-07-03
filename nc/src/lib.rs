//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices

mod vars;

use crate::vars::coords::Coords;
use crate::vars::currents::Currents;
use crate::vars::scalars::Scalars;
use std::path::PathBuf;

#[derive(Debug)]
/// The reconstructed equilibria data
pub struct Nc {
    pub path: PathBuf,
    pub scalars: Scalars,
    pub coords: Coords,
    pub currents: Currents,
}

impl Nc {
    /// Creates a `Scalars` by extracting data from `f`.
    pub fn open(path: PathBuf) -> Result<Nc, netcdf::Error> {
        let netcdf_file = netcdf::open(path.to_str().expect("Path not found"))?;

        let nc = Nc {
            path,
            scalars: Scalars::from_netcdf_file(&netcdf_file),
            coords: Coords::from_netcdf_file(&netcdf_file),
            currents: Currents::from_netcdf_file(&netcdf_file),
        };

        Nc::check_dims(&nc);
        Ok(nc)
    }

    fn check_dims(&self) {
        let psi_len = self.coords.psi_len;
        if self.currents.g_len != psi_len {
            panic!(
                "'g' dimensions do not match (psi_len = {}, g_len = {})",
                psi_len, self.currents.g_len
            )
        }
        if self.currents.i_len != psi_len {
            panic!(
                "'i' dimensions do not match (psi_len = {}, i_len = {})",
                psi_len, self.currents.i_len
            )
        }
    }
}

/// Extracts all values from a coord.
pub(crate) fn extract_1d_var(f: &netcdf::File, field: &str) -> Result<Vec<f64>, netcdf::Error> {
    let err_msg = String::from("'") + field + "' field not found in NetCDF file.";
    let var = match f.variable(field) {
        Some(coord) => coord,
        None => return Err(err_msg.into()),
    };
    let values: Vec<f64> = var.get_values(..)?;
    Ok(values)
}

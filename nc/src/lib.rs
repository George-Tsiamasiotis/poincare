//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices

mod vars;

use ndarray::{Array, Ix2};
use std::process;

// use crate::vars::bfield::Bfield;
use crate::vars::bfield::Bfield;
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
    pub bfield: Bfield,
}

impl Nc {
    /// Creates a `Scalars` by extracting data from `f`.
    pub fn open(path: PathBuf) -> Result<Nc, &'static str> {
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return Err("path does not exist."),
        };
        let netcdf_file = match netcdf::open(path_str) {
            Ok(file) => file,
            Err(_) => return Err("Error opening NetCDF file."),
        };

        let err_msg = String::from("Error creating Nc: ");
        let nc = Nc {
            path,
            scalars: Scalars::build(&netcdf_file).unwrap_or_else(|err| {
                println!("{}{}", &err_msg, err.0 + err.1);
                process::exit(1);
            }),
            coords: Coords::from_netcdf_file(&netcdf_file),
            currents: Currents::from_netcdf_file(&netcdf_file),
            bfield: Bfield::from_netcdf_file(&netcdf_file),
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

// Returns a 2D vector, containing var(ψ, θ)
pub(crate) fn extract_2d_var(
    f: &netcdf::File,
    field: &str,
) -> Result<Vec<Vec<f64>>, netcdf::Error> {
    // Shape must be (psi_len, theta_len)
    let theta_len = f.variable("boozer_theta").unwrap().len();
    let psi_len = f.variable("psi").unwrap().len();
    let shape = (psi_len, theta_len);

    // Store in an array first and then convert to 2D vec
    let mut data: Array<f64, Ix2> = Array::<f64, Ix2>::zeros(shape);

    // Store data to 2D array
    f.variable(field)
        .unwrap()
        .get_into(data.view_mut(), (.., ..))
        .unwrap();

    // Store data to 2D vec
    let mut data_vec: Vec<Vec<f64>> = Vec::with_capacity(psi_len);
    for row in data.rows() {
        data_vec.push(row.to_vec());
    }
    Ok(data_vec)
}

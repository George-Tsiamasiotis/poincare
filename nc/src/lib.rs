use std::path::PathBuf;

use ndarray::{Array, Ix1};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Scalars {
    pub Baxis: f64,
    pub raxis: f64,
    pub psi_wall: f64,
}

#[derive(Debug)]
pub struct Coords {
    pub boozer_theta: Array<f64, Ix1>,
    pub psi: Array<f64, Ix1>,
}

#[derive(Debug)]
pub struct Nc {
    pub path: PathBuf,
    pub scalars: Scalars,
    pub coords: Coords,
}

impl Scalars {
    fn from_netcdf_file(f: &netcdf::File) -> Self {
        Scalars {
            Baxis: f
                .variable("Baxis")
                .expect("'Baxis' field not found.")
                .get::<f64, _>(..)
                .expect("'Baxis is not a ")
                .first()
                .expect("'Baxis' field is empty.")
                .clone(),
            raxis: f
                .variable("raxis")
                .expect("'raxis' field not found.")
                .get::<f64, _>(..)
                .expect("'raxis is not a numerical variable")
                .first()
                .expect("'raxis' field is empty.")
                .clone(),
            psi_wall: {
                let psi_var = f.variable("psi").expect("'psi' variable not found.");
                let psi_values = psi_var
                    .get::<f64, _>(..)
                    .expect("Error extracting values from 'psi' variable.");
                // Assume that 'psi' is ordered (as it should be)
                psi_values.last().expect("'psi' coord is empty.").clone()
            },
        }
    }
}

impl Coords {
    fn from_netcdf_file(f: &netcdf::File) -> Self {
        let boozer_theta_var = f
            .variable("boozer_theta")
            .expect("'boozer_theta' coord not found.");
        let psi_var = f.variable("psi").expect("'psi' coord not found.");

        let boozer_theta: Vec<f64> = boozer_theta_var
            .get_values(..)
            .expect("Error extracting 'boozer_theta' coord");
        let psi: Vec<f64> = psi_var
            .get_values(..)
            .expect("Error extracting 'psi' coord");
        Coords {
            boozer_theta: Array::from_vec(boozer_theta),
            psi: Array::from_vec(psi),
        }
    }
}

impl Nc {
    pub fn open(path: PathBuf) -> Result<Nc, netcdf::Error> {
        let netcdf_file = match netcdf::open(path.to_str().expect("path not found")) {
            Ok(nc_file) => nc_file,
            Err(error) => return Err(error),
        };

        let scalars = Scalars::from_netcdf_file(&netcdf_file);
        let coords = Coords::from_netcdf_file(&netcdf_file);

        let nc = Nc {
            path,
            scalars,
            coords,
        };

        Ok(nc)
    }
}

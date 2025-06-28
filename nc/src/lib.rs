use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Scalars {
    pub Baxis: f64,
    pub raxis: f64,
    pub psi_wall: f64,
}

#[derive(Debug)]
pub struct Coords {
    boozer_theta: Vec<f64>,
    psi: Vec<f64>,
}

#[derive(Debug)]
pub struct Nc {
    pub path: PathBuf,
    pub scalars: Scalars,
}

impl Scalars {
    fn from_netcdf_file(f: netcdf::File) -> Self {
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

impl Nc {
    pub fn open(path: PathBuf) -> Result<Nc, netcdf::Error> {
        let netcdf_file = match netcdf::open(path.to_str().expect("path not found")) {
            Ok(nc_file) => nc_file,
            Err(error) => return Err(error),
        };

        let scalars = Scalars::from_netcdf_file(netcdf_file);

        let nc = Nc { scalars, path };

        Ok(nc)
    }
}

use ndarray::{Array, Ix1};

// TODO: Error handling
// TODO: Display

#[derive(Debug)]
/// The $\theta$ and $\psi$ coordinates data points.
pub struct Coords {
    pub boozer_theta: Array<f64, Ix1>,
    pub psi: Array<f64, Ix1>,
}

impl Coords {
    /// Creates a `Coords` by extracting data from `f`.
    pub fn from_netcdf_file(f: &netcdf::File) -> Self {
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

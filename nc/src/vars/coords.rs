//! Representation of an equilibrium's theta and psi coordinates

use crate::extract_1d_var;

#[derive(Debug)]
/// The $\theta$ and $\psi$ coordinates data points.
pub struct Coords {
    pub boozer_theta: Vec<f64>,
    pub psi: Vec<f64>,
    pub boozer_theta_len: usize,
    pub psi_len: usize,
    pub boozer_theta_min: f64,
    pub boozer_theta_max: f64,
    pub psi_min: f64,
    pub psi_max: f64,
}

impl Coords {
    /// Creates a `Coords` by extracting data from `f`.
    pub fn from_netcdf_file(f: &netcdf::File) -> Coords {
        let boozer_theta: Vec<f64> = extract_1d_var(f, "boozer_theta").unwrap();
        let boozer_theta_len = boozer_theta.len();
        let psi: Vec<f64> = extract_1d_var(f, "psi").unwrap();
        let psi_len = psi.len();

        // 'boozer_theta' and 'psi' should be sorted
        let boozer_theta_min = *boozer_theta.first().unwrap();
        let boozer_theta_max = *boozer_theta.last().unwrap();
        let psi_min = *psi.first().unwrap();
        let psi_max = *psi.last().unwrap();
        Coords {
            boozer_theta,
            psi,
            boozer_theta_len,
            psi_len,
            boozer_theta_min,
            boozer_theta_max,
            psi_min,
            psi_max,
        }
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Coords:")?;
        writeln!(
            f,
            "\tboozer_theta = [{:.5}, ..., {:.5}], len = {},",
            self.boozer_theta_min, self.boozer_theta_max, self.boozer_theta_len,
        )?;
        writeln!(
            f,
            "\t         psi = [{:.5}, ..., {:.5}], len = {},",
            self.psi_min, self.psi_max, self.psi_len,
        )
    }
}

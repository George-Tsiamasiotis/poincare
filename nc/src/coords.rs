//! `Coords` implementation.

use crate::{
    NcError,
    extract::{extract_1d_var, extract_var_with_axis_value},
};

#[derive(Debug)]
/// Representation of the equilibrium's `psi` and `boozer_theta` coordinates.
pub struct Coords {
    /// The ψ coordinate.
    pub psi: Vec<f64>,
    /// The θ coordinate.
    pub theta: Vec<f64>,
    /// The ψ coordinate's length.
    pub psi_len: usize,
    /// The θ coordinate's length.
    pub theta_len: usize,
    /// The ψ coordinate's span (min, max).
    pub psi_span: (f64, f64),
    /// The θ coordinate's span (min, max).
    pub theta_span: (f64, f64),
}

impl Coords {
    /// Creates a `Coords` containing the coordinate variables from the NetCDF file.
    pub(crate) fn build(f: &netcdf::File) -> Result<Self, NcError> {
        // Extrapolate psi to later extrapolate all other variables to include a value
        // at the axis.
        let psi: Vec<f64> = extract_var_with_axis_value(f, "psi", 0.0)?;
        let theta = extract_1d_var(f, "boozer_theta")?;

        let psi_len = psi.len();
        let theta_len = theta.len();

        // Safe unwrap(); both psi and theta have already been checked.
        let psi_span = (*psi.first().unwrap(), *psi.last().unwrap());
        let theta_span = (*theta.first().unwrap(), *theta.last().unwrap());

        Ok(Coords {
            psi,
            theta,
            psi_len,
            theta_len,
            psi_span,
            theta_span,
        })
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Coords:")?;
        writeln!(
            f,
            "\ttheta = [{:.5}, ..., {:.5}], len = {},",
            self.theta_span.0, self.theta_span.1, self.theta_len,
        )?;
        writeln!(
            f,
            "\t  psi = [{:.5}, ..., {:.5}], len = {},",
            self.psi_span.0, self.psi_span.1, self.psi_len,
        )
    }
}

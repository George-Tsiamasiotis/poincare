//! `Scalars` implementation.

use crate::{NcError, extract::extract_1d_var, extract::extract_scalar};

#[derive(Debug)]
/// Representation of an equilibrium's scalar values. `baxis` and `raxis` are the only quantities
/// in non-normalized units, and are not used in any calculations.
pub struct Scalars {
    /// Magnetic field strength on the axis in \[*T*\].
    pub baxis: f64,
    /// Tokamak's major radius in \[*m*\].
    pub raxis: f64,
    /// Last closed surface \[*Normalised*\].
    pub psi_wall: f64,
}

impl Scalars {
    /// Creates a `Scalars` containing the needed scalar values from the NetCDF file.
    pub(crate) fn build(f: &netcdf::File) -> Result<Self, NcError> {
        let baxis = extract_scalar(f, "Baxis")?;
        let raxis = extract_scalar(f, "raxis")?;

        // We can safely assume that the coords are sorted.
        // Whether the variable is empty or not is checked in the extraction.
        let psi_wall = match extract_1d_var(f, "psi")?.last() {
            Some(last) => *last,
            None => unreachable!("Error extracting psi_wall."),
        };

        Ok(Self {
            baxis,
            raxis,
            psi_wall,
        })
    }
}

impl std::fmt::Display for Scalars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Scalars:\n\tbaxis = {:.5} [T],\n\traxis = {:.5} [m],\n\tpsi_wall = {:.5}",
            self.baxis, self.raxis, self.psi_wall
        )
    }
}

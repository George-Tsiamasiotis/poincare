//! Representation of an equilibrium's scalar values.

use crate::{NcError, extract::extract_1d, extract::extract_scalar};

#[derive(Debug)]
pub struct Scalars {
    pub baxis: f64,    // Magnetic field strength on the axis in [T].
    pub raxis: f64,    // Tokamak's major radius in [m].
    pub psi_wall: f64, // Last closed surface [Normalised].
}

impl Scalars {
    /// Creates a struct containing the needed scalar values
    pub(crate) fn build(f: &netcdf::File) -> Result<Self, NcError> {
        let baxis = extract_scalar(f, "Baxis")?;
        let raxis = extract_scalar(f, "raxis")?;
        let psis = extract_1d(f, "psi")?;

        // We can safely assume that the coords are sorted.
        // Whether the variable is empty or not is checked in the extraction.
        let psi_wall = match psis.last() {
            Some(psi_wall) => *psi_wall,
            None => unreachable!(),
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

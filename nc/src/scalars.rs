//! Representation of an equilibrium's scalar values.

use crate::{NcError, extract::extract_scalar};

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
        let psi_wall = 1.0; // TODO:

        Ok(Self {
            baxis,
            raxis,
            psi_wall,
        })
    }
}

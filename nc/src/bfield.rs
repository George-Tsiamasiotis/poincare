use ndarray::Array2;

use crate::{NcError, extract::extract_2d_var};

#[derive(Debug)]
pub struct Bfield {
    /// Magnetic field strength as a function of ψ and θ.
    pub b: Array2<f64>,
}

impl Bfield {
    /// Creates a `Bfield` containing the magnetic field data from the NetCDF file.
    pub(crate) fn build(f: &netcdf::File) -> Result<Self, NcError> {
        let b = extract_2d_var(f, "b_field_norm")?;
        Ok(Bfield { b })
    }
}

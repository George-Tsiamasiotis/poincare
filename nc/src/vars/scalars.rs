//! Representation of an equilibrium's scalar values.

#[derive(Debug)]
/// The scalar values of the equilibrium.
pub struct Scalars {
    pub baxis: f64,    // Magnetic field strength on axis [T]
    pub raxis: f64,    // Major radius [m]
    pub psi_wall: f64, // last closed surface
}

impl Scalars {
    /// Creates a `Scalars` from the NetCDF file.
    pub(crate) fn from_netcdf_file(f: &netcdf::File) -> Scalars {
        Scalars {
            baxis: Scalars::extract_field(f, "Baxis").unwrap(),
            raxis: Scalars::extract_field(f, "raxis").unwrap(),
            psi_wall: 1.0, // TODO:
        }
    }

    /// Extracts a single valued `Variable`.
    fn extract_field(f: &netcdf::File, field: &str) -> Result<f64, ScalarError> {
        use ScalarError::*;

        // Extract 'Variable' field from file
        let variable = match f.variable(field) {
            Some(variable) => variable,
            None => return Err(FieldNotFound(field.into())),
        };

        // Make sure the value we extract is indeed scalar
        if variable.len() != 1 {
            return Err(NonScalarField(field.into()));
        }

        // Extract variable's value
        let value = match variable.get_value::<f64, _>(..) {
            Ok(value) => value,
            Err(_) => return Err(NaNField(field.into())),
        };
        Ok(value)
    }
}

impl std::fmt::Display for Scalars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Scalars:\n\tbaxis = {:.5}[T],\n\traxis = {:.5}[m],\n\tpsi_wall = {:.5}",
            self.baxis, self.raxis, self.psi_wall
        )
    }
}

#[derive(thiserror::Error)]
enum ScalarError {
    #[error("'Error: '{0}' field does not exist.")]
    FieldNotFound(String),
    #[error("Error: '{0}' is not a scalar field.")]
    NonScalarField(String),
    #[error("Error: '{0}' is NaN.")]
    NaNField(String),
}

impl std::fmt::Debug for ScalarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n\t{}", self)?;
        Ok(())
    }
}

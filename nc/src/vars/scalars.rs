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
    pub(crate) fn build(f: &netcdf::File) -> Result<Scalars, (String, &'static str)> {
        let s = Scalars {
            baxis: match Scalars::extract_field(f, "baxis") {
                Ok(val) => val,
                Err(err) => return Err(("Baxis ".into(), err)),
            },
            raxis: match Scalars::extract_field(f, "raxis") {
                Ok(val) => val,
                Err(err) => return Err(("raxis ".into(), err)),
            },
            psi_wall: 1.0, // TODO:
        };

        Ok(s)
    }

    /// Extracts a single valued `Variable`.
    fn extract_field(f: &netcdf::File, field: &str) -> Result<f64, &'static str> {
        // Extract 'Variable' field from file
        let variable = match f.variable(field) {
            Some(var) => var,
            None => return Err("field not found"),
        };

        // Make sure the value we extract is indeed scalar
        match variable.len() {
            0 => return Err("field is empty."),
            2.. => return Err("field is not scalar."),
            1 => (),
        }

        // Extract variable's value
        let value = match variable.get_value::<f64, _>(..) {
            Ok(value) => value,
            Err(_) => return Err("field is NaN"),
        };

        Ok(value)
    }
}

impl std::fmt::Display for Scalars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Scalars:\n\tbaxis = {:.5}[T],\n\traxis = {:.5}[m],\n\tpsi_wall = {:.5}",
            self.baxis, self.raxis, self.psi_wall
        )
    }
}

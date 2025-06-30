//! Representation of an equilibrium's theta and psi coordinates

#[derive(Debug)]
/// The $\theta$ and $\psi$ coordinates data points.
pub struct Coords {
    pub boozer_theta: Vec<f64>,
    pub psi: Vec<f64>,
}

impl Coords {
    /// Creates a `Coords` by extracting data from `f`.
    pub fn from_netcdf_file(f: &netcdf::File) -> Coords {
        Coords {
            boozer_theta: Coords::extract_coord(f, "boozer_theta").unwrap(),
            psi: Coords::extract_coord(f, "psi").unwrap(),
        }
    }

    /// Extracts all values from a coord.
    fn extract_coord(f: &netcdf::File, field: &str) -> Result<Vec<f64>, CoordError> {
        use CoordError::*;
        let var = match f.variable(field) {
            Some(coord) => coord,
            None => return Err(CoordNotFound(field.into())),
        };
        let values: Vec<f64> = match var.get_values(..) {
            Ok(values) => values,
            Err(_) => return Err(ExtendsError(field.into())),
        };
        Ok(values)
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Coords:")?;
        writeln!(
            f,
            "\tboozer_theta = [{:.5}, ..., {:.5}], len = {},",
            self.boozer_theta.first().unwrap(),
            self.boozer_theta.last().unwrap(),
            self.boozer_theta.len(),
        )?;
        writeln!(
            f,
            "\t         psi = [{:.5}, ..., {:.5}], len = {},",
            self.psi.first().unwrap(),
            self.psi.last().unwrap(),
            self.psi.len(),
        )
    }
}

#[derive(thiserror::Error)]
enum CoordError {
    #[error("Error: '{0}' field does not exist.")]
    CoordNotFound(String),
    #[error("Error extracting '{0}' values.")]
    ExtendsError(String),
}

impl std::fmt::Debug for CoordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n\t{}", self)?;
        Ok(())
    }
}

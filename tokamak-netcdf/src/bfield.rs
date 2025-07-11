use ndarray::Array2;

use crate::{Result, extract::extract_2d_var};

/// Representation of the equilibrium's magnetic field **B**.
pub struct Bfield {
    /// Magnetic field strength as a function of ψ and θ.
    pub b: Array2<f64>,
    /// Magnetic field values shape (ψ, θ).
    shape: (usize, usize),
}

impl Bfield {
    /// Creates a `Bfield` containing the magnetic field data from the NetCDF file.
    pub(crate) fn build(f: &netcdf::File) -> Result<Self> {
        let b = extract_2d_var(f, "b_field_norm")?;

        let shape: (usize, usize) = (b.dim().0, b.dim().1);
        Ok(Bfield { b, shape })
    }
}

impl std::fmt::Debug for Bfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.shape.0;
        let columns = self.shape.1;
        writeln!(f, "Bfield {{")?;
        writeln!(
            f,
            "    (Array: dim={:?}, shape={:?})",
            self.b.ndim(),
            self.shape,
        )?;

        for row in [0, 1, rows - 2, rows - 1] {
            writeln!(
                f,
                "    ({:.7}, {:.7}, ..., {:.7}, {:.7})",
                self.b[[row, 0]],
                self.b[[row, 1]],
                self.b[[row, columns - 2]],
                self.b[[row, columns - 1]]
            )?;
            if row == 1 {
                writeln!(f, "    (........., ........., ..., ........., .........) ")?;
            }
        }
        write!(f, "}}")
    }
}

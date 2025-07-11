//! `Currents` implementation.

use ndarray::Array1;

use crate::{
    Result,
    extract::{extract_var_with_axis_value, extract_var_with_first_axis_value},
};

/// Representation of the equilibrium's **I** and **g** toroidal and poloidal plasma currents.
pub struct Currents {
    /// Plasma toroidal current **I**.
    pub i: Array1<f64>,
    /// Plasma poloidal current **g**.
    pub g: Array1<f64>,
    /// The toroidal currrent's length.
    i_len: usize,
    /// The poloidal currrent's length.
    g_len: usize,
    /// The toroidal current's span (min, max).
    i_span: (f64, f64),
    /// The poloidal current's span (min, max).
    g_span: (f64, f64),
}

impl Currents {
    /// Creates a `Currents` containing the plasma currents **I** and **g** from the NetCDF file.
    pub(crate) fn build(f: &netcdf::File) -> Result<Self> {
        let g = extract_var_with_first_axis_value(f, "g_norm")?;
        let i = extract_var_with_axis_value(f, "I_norm", 0.0)?;

        let g_len = g.len();
        let i_len = i.len();

        // Safe unwrap(); both g and i have already been checked.
        let g_span = (*g.first().unwrap(), *g.last().unwrap());
        let i_span = (*i.first().unwrap(), *i.last().unwrap());

        let currents = Currents {
            i,
            g,
            i_len,
            g_len,
            i_span,
            g_span,
        };

        Ok(currents)
    }
}

impl std::fmt::Debug for Currents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Currents: {{")?;
        writeln!(
            f,
            "        i = [{:.7}, ..., {:.7}], len = {},",
            self.i_span.0, self.i_span.1, self.i_len,
        )?;
        writeln!(
            f,
            "        g = [{:.7}, ..., {:.7}], len = {},",
            self.g_span.0, self.g_span.1, self.g_len,
        )?;
        write!(f, "}}")
    }
}

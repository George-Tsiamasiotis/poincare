use std::fmt::Display;

use crate::extract_1d_var;

#[derive(Debug)]
pub struct Currents {
    pub g: Vec<f64>,
    pub i: Vec<f64>,
    pub g_len: usize,
    pub i_len: usize,
    pub g_min: f64,
    pub g_max: f64,
    pub i_min: f64,
    pub i_max: f64,
    // Derivatives are to be calculated through the splines
}

impl Currents {
    /// Creates a `Scalars` from the NetCDF file.
    pub(crate) fn from_netcdf_file(f: &netcdf::File) -> Currents {
        let g = extract_1d_var(f, "g_norm").unwrap();
        let i = extract_1d_var(f, "I_norm").unwrap();
        let g_len = g.len();
        let i_len = i.len();
        // I is sorted, g is sorted in reverse order
        let g_min = *g.last().unwrap();
        let g_max = *g.first().unwrap();
        let i_min = *i.first().unwrap();
        let i_max = *i.last().unwrap();

        Currents {
            g,
            i,
            g_len,
            i_len,
            g_min,
            g_max,
            i_min,
            i_max,
        }
    }
}

impl Display for Currents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Currents:")?;
        writeln!(
            f,
            "\tg = [{:.5}, ..., {:.5}], len = {},",
            self.g_min, self.g_max, self.g_len,
        )?;
        writeln!(
            f,
            "\ti = [{:.5}, ..., {:.5}], len = {},",
            self.i_min, self.i_max, self.i_len,
        )
    }
}

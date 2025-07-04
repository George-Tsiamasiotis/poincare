use crate::extract_2d_var;

#[derive(Debug)]
pub struct Bfield {
    pub b: Vec<Vec<f64>>,
}

impl Bfield {
    /// Creates a `Bfield` from the NetCDF file.
    pub(crate) fn from_netcdf_file(f: &netcdf::File) -> Bfield {
        let b = extract_2d_var(f, "b_field_norm").unwrap();
        Bfield { b }
    }
}

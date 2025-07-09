use ndarray::{Array, Ix2};
use std::path::PathBuf;

/// Creates a phony NetCDF file simulating the actual equilibrium.
pub(crate) fn phony_netcdf_path() -> Result<PathBuf, netcdf::Error> {
    let path = std::env::temp_dir().join("phony.nc");
    let path_str = path.to_str().expect("*Probably* won't fail.");

    let mut f = netcdf::create(path_str)?;

    // Scalars
    f.add_variable::<f64>("Baxis", &[])?
        .put_values(&[1.0], ..)?;
    f.add_variable::<f64>("raxis", &[])?
        .put_values(&[1.65], ..)?;

    // 1D variables
    // Evidently `psi` and `boozer_theta` appear both as coords and variables.
    let shape = (2, 3);
    f.add_dimension("psi", shape.0)?;
    f.add_dimension("boozer_theta", shape.1)?;

    f.add_variable::<f64>("psi", &["psi"])?;
    f.add_variable::<f64>("boozer_theta", &["boozer_theta"])?;
    f.add_variable::<f64>("I_norm", &["psi"])?
        .put_values(&[0.0, 0.1], ..)?;
    f.add_variable::<f64>("g_norm", &["psi"])?
        .put_values(&[0.2, 0.1], ..)?;

    // 2D variable
    let b_values =
        Array::<f64, Ix2>::from_shape_vec(shape, vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]).unwrap();
    f.add_variable::<f64>("b_field_norm", &["psi", "boozer_theta"])?
        .put(b_values.view(), (.., ..))?;

    Ok(f.path()?)
}

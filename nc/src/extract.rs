//! Functions for extracting and checking data from the NetCDF file.

use crate::NcError;
use ndarray::{Array, Ix2};

/// Extracts a `Variable` fron a NetCDF file.
fn extract_variable<'a>(
    f: &'a netcdf::File,
    name: &'a str,
) -> Result<netcdf::Variable<'a>, NcError> {
    f.variable(name)
        .ok_or(NcError::VariableNotFound(name.into()))
}

/// Checks if a `Variable` is empty.
pub(crate) fn check_if_empty(var: &netcdf::Variable) -> Result<(), NcError> {
    match var.len() {
        1.. => Ok(()),
        0 => Err(NcError::EmptyVariable(var.name().into())),
    }
}

/// Extracts a scalar (0D) `Variable`'s value.
pub(crate) fn extract_scalar(f: &netcdf::File, name: &str) -> Result<f64, NcError> {
    use crate::NcError::*;

    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    // `var.dimensions()` is () for netcdf's scalar `Variables`. This is probably equivalent to
    // `var.len() == 0`
    if !var.dimensions().is_empty() {
        return Err(NotScalar(name.into()));
    }

    match var.get_value::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(err) => Err(NcError::GetValuesError {
            name: var.name().into(),
            source: err,
        }),
    }
}

/// Extracts a 1D `Variable` and returns its values.
pub(crate) fn extract_1d_var(f: &netcdf::File, name: &str) -> Result<Vec<f64>, NcError> {
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    match var.get_values::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(err) => Err(NcError::GetValuesError {
            name: var.name().into(),
            source: err,
        }),
    }
}

/// Extracts a 2D `Variable` and returns its values as an `ndarray`.
pub(crate) fn extract_2d_var(f: &netcdf::File, name: &str) -> Result<Array<f64, Ix2>, NcError> {
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    if var.dimensions().len() != 2 {
        return Err(NcError::Not2D(var.name().into()));
    }

    // Dimension order is (ψ,θ).
    let dims = var.dimensions().to_vec();
    let shape = (dims[0].len(), dims[1].len());

    let mut data = Array::<f64, Ix2>::zeros(shape);

    match var.get_into(data.view_mut(), (.., ..)) {
        Ok(()) => Ok(data),
        Err(err) => Err(NcError::GetValuesError {
            name: var.name().into(),
            source: err,
        }),
    }
}

/// Extracts a variable from the NetCDF file and prepends the first value (value closest to the
/// magnetic axis) at index 0.
pub(crate) fn extract_var_with_first_axis_value(
    f: &netcdf::File,
    name: &str,
) -> Result<Vec<f64>, NcError> {
    let mut v: Vec<f64> = extract_1d_var(f, name)?;
    v.insert(0, v[0]);
    Ok(v)
}

/// Extracts a variable from the NetCDF file and prepends `element` at index 0.
pub(crate) fn extract_var_with_axis_value(
    f: &netcdf::File,
    name: &str,
    element: f64,
) -> Result<Vec<f64>, NcError> {
    let mut v: Vec<f64> = extract_1d_var(f, name)?;
    v.insert(0, element);
    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;
    use NcError::*;

    static VAR_LENGTH: usize = 10;

    /// Creates a phony NetCDF file for use across the tests.
    fn phony_netcdf() -> netcdf::FileMut {
        let path = std::env::temp_dir().join("phony.nc");
        let path_str = path.to_str().expect("*Probably* won't fail.");

        let mut f = netcdf::create(path_str).expect("Error creating phony.nc.");
        std::fs::remove_file(path).expect("Should never fail");

        f.add_dimension("dim", VAR_LENGTH)
            .expect("Could not add dimension to phony.nc");
        f.add_variable::<f64>("var", &["dim"])
            .expect("Could not add variable to phony.nc");

        f.add_dimension("empty_dim", 0)
            .expect("Could not add dimension to phony.nc");
        f.add_variable::<f64>("empty_var", &["empty_dim"])
            .expect("Could not add variable to phony.nc");

        f
    }

    #[test]
    fn test_extract_variable() {
        let f = phony_netcdf();
        assert!(extract_variable(&f, "var").is_ok());
        assert!(matches!(
            extract_variable(&f, "not_a_var").unwrap_err(),
            VariableNotFound(_)
        ));
    }

    #[test]
    fn test_check_if_empty() {
        let f = phony_netcdf();
        let var = extract_variable(&f, "var").unwrap();
        let empty_var = extract_variable(&f, "empty_var").unwrap();

        assert_eq!(var.len(), VAR_LENGTH);
        assert_eq!(empty_var.len(), 0);
        assert!(matches!(
            check_if_empty(&empty_var).unwrap_err(),
            EmptyVariable(_)
        ));
    }

    #[test]
    fn test_extract_scalar() {
        /*
        Not sure how scalars are defined in NetCDF. The documentation states that they
        used to be treated as a 0D array, but it's been struckthrough.
        */
    }
}

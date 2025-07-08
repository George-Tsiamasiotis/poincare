//! Functions for extracting and checking data from the NetCDF file.

use crate::NcError;

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
        Err(_) => Err(NcError::GetValuesError(var.name().into())),
    }
}

/// Extracts a 1D `Variable` and returns its values.
pub(crate) fn extract_1d_var(f: &netcdf::File, name: &str) -> Result<Vec<f64>, NcError> {
    let var = extract_variable(f, name)?;
    check_if_empty(&var)?;

    match var.get_values::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(_) => Err(NcError::GetValuesError(var.name().into())),
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

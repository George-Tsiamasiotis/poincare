use crate::NcError;

/// Extracts a scalar (0D) variable.
pub(crate) fn extract_scalar(f: &netcdf::File, name: &str) -> Result<f64, NcError> {
    use crate::NcError::*;

    let var = match f.variable(name) {
        Some(var) => var,
        None => return Err(VariableNotFound(name.into())),
    };

    // `.len()` works for N-dimensional arrays too; it returns the total number of elements.
    match var.len() {
        0 => return Err(EmptyVariable(name.into())),
        1 => (),
        2.. => return Err(NotScalar(name.into())),
    };

    match var.get_value::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(dim_err) => Err(DimensionError {
            source: dim_err, // Error::DimensionMismatch
            field: name.into(),
        }),
    }
}

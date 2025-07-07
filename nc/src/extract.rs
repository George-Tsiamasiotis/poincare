use crate::NcError;

fn extract_variable<'a>(
    f: &'a netcdf::File,
    name: &'a str,
) -> Result<netcdf::Variable<'a>, NcError> {
    f.variable(name)
        .ok_or(NcError::VariableNotFound(name.into()))
}

/// Extracts a scalar (0D) variable.
pub(crate) fn extract_scalar(f: &netcdf::File, name: &str) -> Result<f64, NcError> {
    use crate::NcError::*;

    let var = extract_variable(f, name)?;

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

pub(crate) fn extract_1d(f: &netcdf::File, name: &str) -> Result<Vec<f64>, NcError> {
    use crate::NcError::*;

    let var = extract_variable(f, name)?;

    if var.len() == 0 {
        return Err(EmptyVariable(name.into()));
    }

    match var.get_values::<f64, _>(..) {
        Ok(value) => Ok(value),
        Err(err) => Err(DimensionError {
            source: err, // Error::DimensionMismatch
            field: name.into(),
        }),
    }
}

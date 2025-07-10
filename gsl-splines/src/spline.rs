use ndarray::Array1;
use rgsl::{Interp, InterpType};

use crate::Result;
use crate::{SplineError, SplineType};

/// Interface to GSL's splines.
pub struct Spline {
    /// The `Spline`'s type. Corresponds to one of GSL's
    /// [interpolation types](https://www.gnu.org/software/gsl/doc/html/interp.html#d-interpolation-types)
    pub spline_type: SplineType,
    /// 1D array of the **sorted** `x` data points.
    pub xdata: Array1<f64>,
    /// 1D array of the `y` data points.
    pub ydata: Array1<f64>,
    /// Size of the data arrays.
    pub(crate) size: usize,
    /// Span of the xdata array.
    pub(crate) xspan: (f64, f64),
    /// Span of the ydata array.
    pub(crate) yspan: (f64, f64),
    #[allow(dead_code)]
    /// Corresponding GSL's natice `gsl_interp_type`.
    pub(crate) interp_type: InterpType,
    /// Pointer to allocated `Interp` native object.
    pub(crate) gsl_spline: Interp,
}

impl Spline {
    /// Creates a new `Spline`.
    pub fn build(typ: SplineType, xdata: Array1<f64>, ydata: Array1<f64>) -> Result<Self> {
        Spline::check_data(&xdata, &ydata)?;

        // 1D, non-empty arrays of the same length
        let size = xdata.len();
        let xmin = xdata[0];
        let xmax = xdata[size - 1];
        let ymin = ydata[0];
        let ymax = ydata[size - 1];

        let gsl_spline = Interp::new(typ.into(), size).ok_or(SplineError::GSLInterpAlloc)?;

        let s = Spline {
            spline_type: typ,
            xdata,
            ydata,
            size,
            xspan: (xmin, xmax),
            yspan: (ymin, ymax),
            interp_type: typ.into(),
            gsl_spline,
        };
        Ok(s)
    }

    /// Checks that the supplied `x` and `y` data are valid.
    fn check_data(x: &Array1<f64>, y: &Array1<f64>) -> Result<()> {
        if x.is_empty() {
            return Err(SplineError::EmptyDataset("x".into()));
        };
        if y.is_empty() {
            return Err(SplineError::EmptyDataset("y".into()));
        };

        if x.ndim() != y.ndim() && x.ndim() != 1 {
            return Err(SplineError::DatasetMismatch);
        };

        if !x.iter().is_sorted() {
            return Err(SplineError::UnsortedDataset);
        };
        Ok(())
    }
}

impl std::fmt::Debug for Spline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spline")
            .field("spline_type", &self.spline_type)
            .field(
                "xdata",
                &format!(
                    "[{:.5}, .. {:.5}], size={}",
                    self.xspan.0, self.xspan.1, self.size
                ),
            )
            .field(
                "ydata",
                &format!(
                    "[{:.5}, .. {:.5}], size={}",
                    self.yspan.0, self.yspan.1, self.size
                ),
            )
            .finish()
    }
}

use rgsl::InterpType;

/// Spline types supported by GSL.
///
/// Descriptions are taken directly from GSL's
/// [interpolation](https://www.gnu.org/software/gsl/doc/html/interp.html#c.gsl_interp_type) page
#[derive(Debug, Clone, Copy)]
pub enum SplineType {
    /// Linear interpolation. This interpolation method does not require any additional memory.
    Linear,

    /// Polynomial interpolation. This method should only be used for interpolating small number
    /// of points because polynomial interpolation introduces large oscillations, even for
    /// well-behaved datasets. The number of terms in the interpolating polynomial is equal to
    /// the number of points.
    Polynomial,

    /// Cubic spline with natural boundary conditions. The resulting curve is piecewise cubic on
    /// each interval, with matching first and second derivatives at the supplied data-points.
    /// The second derivative is chosen to be zero at the first point and last point.
    Cubic,

    /// Cubic spline with periodic boundary conditions. The resulting curve is piecewise cubic
    /// on each interval, with matching first and second derivatives at the supplied data-points.
    /// The derivatives at the first and last points are also matched. Note that the last point
    /// in the data must have the same y-value as the first point, otherwise the resulting
    /// periodic interpolation will have a discontinuity at the boundary.
    CubicPeriodic,

    /// Non-rounded Akima spline with natural boundary conditions. This method uses the
    /// non-rounded corner algorithm of Wodicka.
    Akima,

    /// Non-rounded Akima spline with periodic boundary conditions. This method uses the
    /// non-rounded corner algorithm of Wodicka.
    AkimaPeriodic,
    // TODO: Steffen method binding is missing
}

impl From<SplineType> for InterpType {
    /// Get the corresponding gsl_interp_type.
    fn from(value: SplineType) -> InterpType {
        use SplineType::*;

        match value {
            Linear => InterpType::linear(),
            Polynomial => InterpType::polynomial(),
            Cubic => InterpType::cspline(),
            CubicPeriodic => InterpType::cspline_periodic(),
            Akima => InterpType::akima(),
            AkimaPeriodic => InterpType::akima_periodic(),
            // Steffen => InterpType::steffen(),
        }
    }
}

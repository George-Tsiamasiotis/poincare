mod acc;
mod error;
mod spline;
mod spline_types;

pub use error::SplineError;
pub use spline::Spline;
pub use spline_types::SplineType;

type Result<T> = std::result::Result<T, SplineError>;

//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices.

mod error;
mod extract;
mod open;

mod scalars;

pub use error::NcError;
pub use open::NcData;

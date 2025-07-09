//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices.

mod error;
mod extract;
mod open;

mod bfield;
mod coords;
mod currents;
mod scalars;

pub use error::NcError;
pub use open::NcData;

pub use bfield::Bfield;
pub use coords::Coords;
pub use currents::Currents;
pub use scalars::Scalars;

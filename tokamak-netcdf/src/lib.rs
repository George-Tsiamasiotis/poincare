//! Provides a cleaner and more suitable NetCDF interface for use with reconstructed equilibria
//! from tokamak devices.
//!
//! ## Example
//!
//! ```no_run
//! # use std::path::PathBuf;
//! # use tokamak_netcdf::NcError;
//! #
//! # fn main() -> Result<(), NcError> {
//! #
//!     // Path must be relative to the directory where "cargo run" is called
//!     let path = PathBuf::from(r"./reconstructed/data.nc");
//!     let nc_data = tokamak_netcdf::NcData::open(path)?;
//!
//!     println!("{:#?}", nc_data);
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Note
//!
//! [libnetcdf](https://github.com/Unidata/netcdf-c) is linked statically, since it is not
//! available by default in most systems.

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

pub type Result<T> = std::result::Result<T, NcError>;

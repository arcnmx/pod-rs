#![deny(missing_docs)]

//! Provides traits that assist with I/O and byte slice conversions involving
//! Plain Old Data.
//!
//! # Safety
//!
//! The `nue-macros` crate can be used for safe automagic derives.

#[cfg(feature = "uninitialized")]
extern crate uninitialized;
#[cfg(feature = "read_exact")]
extern crate read_exact;

/// Re-export the `packed` crate
pub extern crate packed;

mod pod;
mod io;

pub use pod::Pod;
pub use io::{PodReadExt, PodWriteExt};

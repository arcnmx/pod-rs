#![deny(missing_docs)]

//! Provides traits that assist with I/O and byte slice conversions involving Plain Old Data.
//!
//! # Safety
//!
//! The `nue-macros` crate can be used for safe automagic derives.
//!
//! # Example
//!
//! ```
//! use pod::{Pod, Le, Be};
//! # #[cfg(not(feature = "packed/oibit"))]
//! # mod stable {
//! # use pod::packed::{Unaligned, Packed};
//! # unsafe impl Packed for super::Data { }
//! # unsafe impl Unaligned for super::Data { }
//! # }
//!
//! unsafe impl Pod for Data { }
//!
//! #[repr(C)]
//! struct Data(u8, Le<u16>, Be<u32>);
//!
//! # fn main() {
//! let data = Data(1, Le::new(0x2055), Be::new(0xdeadbeef));
//!
//! let cmp = &[
//!     0x01,
//!     0x55, 0x20,
//!     0xde, 0xad, 0xbe, 0xef,
//! ];
//!
//! assert_eq!(cmp, data.as_bytes());
//! # }
//!
//! ```

extern crate byteorder;
extern crate uninitialized;

/// Re-export the `packed` crate
pub extern crate packed;

/// Containers for primitives
pub mod endian;

mod pod;

pub use endian::{Le, Be, Native};
pub use pod::Pod;

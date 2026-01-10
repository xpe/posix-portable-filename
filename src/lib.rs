//! # posix-portable-filename
//!
//! A validated newtype for POSIX portable filenames.
//!
//! ## The Standard
//!
//! POSIX.1-2008 defines the **Portable Filename Character Set** in Section 3.282:
//!
//! - Uppercase: `A-Z`
//! - Lowercase: `a-z`
//! - Digits: `0-9`
//! - Period: `.`
//! - Underscore: `_`
//! - Hyphen: `-`
//!
//! Section 4.8 (Filename Portability) additionally specifies:
//!
//! > Portable filenames shall not have the hyphen character as the first
//! > character since this may cause problems when filenames are passed as
//! > command line arguments.
//!
//! This crate enforces these rules via a newtype that can only be constructed
//! through validation. If you have a [`PortableFilename`], it is guaranteed valid.
//!
//! ## Usage
//!
//! ```
//! use posix_portable_filename::PortableFilename;
//!
//! // Valid filenames
//! let name = PortableFilename::new("my_file.txt").unwrap();
//! assert_eq!(name.as_str(), "my_file.txt");
//!
//! // Invalid: contains space
//! assert!(PortableFilename::new("my file.txt").is_err());
//!
//! // Invalid: starts with hyphen
//! assert!(PortableFilename::new("-rf").is_err());
//!
//! // Invalid: path traversal
//! assert!(PortableFilename::new("..").is_err());
//! ```
//!
//! ## Serde Support
//!
//! Enable the `serde` feature to serialize/deserialize directly into the validated type:
//!
//! ```toml
//! [dependencies]
//! posix-portable-filename = { version = "0.1", features = ["serde"] }
//! ```
//!
//! Deserialization will fail if the string is not a valid portable filename.
//!
//! ## References
//!
//! - [POSIX.1-2008 Base Definitions, Section 3.282](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_282)
//! - [POSIX.1-2008 Base Definitions, Section 4.7](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_08)

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod core;
pub use core::{InvalidFilename, PortableFilename};

#[cfg(feature = "arbitrary")]
mod impl_arbitrary;

#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(doctest)]
mod doctests {
    doc_comment::doctest!("../README.md");
}

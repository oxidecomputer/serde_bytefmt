// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serialize byte arrays and vectors as bytes or as human-readable strings,
//! depending on the format.
//!
//! Many binary formats (e.g. [CBOR]) can natively represent byte sequences,
//! while text formats (e.g. JSON) cannot. This crate bridges the gap: it
//! serializes byte data as hex or base64 strings in human-readable formats,
//! and as efficient raw bytes in binary formats.
//!
//! [CBOR]: https://cbor.io/
//!
//! # Types
//!
//! * [`HexArray<N>`] encodes a fixed-length byte array as a hex string. (The
//!   `alloc` feature is required for serialization.)
//! * [`Base64Vec`] encodes a variable-length byte vector as a base64 string.
//!   (The `alloc` feature is required.)
//!
//! These types can be used directly as struct fields, or be applied to
//! existing `[u8; N]` / `Vec<u8>` fields via `#[serde(with = "...")]`.
//!
//! # Examples
//!
//! Using [`HexArray`] as a field type:
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use serde::{Deserialize, Serialize};
//! use serde_bytefmt::HexArray;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Record {
//!     checksum: HexArray<32>,
//! }
//! # }
//! ```
//!
//! Using `#[serde(with = "...")]` on an existing byte array:
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use serde::{Deserialize, Serialize};
//! use serde_bytefmt::HexArray;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Record {
//!     #[serde(with = "HexArray::<32>")]
//!     checksum: [u8; 32],
//! }
//! # }
//! ```
//!
//! # Features
//!
//! - **`alloc`**: enables [`Base64Vec`], as well as
//!   [`HexArray`] serialization. *Enabled by default.*
//! - **`schemars08`**: derives `JsonSchema` for both types.
//!   *Not enabled by default.*

#![deny(missing_docs)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod base64_vec;
mod hex_array;

#[cfg(feature = "alloc")]
pub use base64_vec::Base64Vec;
pub use hex_array::HexArray;

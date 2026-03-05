// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newtype wrappers for byte arrays and vectors with hex and base64
//! formatting.
//!
//! This crate provides wrapper types that display byte data in
//! human-readable encodings. [`HexArray<N>`] encodes fixed-length byte
//! arrays as hex strings, and [`Base64Vec`] encodes variable-length
//! byte vectors as base64 strings.
//!
//! With the `serde` feature, both types implement `Serialize` and
//! `Deserialize`, encoding as human-readable strings (hex or base64) in text
//! formats like JSON, and as efficient raw bytes in binary formats like [CBOR].
//! You do not have to use the newtypes in your own type definitions; you can
//! refer to them via `#[serde(with = "...")]` instead.
//!
//! [CBOR]: https://cbor.io/
//!
//! # Types
//!
//! * [`HexArray<N>`] encodes a fixed-length byte array as a hex
//!   string. (Requires the `hex` feature.)
//! * [`Base64Vec`] encodes a variable-length byte vector as a base64
//!   string. (Requires the `base64` feature.)
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "hex")] {
//! use byte_wrapper::HexArray;
//!
//! let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
//! assert_eq!(h.to_string(), "0102abff");
//!
//! let parsed: HexArray<4> = "0102abff".parse().unwrap();
//! assert_eq!(parsed, h);
//! # }
//! ```
//!
//! With the **`serde`** feature:
//!
//! ```
//! # #[cfg(all(feature = "hex", feature = "serde"))] {
//! use byte_wrapper::HexArray;
//! use serde::{Deserialize, Serialize};
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
//! # #[cfg(all(feature = "hex", feature = "serde"))] {
//! use byte_wrapper::HexArray;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Record {
//!     #[serde(with = "HexArray::<32>")]
//!     checksum: [u8; 32],
//! }
//! # }
//! ```
//!
//! # Alternatives
//!
//! Several crates solve parts of this problem, often using slightly
//! different approaches from `byte-wrapper`.
//!
//! | feature                              | `byte-wrapper` | [`serde-human-bytes`] 0.1.2 | [`serde-encoded-bytes`] 0.2.1 | [`serde_with`] 3.17.0 | [`hex-buffer-serde`] 0.4.0 | [`hexutil`] 0.1.0                    | [`serde-bytes-repr`] 0.3.0 |
//! |--------------------------------------|----------------|-----------------------------|-----------------------------|----------------------|----------------------------|--------------------------------------|-----------------------------|
//! | newtype wrappers                     | yes            | yes (hex only)              | no                          | no                   | no                         | no                                   | no                          |
//! | [`is_human_readable()`] switch       | yes            | yes                         | yes                         | no                   | yes                        | yes                                  | no                          |
//! | [`Display`] / [`FromStr`] / [`Deref`]| yes            | [`Deref`] only              | no                          | no                   | no                         | [`Display`] / [`FromStr`] via macro  | no                          |
//! | hex encoding                         | yes            | yes                         | yes                         | yes                  | yes                        | yes                                  | yes                         |
//! | base64 encoding                      | yes            | yes                         | yes                         | yes                  | no                         | no                                   | yes                         |
//! | `[u8; N]` support                    | yes            | yes                         | yes                         | yes                  | yes                        | via macros                           | no                          |
//! | `no_std`                             | yes            | yes                         | yes                         | yes                  | yes                        | yes                                  | no                          |
//! | [`JsonSchema`] (schemars)            | yes            | no                          | no                          | yes                  | no                         | no                                   | no                          |
//! | `#[serde(with)]` support             | yes            | yes                         | yes                         | yes                  | yes                        | no                                   | no                          |
//!
//! The closest alternatives are:
//!
//! * [`serde-human-bytes`], which provides
//!   newtypes with [`Deref`], [`is_human_readable()`] switching,
//!   and both hex and base64 encoding, but lacks [`Display`] /
//!   [`FromStr`] and [`JsonSchema`] support.
//!
//! * [`serde-encoded-bytes`], which provides most of the features of this
//!   crate, and is more general in some ways, but doesn't provide newtype
//!   or schemars support.
//!
//! * [`serde_with`], which offers schemars integration but does not check
//!   [`is_human_readable()`] by default.
//!
//! [`serde-encoded-bytes`]: https://docs.rs/serde-encoded-bytes
//! [`hex-buffer-serde`]: https://docs.rs/hex-buffer-serde
//! [`hexutil`]: https://docs.rs/hexutil
//! [`serde_with`]: https://docs.rs/serde_with
//! [`serde-bytes-repr`]: https://docs.rs/serde-bytes-repr
//! [`serde-human-bytes`]: https://docs.rs/serde-human-bytes
//! [`Display`]: core::fmt::Display
//! [`FromStr`]: core::str::FromStr
//! [`Deref`]: core::ops::Deref
//! [`JsonSchema`]: https://docs.rs/schemars/0.8/schemars/trait.JsonSchema.html
//! [`is_human_readable()`]: https://docs.rs/serde/latest/serde/trait.Serializer.html#method.is_human_readable
//! [`Hex`]: https://docs.rs/serde_with/latest/serde_with/hex/struct.Hex.html
//! [`Base64`]: https://docs.rs/serde_with/latest/serde_with/base64/struct.Base64.html
//!
//! # Features
//!
//! - **`hex`**: enables [`HexArray`]. *Enabled by default.*
//! - **`base64`**: enables [`Base64Vec`] (implies `alloc`).
//!   *Enabled by default.*
//! - **`alloc`**: enables `alloc` support (required by `base64`).
//! - **`serde`**: implements `Serialize` and `Deserialize` for
//!   enabled types. *Not enabled by default.*
//! - **`schemars08`**: derives `JsonSchema` for enabled types.
//!   *Not enabled by default.*

#![deny(missing_docs)]
#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "base64")]
mod base64_vec;
#[cfg(feature = "hex")]
mod hex_array;
#[cfg(all(feature = "schemars08", any(feature = "base64", feature = "hex")))]
mod schemars_util;

#[cfg(feature = "base64")]
pub use base64_vec::{Base64Vec, ParseBase64Error};
#[cfg(feature = "hex")]
pub use hex_array::{HexArray, ParseHexError};

// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::{format, string::String};

/// Builds a schemars extensions map containing the `x-rust-type` entry for the
/// given type name. The version is the earliest semver-compatible release of
/// this crate; update it only on breaking changes.
pub(crate) fn x_rust_type_extension(
    type_name: &str,
) -> schemars08::Map<String, serde_json::Value> {
    [(
        "x-rust-type".into(),
        serde_json::json!({
            "crate": "byte-wrapper",
            "version": "0.1.0",
            "path": format!("byte_wrapper::{type_name}"),
        }),
    )]
    .into_iter()
    .collect()
}

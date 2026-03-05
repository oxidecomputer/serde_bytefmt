// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`Base64Vec`] newtype wrapper.

extern crate alloc;

use crate::base64_vec;
use alloc::vec::Vec;
use base64::Engine;
use core::fmt;
use serde_core::{Deserializer, Serializer};

/// A byte vector that serializes as base64 in human-readable formats.
///
/// This type can be used in two ways:
///
/// 1. Directly as a field type, with serde impls built in.
/// 2. With `#[serde(with = "Base64Vec")]` and `#[schemars(with = "Base64Vec")]`
///    on a `Vec<u8>` field.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Base64Vec(pub Vec<u8>);

impl Base64Vec {
    /// Creates a new `Base64Vec` from a byte vector.
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns the inner byte vector.
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Serializes a byte vector as base64 (for `#[serde(with = "Base64Vec")]`).
    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        base64_vec::serialize(bytes, serializer)
    }

    /// Deserializes a byte vector from base64 (for `#[serde(with = "Base64Vec")]`).
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        base64_vec::deserialize(deserializer)
    }
}

impl fmt::Debug for Base64Vec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Base64Vec({})",
            base64::engine::general_purpose::STANDARD.encode(&self.0)
        )
    }
}

impl fmt::Display for Base64Vec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        base64::engine::general_purpose::STANDARD.encode(&self.0).fmt(f)
    }
}

impl core::ops::Deref for Base64Vec {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Base64Vec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Base64Vec {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Base64Vec {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<Vec<u8>> for Base64Vec {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<Base64Vec> for Vec<u8> {
    #[inline]
    fn from(base64_vec: Base64Vec) -> Self {
        base64_vec.0
    }
}

impl serde_core::Serialize for Base64Vec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        base64_vec::serialize(&self.0, serializer)
    }
}

impl<'de> serde_core::Deserialize<'de> for Base64Vec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        base64_vec::deserialize(deserializer).map(Self)
    }
}

#[cfg(feature = "schemars08")]
mod schemars_impls {
    use super::Base64Vec;
    use alloc::string::String;
    use schemars08::{
        JsonSchema,
        r#gen::SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject},
    };

    impl JsonSchema for Base64Vec {
        fn schema_name() -> String {
            "Base64Vec".into()
        }

        fn is_referenceable() -> bool {
            false
        }

        fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                format: Some("byte".into()),
                extensions: [("contentEncoding".into(), "base64".into())]
                    .into_iter()
                    .collect(),
                ..Default::default()
            })
        }
    }
}

// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`HexArray`] newtype wrapper.

use core::{convert::TryInto, fmt};
use serde_core::{
    Deserializer,
    de::{Expected, Visitor},
};

/// A byte array that serializes as hex in human-readable formats.
///
/// This type can be used in two ways:
///
/// 1. Directly as a field type, with serde impls built in.
/// 2. With `#[serde(with = "HexArray::<N>")]` and
///    `#[schemars(with = "HexArray<N>")]` on a `[u8; N]` field.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct HexArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for HexArray<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> HexArray<N> {
    /// Creates a new `HexArray` from a byte array.
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Returns the inner byte array.
    #[inline]
    pub const fn into_inner(self) -> [u8; N] {
        self.0
    }

    /// Serializes a byte array as hex (for
    /// `#[serde(with = "HexArray::<N>")]`).
    #[cfg(feature = "alloc")]
    pub fn serialize<S>(
        bytes: &[u8; N],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serialize_lower(bytes, serializer)
    }

    /// Deserializes a byte array from hex (for
    /// `#[serde(with = "HexArray::<N>")]`).
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer)
    }
}

/// Serializes a byte slice as lower-case hex if human-readable, or as
/// raw bytes if not.
#[cfg(feature = "alloc")]
fn serialize_lower<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde_core::Serializer,
{
    use hex::ToHex;

    if serializer.is_human_readable() {
        let s = bytes.encode_hex::<alloc::string::String>();
        serializer.serialize_str(&s)
    } else {
        serializer.serialize_bytes(bytes)
    }
}

/// Deserializes hex strings (if human-readable) or byte arrays (if not)
/// to `[u8; N]`.
fn deserialize<'de, D, const N: usize>(
    deserializer: D,
) -> Result<[u8; N], D::Error>
where
    D: Deserializer<'de>,
{
    use serde_core::de::Error;

    if deserializer.is_human_readable() {
        // hex::FromHex doesn't have an implementation for
        // const-generic N, so do our own thing.
        struct HexVisitor<const N: usize>;

        impl<'de2, const N: usize> Visitor<'de2> for HexVisitor<N> {
            type Value = [u8; N];

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a hex-encoded string {} bytes long", N)
            }

            fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut out = [0u8; N];
                hex::decode_to_slice(data, &mut out).map_err(Error::custom)?;
                Ok(out)
            }

            fn visit_borrowed_str<E>(
                self,
                data: &'de2 str,
            ) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut out = [0u8; N];
                hex::decode_to_slice(data, &mut out).map_err(Error::custom)?;
                Ok(out)
            }
        }

        deserializer.deserialize_str(HexVisitor)
    } else {
        struct BytesVisitor<const N: usize>;

        impl<'de2, const N: usize> Visitor<'de2> for BytesVisitor<N> {
            type Value = [u8; N];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a byte array [u8; {}]", N)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map_err(|_| E::invalid_length(v.len(), &HexExpected::<N>))
            }
        }

        deserializer.deserialize_bytes(BytesVisitor)
    }
}

struct HexExpected<const N: usize>;

impl<const N: usize> Expected for HexExpected<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array [u8; {}]", N)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> fmt::Debug for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HexArray({})", hex::encode(self.0))
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> fmt::Display for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        hex::encode(self.0).fmt(f)
    }
}

impl<const N: usize> core::ops::Deref for HexArray<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> core::ops::DerefMut for HexArray<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HexArray<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for HexArray<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> From<[u8; N]> for HexArray<N> {
    #[inline]
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> From<HexArray<N>> for [u8; N] {
    #[inline]
    fn from(hex_array: HexArray<N>) -> Self {
        hex_array.0
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> serde_core::Serialize for HexArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serialize_lower(&self.0, serializer)
    }
}

impl<'de, const N: usize> serde_core::Deserialize<'de> for HexArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer).map(Self)
    }
}

#[cfg(feature = "schemars08")]
mod schemars_impls {
    use super::HexArray;
    use alloc::{boxed::Box, format, string::String};
    use schemars08::{
        JsonSchema,
        r#gen::SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject, StringValidation},
    };

    impl<const N: usize> JsonSchema for HexArray<N> {
        fn schema_name() -> String {
            format!("HexArray_{N}")
        }

        fn is_referenceable() -> bool {
            false
        }

        fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
            let hex_len = N * 2;
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                string: Some(Box::new(StringValidation {
                    min_length: Some(hex_len as u32),
                    max_length: Some(hex_len as u32),
                    pattern: Some(format!("^[0-9a-fA-F]{{{hex_len}}}$")),
                })),
                ..Default::default()
            })
        }
    }
}

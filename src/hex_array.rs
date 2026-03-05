// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`HexArray`] newtype wrapper.

use core::{convert::TryInto, fmt, fmt::Write};
use serde_core::{
    Deserializer,
    de::{Expected, SeqAccess, Visitor},
};

/// A byte array that serializes as hex in human-readable formats.
///
/// This type can be used in two ways:
///
/// 1. Directly as a field type, with serde impls built in.
/// 2. With `#[serde(with = "HexArray::<N>")]` and
///    `#[schemars(with = "HexArray<N>")]` on a `[u8; N]` field.
///
/// # Examples
///
/// As a direct field type:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_bytefmt::HexArray;
///
/// #[derive(Serialize, Deserialize)]
/// struct Record {
///     checksum: HexArray<32>,
/// }
/// ```
///
/// With `#[serde(with)]` on a raw byte array:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_bytefmt::HexArray;
///
/// #[derive(Serialize, Deserialize)]
/// struct Record {
///     #[serde(with = "HexArray::<32>")]
///     checksum: [u8; 32],
/// }
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Serializes a byte array as hex in human-readable formats, or as raw
    /// bytes otherwise.
    ///
    /// Intended for use with `#[serde(with = "HexArray::<N>")]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use serde_bytefmt::HexArray;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Record {
    ///     #[serde(with = "HexArray::<4>")]
    ///     id: [u8; 4],
    /// }
    ///
    /// let r = Record { id: [0x01, 0x02, 0x03, 0x04] };
    /// let json = serde_json::to_string(&r).unwrap();
    /// assert_eq!(json, r#"{"id":"01020304"}"#);
    /// ```
    pub fn serialize<S>(
        bytes: &[u8; N],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        serialize_lower(bytes, serializer)
    }

    /// Deserializes a byte array from hex if the format is human-readable, or as
    /// raw bytes otherwise.
    ///
    /// Intended for use with `#[serde(with = "HexArray::<N>")]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use serde_bytefmt::HexArray;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Record {
    ///     #[serde(with = "HexArray::<4>")]
    ///     id: [u8; 4],
    /// }
    ///
    /// let r: Record = serde_json::from_str(r#"{"id":"01020304"}"#).unwrap();
    /// assert_eq!(r.id, [0x01, 0x02, 0x03, 0x04]);
    /// ```
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer)
    }
}

/// Formats a byte slice as lower-case hex.
///
/// This is used both for serialization (via `Display`) and as the
/// inner value in `HexArray`'s `Debug` output (via `Debug`).
struct HexDisplay<'a>(&'a [u8]);

impl fmt::Display for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for HexDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Serializes a byte slice as lower-case hex if human-readable, or as
/// raw bytes if not.
fn serialize_lower<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde_core::Serializer,
{
    if serializer.is_human_readable() {
        serializer.collect_str(&HexDisplay(bytes))
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

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de2>,
            {
                let mut out = [0u8; N];
                for (i, byte) in out.iter_mut().enumerate() {
                    *byte = seq.next_element()?.ok_or_else(|| {
                        Error::invalid_length(i, &HexExpected::<N>)
                    })?;
                }
                Ok(out)
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

impl<const N: usize> fmt::Debug for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HexArray").field(&HexDisplay(&self.0)).finish()
    }
}

impl<const N: usize> fmt::Display for HexArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_len = N * 2;

        match f.width() {
            Some(width) if width > content_len => {
                let padding = width - content_len;
                let fill = f.fill();
                let (pre, post) = match f.align() {
                    Some(fmt::Alignment::Left) => (0, padding),
                    Some(fmt::Alignment::Right) | None => (padding, 0),
                    Some(fmt::Alignment::Center) => {
                        (padding / 2, padding - padding / 2)
                    }
                };
                for _ in 0..pre {
                    f.write_char(fill)?;
                }
                for byte in &self.0 {
                    write!(f, "{byte:02x}")?;
                }
                for _ in 0..post {
                    f.write_char(fill)?;
                }
                Ok(())
            }
            Some(_) | None => {
                for byte in &self.0 {
                    write!(f, "{byte:02x}")?;
                }
                Ok(())
            }
        }
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

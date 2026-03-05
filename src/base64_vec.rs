// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`Base64Vec`] newtype wrapper.

use alloc::vec::Vec;
use base64::Engine;
use core::{error, fmt, str::FromStr};

/// A byte vector that displays and parses as base64.
///
/// `Base64Vec` wraps `Vec<u8>`, providing [`Display`](fmt::Display)
/// and [`FromStr`] implementations that use base64 encoding.
///
/// With the **`serde`** feature enabled, it also implements
/// `Serialize` and `Deserialize` (base64 strings in human-readable
/// formats, raw bytes in binary formats), and can be used with
/// `#[serde(with = "Base64Vec")]` on `Vec<u8>` fields.
///
/// # Examples
///
/// ```
/// use byte_wrapper::Base64Vec;
///
/// let b = Base64Vec::new(vec![1, 2, 3]);
/// assert_eq!(b.to_string(), "AQID");
///
/// let parsed: Base64Vec = "AQID".parse().unwrap();
/// assert_eq!(*parsed, [1, 2, 3]);
/// ```
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Base64Vec(pub Vec<u8>);

impl Base64Vec {
    /// Creates a new `Base64Vec` from a byte vector.
    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns the inner byte vector.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

/// Error returned by [`Base64Vec::from_str`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseBase64Error {
    /// An invalid byte was found at the given offset.
    InvalidByte {
        /// Byte offset of the invalid symbol.
        offset: usize,
        /// The invalid byte value.
        byte: u8,
    },
    /// The input length (in valid base64 symbols) is invalid.
    InvalidLength {
        /// The invalid length.
        length: usize,
    },
    /// The last non-padding symbol has nonzero trailing bits that
    /// would be discarded, indicating corrupted or truncated input.
    InvalidLastSymbol {
        /// Byte offset of the invalid symbol.
        offset: usize,
        /// The invalid byte value.
        byte: u8,
    },
    /// Padding was absent, incorrect, or otherwise not as expected.
    InvalidPadding,
}

impl fmt::Display for ParseBase64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseBase64Error::InvalidByte { offset, byte } => {
                write!(f, "invalid base64 symbol {byte}, offset {offset}",)
            }
            ParseBase64Error::InvalidLength { length } => {
                write!(f, "invalid base64 input length: {length}",)
            }
            ParseBase64Error::InvalidLastSymbol { offset, byte } => {
                write!(
                    f,
                    "invalid base64 last symbol {byte}, \
                     offset {offset}",
                )
            }
            ParseBase64Error::InvalidPadding => {
                write!(f, "invalid base64 padding")
            }
        }
    }
}

impl error::Error for ParseBase64Error {}

fn from_decode_error(e: base64::DecodeError) -> ParseBase64Error {
    match e {
        base64::DecodeError::InvalidByte(offset, byte) => {
            ParseBase64Error::InvalidByte { offset, byte }
        }
        base64::DecodeError::InvalidLength(length) => {
            ParseBase64Error::InvalidLength { length }
        }
        base64::DecodeError::InvalidLastSymbol(offset, byte) => {
            ParseBase64Error::InvalidLastSymbol { offset, byte }
        }
        base64::DecodeError::InvalidPadding => ParseBase64Error::InvalidPadding,
    }
}

impl FromStr for Base64Vec {
    type Err = ParseBase64Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base64::engine::general_purpose::STANDARD
            .decode(s)
            .map(Self)
            .map_err(from_decode_error)
    }
}

impl fmt::Debug for Base64Vec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Base64Vec")
            .field(&base64::engine::general_purpose::STANDARD.encode(&self.0))
            .finish()
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

#[cfg(feature = "serde")]
mod serde_impls {
    use super::Base64Vec;
    use alloc::vec::Vec;
    use base64::Engine;
    use core::fmt;
    use serde_core::{
        Deserializer, Serializer,
        de::{SeqAccess, Visitor},
    };

    /// Serializes a byte slice as base64 if human-readable, or as
    /// raw bytes if not.
    fn serialize_bytes<S>(
        bytes: &[u8],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let encoded =
                base64::engine::general_purpose::STANDARD.encode(bytes);
            serializer.serialize_str(&encoded)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }

    /// Deserializes base64 strings (if human-readable) or byte
    /// arrays (if not) to `Vec<u8>`.
    fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_core::de::Error;

        if deserializer.is_human_readable() {
            struct Base64Visitor;

            impl<'de2> Visitor<'de2> for Base64Visitor {
                type Value = Vec<u8>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "a base64-encoded string")
                }

                fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    base64::engine::general_purpose::STANDARD
                        .decode(data)
                        .map_err(Error::custom)
                }
            }

            deserializer.deserialize_str(Base64Visitor)
        } else {
            struct BytesVisitor;

            impl<'de2> Visitor<'de2> for BytesVisitor {
                type Value = Vec<u8>;

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "a byte array")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(v.to_vec())
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(v)
                }

                fn visit_seq<A>(
                    self,
                    mut seq: A,
                ) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de2>,
                {
                    let hint = seq.size_hint().unwrap_or(0);
                    let mut out = Vec::with_capacity(hint.min(4096));
                    while let Some(byte) = seq.next_element()? {
                        out.push(byte);
                    }
                    Ok(out)
                }
            }

            deserializer.deserialize_bytes(BytesVisitor)
        }
    }

    impl Base64Vec {
        /// Serializes a byte vector as base64 if the format is
        /// human-readable, or as raw bytes otherwise.
        ///
        /// Intended for use with `#[serde(with = "Base64Vec")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::Base64Vec;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Blob {
        ///     #[serde(with = "Base64Vec")]
        ///     data: Vec<u8>,
        /// }
        ///
        /// let b = Blob { data: vec![1, 2, 3] };
        /// let json = serde_json::to_string(&b).unwrap();
        /// assert_eq!(json, r#"{"data":"AQID"}"#);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn serialize<S>(
            bytes: &[u8],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize_bytes(bytes, serializer)
        }

        /// Deserializes a byte vector from base64 if the format is
        /// human-readable, or as raw bytes otherwise.
        ///
        /// Intended for use with `#[serde(with = "Base64Vec")]`.
        ///
        /// # Examples
        ///
        /// ```
        /// use byte_wrapper::Base64Vec;
        /// use serde::{Deserialize, Serialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct Blob {
        ///     #[serde(with = "Base64Vec")]
        ///     data: Vec<u8>,
        /// }
        ///
        /// let b: Blob = serde_json::from_str(r#"{"data":"AQID"}"#).unwrap();
        /// assert_eq!(b.data, [1, 2, 3]);
        /// ```
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_bytes(deserializer)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl serde_core::Serialize for Base64Vec {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize_bytes(&self.0, serializer)
        }
    }

    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    impl<'de> serde_core::Deserialize<'de> for Base64Vec {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_bytes(deserializer).map(Self)
        }
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
            let mut extensions = crate::x_rust_type_extension("Base64Vec");
            extensions.insert("contentEncoding".into(), "base64".into());
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                format: Some("byte".into()),
                extensions,
                ..Default::default()
            })
        }
    }
}

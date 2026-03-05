/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`HasBase64Direct`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "HasBase64Direct",
///  "type": "object",
///  "required": [
///    "data"
///  ],
///  "properties": {
///    "data": {
///      "type": "string",
///      "format": "byte",
///      "contentEncoding": "base64",
///      "x-rust-type": {
///        "crate": "byte-wrapper",
///        "path": "byte_wrapper::Base64Vec",
///        "version": "0.1.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct HasBase64Direct {
    pub data: ::byte_wrapper::Base64Vec,
}

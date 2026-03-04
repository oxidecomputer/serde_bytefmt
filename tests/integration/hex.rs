// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "alloc")]
mod alloc_tests {
    use hex_literal::hex;
    use serde::{Deserialize, Serialize};

    /// Test that `HexArray` works with `#[serde(with = "...")]`.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
    struct WithHexArrayAttr {
        #[serde(with = "serde_bytefmt::HexArray::<16>")]
        x: [u8; 16],
    }

    /// Test using `HexArray` directly as a field type.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
    struct WithHexArrayDirect {
        x: serde_bytefmt::HexArray<16>,
    }

    static FIXTURE: WithHexArrayAttr =
        WithHexArrayAttr { x: hex!("0123456789abcdef0123456789abcdef") };

    static AS_JSON: &str = r#"{"x":"0123456789abcdef0123456789abcdef"}"#;
    static AS_CBOR: [u8; 20] = hex!("a16178500123456789abcdef0123456789abcdef");

    #[test]
    fn hex_serialize() {
        assert_eq!(
            serde_json::to_string(&FIXTURE)
                .expect("serializing as JSON succeeded"),
            AS_JSON,
            "JSON matched",
        );
        let mut cbor_actual: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&FIXTURE, &mut cbor_actual)
            .expect("writing to vec<u8> succeeded");

        assert_eq!(cbor_actual, AS_CBOR, "CBOR matched");
    }

    #[test]
    fn hex_deserialize() {
        let json_actual: WithHexArrayAttr = serde_json::from_str(AS_JSON)
            .expect("deserializing from JSON succeeded");
        assert_eq!(FIXTURE, json_actual, "deserializing from JSON matched");

        let cbor_actual: WithHexArrayAttr =
            ciborium::de::from_reader(&AS_CBOR[..])
                .expect("deserializing from CBOR succeeded");
        assert_eq!(FIXTURE, cbor_actual, "deserializing from CBOR succeeded");
    }

    #[test]
    fn hex_array_direct() {
        let fixture = WithHexArrayDirect {
            x: serde_bytefmt::HexArray::new(hex!(
                "0123456789abcdef0123456789abcdef"
            )),
        };

        let json = serde_json::to_string(&fixture).expect("serialized");
        assert_eq!(json, AS_JSON);

        let roundtrip: WithHexArrayDirect =
            serde_json::from_str(&json).expect("deserialized");
        assert_eq!(fixture, roundtrip);
    }
}

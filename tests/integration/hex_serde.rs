// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::HexArray;
use hex_literal::hex;
use serde::{Deserialize, Serialize};

/// Test that `HexArray` works with `#[serde(with = "...")]`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithHexArrayAttr {
    #[serde(with = "HexArray::<16>")]
    x: [u8; 16],
}

/// Test using `HexArray` directly as a field type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithHexArrayDirect {
    x: HexArray<16>,
}

static FIXTURE: WithHexArrayAttr =
    WithHexArrayAttr { x: hex!("0123456789abcdef0123456789abcdef") };

static AS_JSON: &str = r#"{"x":"0123456789abcdef0123456789abcdef"}"#;
static AS_CBOR: [u8; 20] = hex!("a16178500123456789abcdef0123456789abcdef");

#[test]
fn hex_serde_roundtrip() {
    let json = serde_json::to_string(&FIXTURE)
        .expect("serializing as JSON succeeded");
    assert_eq!(json, AS_JSON, "JSON matched");

    let json_roundtrip: WithHexArrayAttr =
        serde_json::from_str(&json)
            .expect("JSON roundtrip deserialization succeeded");
    assert_eq!(FIXTURE, json_roundtrip, "JSON roundtrip matched");

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&FIXTURE, &mut cbor)
        .expect("serializing as CBOR succeeded");
    assert_eq!(cbor, AS_CBOR, "CBOR matched");

    let cbor_roundtrip: WithHexArrayAttr =
        ciborium::de::from_reader(&cbor[..])
            .expect("CBOR roundtrip deserialization succeeded");
    assert_eq!(FIXTURE, cbor_roundtrip, "CBOR roundtrip matched");
}

/// Test that `HexArray` can deserialize from a CBOR array
/// (via `visit_seq`), not only from a CBOR byte string.
/// Some binary formats represent byte data as integer
/// sequences rather than native byte strings.
#[test]
fn hex_deserialize_from_seq() {
    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct SmallHex {
        x: HexArray<4>,
    }

    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct SmallHexAttr {
        #[serde(with = "HexArray::<4>")]
        x: [u8; 4],
    }

    // CBOR encoding of {"x": [1, 2, 3, 4]} where the value
    // is a CBOR array (major type 4), not a byte string
    // (major type 2).
    let cbor_array = hex!("a1 6178 84 01020304");

    let direct: SmallHex = ciborium::de::from_reader(&cbor_array[..])
        .expect("deserialized HexArray from CBOR array");
    assert_eq!(direct, SmallHex { x: HexArray::new([1, 2, 3, 4]) },);

    let with_attr: SmallHexAttr = ciborium::de::from_reader(&cbor_array[..])
        .expect(
            "deserialized [u8; N] with HexArray from \
                 CBOR array",
        );
    assert_eq!(with_attr, SmallHexAttr { x: [1, 2, 3, 4] });
}

/// Verify that a CBOR array with more elements than `N` is
/// rejected rather than silently truncated.
#[test]
fn hex_deserialize_from_seq_too_long() {
    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct SmallHex {
        x: HexArray<4>,
    }

    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct SmallHexAttr {
        #[serde(with = "HexArray::<4>")]
        x: [u8; 4],
    }

    // CBOR encoding of {"x": [1, 2, 3, 4, 5]} — one element
    // too many for HexArray<4>.
    let cbor_array = hex!("a1 6178 85 0102030405");

    let err = ciborium::de::from_reader::<SmallHex, _>(&cbor_array[..])
        .expect_err("trailing elements should be rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("invalid length 5"),
        "error should report the oversized length, got: {msg}",
    );
    assert!(
        msg.contains("expected a byte array [u8; 4]"),
        "error should describe the expected type, got: {msg}",
    );

    let err = ciborium::de::from_reader::<SmallHexAttr, _>(&cbor_array[..])
        .expect_err("trailing elements should be rejected with attr");
    let msg = err.to_string();
    assert!(
        msg.contains("invalid length 5"),
        "error should report the oversized length, got: {msg}",
    );
    assert!(
        msg.contains("expected a byte array [u8; 4]"),
        "error should describe the expected type, got: {msg}",
    );
}

/// JSON is human-readable, so `HexArray` expects a hex string,
/// not an array of integers. Verify that a JSON array is
/// rejected.
#[test]
fn hex_json_array_rejected() {
    let json = r#"{"x":[1,2,3,4]}"#;

    let err = serde_json::from_str::<WithHexArrayAttr>(json)
        .expect_err("JSON array should not deserialize as HexArray");
    let msg = err.to_string();
    assert!(
        msg.contains("hex string"),
        "error should mention hex string, got: {msg}",
    );

    #[derive(Debug, Deserialize)]
    struct SmallHexDirect {
        #[expect(dead_code)]
        x: HexArray<4>,
    }

    let err = serde_json::from_str::<SmallHexDirect>(json).expect_err(
        "JSON array should not deserialize as \
                     direct HexArray",
    );
    let msg = err.to_string();
    assert!(
        msg.contains("hex string"),
        "error should mention hex string, got: {msg}",
    );
}

#[test]
fn hex_array_direct() {
    let fixture = WithHexArrayDirect {
        x: HexArray::new(hex!("0123456789abcdef0123456789abcdef")),
    };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithHexArrayDirect =
        serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);
}

/// Verify that hex strings with wrong lengths produce clear error
/// messages that include the expected length.
#[test]
fn hex_wrong_length_rejected() {
    // Too short.
    let json = r#"{"x":"0102"}"#;
    let err = serde_json::from_str::<WithHexArrayAttr>(json)
        .expect_err("too-short hex should be rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("32 hex digits"),
        "error should mention expected length, got: {msg}",
    );

    // Too long.
    let json = format!(r#"{{"x":"{}"}}"#, "ab".repeat(20),);
    let err = serde_json::from_str::<WithHexArrayAttr>(&json)
        .expect_err("too-long hex should be rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("32 hex digits"),
        "error should mention expected length, got: {msg}",
    );
}

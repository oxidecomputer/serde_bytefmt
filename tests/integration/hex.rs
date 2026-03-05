// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_bytefmt::HexArray;

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
fn hex_serialize() {
    assert_eq!(
        serde_json::to_string(&FIXTURE).expect("serializing as JSON succeeded"),
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
    assert_eq!(FIXTURE, json_actual, "deserializing from JSON matched",);

    let cbor_actual: WithHexArrayAttr = ciborium::de::from_reader(&AS_CBOR[..])
        .expect("deserializing from CBOR succeeded");
    assert_eq!(FIXTURE, cbor_actual, "deserializing from CBOR succeeded",);
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
        msg.contains("hex-encoded string"),
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
        msg.contains("hex-encoded string"),
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

// -- Display and Debug formatting tests --

#[test]
fn hex_display() {
    let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h}"), "0102abff");

    // Empty array.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty}"), "");

    let h = HexArray::new([0xab, 0xcd]);

    // Right alignment (the default).
    assert_eq!(format!("{h:>10}"), "      abcd");
    assert_eq!(format!("{h:10}"), "      abcd");

    // Left alignment.
    assert_eq!(format!("{h:<10}"), "abcd      ");

    // Center alignment (even and odd padding).
    assert_eq!(format!("{h:^10}"), "   abcd   ");
    assert_eq!(format!("{h:^9}"), "  abcd   ");

    // Custom fill character.
    assert_eq!(format!("{h:_>10}"), "______abcd");
    assert_eq!(format!("{h:_<10}"), "abcd______");
    assert_eq!(format!("{h:_^10}"), "___abcd___");

    // Width smaller than or equal to content: no truncation.
    assert_eq!(format!("{h:2}"), "abcd");
    assert_eq!(format!("{h:4}"), "abcd");
}

#[test]
fn hex_debug() {
    let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
    assert_eq!(format!("{h:?}"), "HexArray(0102abff)");

    // Empty array.
    let empty = HexArray::new([]);
    assert_eq!(format!("{empty:?}"), "HexArray()");

    // Alternate flag.
    let h = HexArray::new([0xab, 0xcd]);
    assert_eq!(format!("{h:#?}"), "HexArray(\n    abcd,\n)");
}

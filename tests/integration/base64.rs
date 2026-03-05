// Copyright (c) The serde_bytefmt Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_bytefmt::{Base64Vec, ParseBase64Error};

/// Test that `Base64Vec` works with `#[serde(with = "...")]`.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithBase64VecAttr {
    #[serde(with = "Base64Vec")]
    data: Vec<u8>,
}

/// Test using `Base64Vec` directly as a field type.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct WithBase64VecDirect {
    data: Base64Vec,
}

static FIXTURE: &[u8] = &hex!("0123456789abcdef0123456789abcdef");

// Base64 encoding of the fixture bytes.
static AS_JSON: &str = r#"{"data":"ASNFZ4mrze8BI0VniavN7w=="}"#;

// CBOR: map with one key "data" and a byte string value.
static AS_CBOR: [u8; 23] =
    hex!("a1646461746150 0123456789abcdef0123456789abcdef");

fn fixture() -> WithBase64VecAttr {
    WithBase64VecAttr { data: FIXTURE.to_vec() }
}

#[test]
fn base64_serialize() {
    let fixture = fixture();

    assert_eq!(
        serde_json::to_string(&fixture).expect("serializing as JSON succeeded"),
        AS_JSON,
        "JSON matched",
    );

    let mut cbor_actual: Vec<u8> = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor_actual)
        .expect("writing to vec<u8> succeeded");

    assert_eq!(cbor_actual, AS_CBOR, "CBOR matched");
}

#[test]
fn base64_deserialize() {
    let fixture = fixture();

    let json_actual: WithBase64VecAttr = serde_json::from_str(AS_JSON)
        .expect("deserializing from JSON succeeded");
    assert_eq!(fixture, json_actual, "deserializing from JSON matched");

    let cbor_actual: WithBase64VecAttr =
        ciborium::de::from_reader(&AS_CBOR[..])
            .expect("deserializing from CBOR succeeded");
    assert_eq!(fixture, cbor_actual, "deserializing from CBOR succeeded");
}

/// Test that `Base64Vec` can deserialize from a CBOR array
/// (via `visit_seq`), not only from a CBOR byte string.
/// Some binary formats represent byte data as integer sequences
/// rather than native byte strings.
#[test]
fn base64_deserialize_from_seq() {
    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct DirectSmall {
        data: Base64Vec,
    }

    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct AttrSmall {
        #[serde(with = "Base64Vec")]
        data: Vec<u8>,
    }

    // CBOR encoding of {"data": [1, 2, 3, 4]} where the value
    // is a CBOR array (major type 4), not a byte string
    // (major type 2).
    let cbor_array = hex!("a1 6464617461 84 01020304");

    let direct: DirectSmall = ciborium::de::from_reader(&cbor_array[..])
        .expect("deserialized Base64Vec from CBOR array");
    assert_eq!(direct, DirectSmall { data: Base64Vec::new(vec![1, 2, 3, 4]) },);

    let with_attr: AttrSmall = ciborium::de::from_reader(&cbor_array[..])
        .expect("deserialized Vec<u8> with Base64Vec from CBOR array");
    assert_eq!(with_attr, AttrSmall { data: vec![1, 2, 3, 4] },);
}

/// JSON is human-readable, so `Base64Vec` expects a base64 string,
/// not an array of integers. Verify that a JSON array is rejected.
#[test]
fn base64_json_array_rejected() {
    let json = r#"{"data":[1,2,3,4]}"#;

    let err = serde_json::from_str::<WithBase64VecAttr>(json)
        .expect_err("JSON array should not deserialize as Base64Vec");
    let msg = err.to_string();
    assert!(
        msg.contains("base64-encoded string"),
        "error should mention base64 string, got: {msg}",
    );

    #[derive(Debug, Deserialize)]
    struct DirectSmall {
        #[expect(dead_code)]
        data: Base64Vec,
    }

    let err = serde_json::from_str::<DirectSmall>(json).expect_err(
        "JSON array should not deserialize as direct \
             Base64Vec",
    );
    let msg = err.to_string();
    assert!(
        msg.contains("base64-encoded string"),
        "error should mention base64 string, got: {msg}",
    );
}

#[test]
fn base64_vec_with_attr() {
    let fixture = WithBase64VecAttr { data: FIXTURE.to_vec() };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithBase64VecAttr =
        serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);
}

#[test]
fn base64_vec_direct() {
    let fixture =
        WithBase64VecDirect { data: Base64Vec::new(FIXTURE.to_vec()) };

    let json = serde_json::to_string(&fixture).expect("serialized");
    assert_eq!(json, AS_JSON);

    let roundtrip: WithBase64VecDirect =
        serde_json::from_str(&json).expect("deserialized");
    assert_eq!(fixture, roundtrip);
}

// -- FromStr tests --

#[test]
fn base64_from_str() {
    let b: Base64Vec = "AQID".parse().expect("parsed");
    assert_eq!(*b, [1, 2, 3]);

    // With padding.
    let b: Base64Vec = "AQIDBA==".parse().expect("parsed padded");
    assert_eq!(*b, [1, 2, 3, 4]);

    // Empty.
    let b: Base64Vec = "".parse().expect("parsed empty");
    assert!(b.is_empty());
}

#[test]
fn base64_from_str_invalid_byte() {
    let err = "AQ!D".parse::<Base64Vec>().expect_err("invalid byte");
    assert_eq!(
        err,
        ParseBase64Error::InvalidByte { offset: 2, byte: b'!' },
        "got: {err}",
    );
    assert!(err.to_string().contains("offset 2"), "got: {err}",);
}

#[test]
fn base64_from_str_invalid_length() {
    // A single base64 character is not a valid encoding.
    let err = "A".parse::<Base64Vec>().expect_err("invalid length");
    assert_eq!(err, ParseBase64Error::InvalidLength { length: 1 });
}

// -- Display and Debug formatting tests --

#[test]
fn base64_display() {
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b}"), "AQID");

    // With padding.
    let b = Base64Vec::new(vec![1, 2, 3, 4]);
    assert_eq!(format!("{b}"), "AQIDBA==");

    // Empty.
    let b = Base64Vec::new(vec![]);
    assert_eq!(format!("{b}"), "");

    // Width and alignment.
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:>10}"), "      AQID");
    assert_eq!(format!("{b:<10}"), "AQID      ");
    assert_eq!(format!("{b:^10}"), "   AQID   ");
    assert_eq!(format!("{b:_>10}"), "______AQID");

    // Width smaller than content: no truncation.
    assert_eq!(format!("{b:2}"), "AQID");
}

#[test]
fn base64_debug() {
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:?}"), r#"Base64Vec("AQID")"#);

    // Empty.
    let b = Base64Vec::new(vec![]);
    assert_eq!(format!("{b:?}"), r#"Base64Vec("")"#);

    // Alternate flag.
    let b = Base64Vec::new(vec![1, 2, 3]);
    assert_eq!(format!("{b:#?}"), "Base64Vec(\n    \"AQID\",\n)",);
}

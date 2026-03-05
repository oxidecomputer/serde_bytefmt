// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::Base64Vec;
use hex_literal::hex;
use serde::{Deserialize, Serialize};

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
fn base64_serde_roundtrip() {
    let fixture = fixture();

    let json = serde_json::to_string(&fixture)
        .expect("serializing as JSON succeeded");
    assert_eq!(json, AS_JSON, "JSON matched");

    let json_roundtrip: WithBase64VecAttr =
        serde_json::from_str(&json)
            .expect("JSON roundtrip deserialization succeeded");
    assert_eq!(fixture, json_roundtrip, "JSON roundtrip matched");

    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&fixture, &mut cbor)
        .expect("serializing as CBOR succeeded");
    assert_eq!(cbor, AS_CBOR, "CBOR matched");

    let cbor_roundtrip: WithBase64VecAttr =
        ciborium::de::from_reader(&cbor[..])
            .expect("CBOR roundtrip deserialization succeeded");
    assert_eq!(fixture, cbor_roundtrip, "CBOR roundtrip matched");
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

// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use byte_wrapper::Base64Vec;
use schemars08::{self as schemars, JsonSchema, schema_for};

#[test]
fn base64_vec_schema() {
    let schema = schema_for!(Base64Vec);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Base64Vec",
        "type": "string",
        "format": "byte",
        "contentEncoding": "base64"
    });
    assert_eq!(actual, expected);
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithBase64VecAttr {
    #[schemars(with = "Base64Vec")]
    data: Vec<u8>,
}

#[expect(dead_code)]
#[derive(JsonSchema)]
struct WithBase64VecDirect {
    data: Base64Vec,
}

#[test]
fn with_base64_vec_attr() {
    let schema = schema_for!(WithBase64VecAttr);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithBase64VecAttr",
        "type": "object",
        "required": ["data"],
        "properties": {
            "data": {
                "type": "string",
                "format": "byte",
                "contentEncoding": "base64"
            }
        }
    });
    assert_eq!(actual, expected);
}

#[test]
fn with_base64_vec_direct() {
    let schema = schema_for!(WithBase64VecDirect);
    let actual = serde_json::to_value(&schema).expect("serialized");
    let expected = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "WithBase64VecDirect",
        "type": "object",
        "required": ["data"],
        "properties": {
            "data": {
                "type": "string",
                "format": "byte",
                "contentEncoding": "base64"
            }
        }
    });
    assert_eq!(actual, expected);
}

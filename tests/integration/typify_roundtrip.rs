// Copyright (c) The byte-wrapper Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! End-to-end tests verifying that schemas produced by our `JsonSchema`
//! impls are correctly consumed by typify's `x-rust-type` replacement.

use quote::quote;
use schemars08::{
    self as schemars, JsonSchema, schema::RootSchema, schema_for,
};
use typify_impl::{CrateVers, TypeSpace, TypeSpaceSettings};

/// Feed a root schema into typify with byte-wrapper configured as a
/// known crate, format the output with prettyplease, and return it.
fn typify_output(root_schema: &RootSchema) -> String {
    let mut settings = TypeSpaceSettings::default();
    settings.with_crate(
        "byte-wrapper",
        CrateVers::Version("0.1.0".parse().unwrap()),
        None,
    );
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(root_schema.clone()).expect("added root schema");

    let tokens = quote! { #type_space };
    let file = syn::parse2(tokens).expect("parsed token stream");
    prettyplease::unparse(&file)
}

// -- Base64Vec --------------------------------------------------------

#[cfg(feature = "base64")]
mod base64_tests {
    use super::*;
    use byte_wrapper::Base64Vec;

    #[expect(dead_code)]
    #[derive(JsonSchema)]
    struct HasBase64Direct {
        data: Base64Vec,
    }

    #[test]
    fn base64_direct() {
        let code = typify_output(&schema_for!(HasBase64Direct));
        expectorate::assert_contents(
            "tests/output/typify-base64-direct.rs",
            &code,
        );
    }

    #[expect(dead_code)]
    #[derive(JsonSchema)]
    struct HasBase64WithAttr {
        #[schemars(with = "Base64Vec")]
        data: Vec<u8>,
    }

    #[test]
    fn base64_with_attr() {
        let code = typify_output(&schema_for!(HasBase64WithAttr));
        expectorate::assert_contents(
            "tests/output/typify-base64-with-attr.rs",
            &code,
        );
    }
}

// -- HexArray ---------------------------------------------------------

#[cfg(feature = "hex")]
mod hex_tests {
    use super::*;
    use byte_wrapper::HexArray;

    #[expect(dead_code)]
    #[derive(JsonSchema)]
    struct HasHexDirect {
        checksum: HexArray<32>,
    }

    #[test]
    fn hex_direct() {
        let code = typify_output(&schema_for!(HasHexDirect));
        expectorate::assert_contents(
            "tests/output/typify-hex-direct.rs",
            &code,
        );
    }

    #[expect(dead_code)]
    #[derive(JsonSchema)]
    struct HasHexWithAttr {
        #[schemars(with = "HexArray<32>")]
        checksum: [u8; 32],
    }

    #[test]
    fn hex_with_attr() {
        let code = typify_output(&schema_for!(HasHexWithAttr));
        expectorate::assert_contents(
            "tests/output/typify-hex-with-attr.rs",
            &code,
        );
    }
}

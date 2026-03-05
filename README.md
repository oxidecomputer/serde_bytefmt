<!-- cargo-sync-rdme title [[ -->
# byte-wrapper
<!-- cargo-sync-rdme ]] -->

<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/byte-wrapper.svg?)
[![crates.io](https://img.shields.io/crates/v/byte-wrapper.svg?logo=rust)](https://crates.io/crates/byte-wrapper)
[![docs.rs](https://img.shields.io/docsrs/byte-wrapper.svg?logo=docs.rs)](https://docs.rs/byte-wrapper)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->

<!-- cargo-sync-rdme rustdoc [[ -->
Newtype wrappers for byte arrays and vectors with hex and base64
formatting.

This crate provides wrapper types that display byte data in
human-readable encodings. [`HexArray<N>`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/hex_array/struct.HexArray.html) encodes fixed-length byte
arrays as hex strings, and [`Base64Vec`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/base64_vec/struct.Base64Vec.html) encodes variable-length
byte vectors as base64 strings.

With the `serde` feature, both types implement `Serialize` and
`Deserialize`, encoding as human-readable strings (hex or base64) in text
formats like JSON, and as efficient raw bytes in binary formats like [CBOR].
You do not have to use the newtypes in your own type definitions; you can
refer to them via `#[serde(with = "...")]` instead.

## Types

* [`HexArray<N>`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/hex_array/struct.HexArray.html) encodes a fixed-length byte array as a hex
  string. (Requires the `hex` feature.)
* [`Base64Vec`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/base64_vec/struct.Base64Vec.html) encodes a variable-length byte vector as a base64
  string. (Requires the `base64` feature.)

## Examples

````rust
use byte_wrapper::HexArray;

let h = HexArray::new([0x01, 0x02, 0xab, 0xff]);
assert_eq!(h.to_string(), "0102abff");

let parsed: HexArray<4> = "0102abff".parse().unwrap();
assert_eq!(parsed, h);
````

With the **`serde`** feature:

````rust
use byte_wrapper::HexArray;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Record {
    checksum: HexArray<32>,
}
````

Using `#[serde(with = "...")]` on an existing byte array:

````rust
use byte_wrapper::HexArray;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Record {
    #[serde(with = "HexArray::<32>")]
    checksum: [u8; 32],
}
````

## Alternatives

The [`serde_with`] crate provides [`Hex`] and [`Base64`] format
adapters via its `serde_as` macro. These always serialize as
hex/base64 strings, even in binary formats like CBOR — they do
not check [`is_human_readable()`] to switch to raw bytes.
`serde_with` also does not provide:

* Standalone newtype wrappers with [`Display`], [`FromStr`],
  [`Deref`], etc. Its adapters only work as serde field
  annotations.
* [`JsonSchema`] (schemars) impls. If you derive `JsonSchema` on
  a struct that uses `serde_as`, schemars won’t know about the
  hex/base64 encoding and will generate an incorrect schema (an
  array of integers rather than a string).

If you only need serde support for human-readable formats and
don’t need newtypes or schema generation, `serde_with` is a
reasonable alternative.

## Features

* **`hex`**: enables [`HexArray`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/hex_array/struct.HexArray.html). *Enabled by default.*
* **`base64`**: enables [`Base64Vec`](https://docs.rs/byte-wrapper/0.1.0/byte_wrapper/base64_vec/struct.Base64Vec.html) (implies `alloc`).
  *Enabled by default.*
* **`alloc`**: enables `alloc` support (required by `base64`).
* **`serde`**: implements `Serialize` and `Deserialize` for
  enabled types. *Not enabled by default.*
* **`schemars08`**: derives `JsonSchema` for enabled types.
  *Not enabled by default.*

[CBOR]: https://cbor.io/
[`serde_with`]: https://docs.rs/serde_with
[`Hex`]: https://docs.rs/serde_with/latest/serde_with/hex/struct.Hex.html
[`Base64`]: https://docs.rs/serde_with/latest/serde_with/base64/struct.Base64.html
[`is_human_readable()`]: https://docs.rs/serde/latest/serde/trait.Serializer.html#method.is_human_readable
[`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
[`FromStr`]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html
[`Deref`]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html
[`JsonSchema`]: https://docs.rs/schemars/0.8/schemars/trait.JsonSchema.html
<!-- cargo-sync-rdme ]] -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

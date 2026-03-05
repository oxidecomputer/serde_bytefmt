<!-- cargo-sync-rdme title [[ -->
# serde_bytefmt
<!-- cargo-sync-rdme ]] -->

<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/serde_bytefmt.svg?)
[![crates.io](https://img.shields.io/crates/v/serde_bytefmt.svg?logo=rust)](https://crates.io/crates/serde_bytefmt)
[![docs.rs](https://img.shields.io/docsrs/serde_bytefmt.svg?logo=docs.rs)](https://docs.rs/serde_bytefmt)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->

<!-- cargo-sync-rdme rustdoc [[ -->
Serialize byte arrays and vectors as bytes or as human-readable strings,
depending on the format.

Many binary formats (e.g. [CBOR]) can natively represent byte sequences,
while text formats (e.g. JSON) cannot. This crate bridges the gap: it
serializes byte data as hex or base64 strings in [human-readable formats],
and as efficient raw bytes in binary formats.

## Types

* [`HexArray<N>`](https://docs.rs/serde_bytefmt/0.1.0/serde_bytefmt/hex_array/struct.HexArray.html) encodes a fixed-length byte array as a hex string.
* [`Base64Vec`](https://docs.rs/serde_bytefmt/0.1.0/serde_bytefmt/base64_vec/struct.Base64Vec.html) encodes a variable-length byte vector as a base64 string.
  (The `alloc` feature is required.)

These types can be used directly as struct fields, or be applied to
existing `[u8; N]` / `Vec<u8>` fields via `#[serde(with = "...")]`.

## Examples

Using [`HexArray`](https://docs.rs/serde_bytefmt/0.1.0/serde_bytefmt/hex_array/struct.HexArray.html) as a field type:

````rust
use serde::{Deserialize, Serialize};
use serde_bytefmt::HexArray;

#[derive(Serialize, Deserialize)]
struct Record {
    checksum: HexArray<32>,
}
````

Using `#[serde(with = "...")]` on an existing byte array:

````rust
use serde::{Deserialize, Serialize};
use serde_bytefmt::HexArray;

#[derive(Serialize, Deserialize)]
struct Record {
    #[serde(with = "HexArray::<32>")]
    checksum: [u8; 32],
}
````

## Features

* **`alloc`**: enables [`Base64Vec`](https://docs.rs/serde_bytefmt/0.1.0/serde_bytefmt/base64_vec/struct.Base64Vec.html). *Enabled by default.*
* **`schemars08`**: derives `JsonSchema` for both types.
  *Not enabled by default.*

[CBOR]: https://cbor.io/
[human-readable formats]: https://docs.rs/serde_core/latest/serde_core/trait.Serializer.html#method.is_human_readable
<!-- cargo-sync-rdme ]] -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

# Serde fast flatten

This crate provides a drop-in replacement for `serde` auto-derive macros with a
faster `flatten` implementation that, incidentally, is also compatible with
non-self-describing formats.

## Summary

The `Deserialize` implementation generated by `serde`'s auto-derive macro for
flattened struct fields, is quite slow compared to the unflattened version,
because it needs to store the unrecognized fields in the parent structure for
subsequent deserilization by the child structure.

This crate provides traits that allow a more performant implementation of struct
flattening, along with auto-derive macros to help implementing them. It also
includes some tests and a benchmark (see below).

## Usage

In `Cargo.toml`, add the following dependency:

```
serde_fast_flatten = { version = "0.1.1", features = ["derive"] }
```

In your code, replace the auto-derive macros `serde::Serialize` and
`serde::Deserialize` with `serde::SerializeFields` and
`serde::DeserializeFields`, respectively. This is needed for all structs
containing a `#[serde(flatten)]` annotation and all structs referenced by such
annotation.

```
// use serde::{Serialize, Deserialize};
use serde_fast_flatten::{SerializeFields, DeserializeFields};

// #[derive(Serialize, Deserialize)]
#[derive(SerializeFields, DeserializeFields)]
struct Paged<T> {
    #[serde(flatten)]
    params: PageParams,
    items: Vec<T>,
}

// #[derive(Serialize, Deserialize)]
#[derive(SerializeFields, DeserializeFields)]
struct PageParams {
    page: u32,
    limit: u32,
}
```

That's it! The auto-derive macros will generate implementations for
`SerializeFields` and `DeserializeFields`, as well as `Serialize` and
`Deserialize` implementations based on them.

## Compatibility

The test suite currently includes roundtrip tests for `serde_json`,
`serde_yaml`, `serde_cbor` (normal and packed repr), `bincode` and
`bitcode`.

For `serde_json` and `serde_yaml`, the serialized output is the same
as the standard `serde` implementation. Using `serde_cbor`, the output
differs. `bincode` and `bitcode` both fail on `serde`'s auto-derived
flatten implementation, and work correctly using the
`serde_fast_flatten`-based implementation.

The auto-derive macros currently only work on `structs`, not on `enums`, and are
only tested with `#[serde(flatten)]` attributes.

## Benchmarks

A simple benchmark can be run using `cargo bench`. It serializes and
deserializes a simple structure with `serde_json`. On my machine (Intel Ultra 9
185H) this gives the following results:

| Name                          | Low       | Median    | High      |
| ----------------------------- | --------- | --------- | --------- |
| serialize/serde_flatten       | 53.415 µs | 53.522 µs | 53.634 µs |
| serialize/serde_unflattened   | 76.089 µs | 76.279 µs | 76.483 µs |
| serialize/fast_flatten        | 54.318 µs | 54.467 µs | 54.647 µs |
|                               |           |           |           |
| deserialize/serde_flatten     | 279.46 µs | 279.89 µs | 280.32 µs |
| deserialize/serde_unflattened | 122.75 µs | 123.06 µs | 123.43 µs |
| deserialize/fast_flatten      | 124.03 µs | 124.20 µs | 124.38 µs |

When _serializing_, there is little difference between the implementation
auto-derived by `serde` and the one written using `serde_fast_flatten`. The
unflattened representation serializes slower, probably due to the increased
output size.

For _deserialization_, however, there is a big difference between the
performance of `serde`'s implementation and the one by `serde_fast_flatten`.
While `serde`'s implementation takes about twice as long to deserialize the test
values from the flattened representation as deserializing from an unflattened
representation, `serde_fast_flatten`'s performance approaches that of the
unflattened representation.

## Roadmap

- Add support for internally-tagged and adjacently-tagged enums (with fast path
  if the tag is the first field encountered, and fallback to content
  deserializer otherwise)
- Add support for all serde attributes
- Add more example types and formats to test suite

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

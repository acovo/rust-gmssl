# gmssl-rs

GmSSL bindings for the Rust programming language.

[Documentation](https://docs.rs/gmssl).


## Release Support

The current supported release of `gmssl` is 0.1 and `gmssl-sys` is 0.1.


## Build & Test

Only support GmSSL 3.1.0+, must set env DEP_OPENSSL_VERSION_NUMBER before compile.

```
export DEP_OPENSSL_VERSION_NUMBER=806354944

cargo build

cargo test -- --nocapture

```

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed under the terms of both the Apache License,
Version 2.0 and the MIT license without any additional terms or conditions.

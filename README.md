# RiteLinked -- HashMap-like containers that hold their key-value pairs in a user controllable order

[![Latest Version](https://img.shields.io/crates/v/ritelinked.svg)](https://crates.io/crates/ritelinked)
[![API Documentation](https://docs.rs/ritelinked/badge.svg)](https://docs.rs/ritelinked)

This crate is a fork of [hashlink](https://github.com/kyren/hashlink) 
to implement more up to date versions of `LinkedHashMap` `LinkedHashSet`.
It use [griddle](https://github.com/jonhoo/griddle) to support amortization.

## Credit

The code in this crate that is copied verbatim from `hashlink`.
Just deleted `LruCache` and replaced `hashbrown` with `griddle` by default.

## License

This library is licensed the same as [hashlink](https://github.com/kyren/hashlink), 
it is licensed under either of:

* MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
* Apache License 2.0 [LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0

at your option.

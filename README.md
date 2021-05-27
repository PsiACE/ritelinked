# RiteLinked -- HashMap-like containers that hold their key-value pairs in a user controllable order

[![Latest Version](https://img.shields.io/crates/v/ritelinked.svg)](https://crates.io/crates/ritelinked)
[![API Documentation](https://docs.rs/ritelinked/badge.svg)](https://docs.rs/ritelinked)

[RiteLinked](https://crates.io/crates/ritelinked) provides more up to date versions of `LinkedHashMap` & `LinkedHashSet` . 
You can easily use it on `std` or `no_std` environment.

Support some practical feature combinations to help you better embed them in existing code: `serde`, `inline-more` etc. 
Especially, it uses `griddle` by default, if you have a lot of data, it can effectively help you reduce the possible tail delay. (Of course, `hashbrown` can also be used) 

## Usage

Add `ritelinked` to `Cargo.toml`:

```toml
ritelinked = "x.y.z"
```

Write some code like this:

``` rust
let mut lru_cache = LinkedHashMap::new();
let key = "key".to_owned();
let _cached_val = lru_cache
    .raw_entry_mut()
    .from_key(&key)
    .or_insert_with(|| (key.clone(), 42));
```

## Benchmark

```
ritelinked              time:   [165.09 ns 165.40 ns 165.79 ns]                       
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

hashlink                time:   [168.11 ns 168.48 ns 168.96 ns]                     
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

linked-hash-map         time:   [370.98 ns 382.59 ns 397.44 ns]                            
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```

## Credit

It is a fork of the popular crate [hashlink](https://crates.io/crates/hashlink), but more adjustments and improvements have been made to the code .

## License

This library is licensed the same as [hashlink](https://github.com/kyren/hashlink), 
it is licensed under either of:

* MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
* Apache License 2.0 [LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0

at your option.

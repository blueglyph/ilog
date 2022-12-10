# ilog

[![crate](https://img.shields.io/crates/v/ilog.svg)](https://crates.io/crates/ilog)
[![documentation](https://docs.rs/ilog/badge.svg)](https://docs.rs/ilog)
[![build status](https://github.com/blueglyph/ilog/actions/workflows/master.yml/badge.svg)](https://github.com/blueglyph/ilog/actions)
[![crate](https://img.shields.io/crates/l/ilog.svg)](https://github.com/blueglyph/ilog/blob/master/LICENSE-MIT)

Base 10 and 2 logarithm functions for integer types.

The `IntLog` trait defines the following methods:

```rust
fn log10(self) -> usize
fn log2(self) -> usize
fn checked_log10(self) -> Option<usize>
fn checked_log2(self) -> Option<usize>
```

The `log2` and `log10` methods are optimized for the integer width and are
`[inline]` as long as the code remains small enough. They typically use constant tables
that are only stored once, even if the methods using them are inlined multiple times.

The **checked** versions of the methods, `checked_log2` and `checked_log10`,
return `None` if the logarithm is undefined for the parameter value, whereas the unchecked
methods mentioned above simply panic. A default implementation is provided in the trait, and in
most cases they needn't be overidden.

## Examples

```rust
use ilog::IntLog;

let hundred: u32 = 100;
assert_eq!(hundred.log10(), 2);
assert_eq!(u32::log10(99), 1);

let value: u64 = 256;
assert_eq!(value.log2(), 8);
assert_eq!(u64::log2(255), 7);

assert_eq!(u32::checked_log2(63), Some(5));
assert_eq!(0_u32.checked_log2(), None);
```

## Usage

Add this dependency to your `Cargo.toml` file:

```toml
[dependencies]
ilog = "0"
```

## Compatibility

The `ilog` crate is tested for rustc 1.65 and greater.

## Releases

[RELEASES.md](RELEASES.md) keeps a log of all the releases.

## License

Licensed under [MIT license](https://choosealicense.com/licenses/mit/).

# ilog

Base 10 and 2 logarithm functions for integer types.

The `IntLog` trait defines the following functions:

```rust
fn log10(self) -> usize
fn log2(self) -> usize
fn checked_log10(self) -> Option<usize>
fn checked_log2(self) -> Option<usize>
```

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

## License

Licensed under [MIT license](http://opensource.org/licenses/MIT).
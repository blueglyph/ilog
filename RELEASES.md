# 1.0.0 (2023-03-07)

- Added implementations for `&mut` and `Box<>`

# 0.1.4 (2022-12-14)

- Made the crate no-std.

# 0.1.3 (2022-12-12)

- Added a comment about unstable_name_collisions for versions of rustc earlier than 1.65.

# 0.1.2 (2022-12-10)

- Added reference implementations for all types.
- Removed default `IntLog` implementations.
- Removed `num-traits` dependency.

# 0.1.1 (2022-12-09)

- IntLog::{log2, log10} implementation for all integer primitives, adding u8, i8, u16, i16, i32, i64, u128, i128, usize, isize.
- Macros have been used to generate the implementations and tests.

# 0.1.0 (2022-12-09)

First version

- IntLog::{log2, log10} implementation for u32, u64.
# `tai_to_gpst_nanos`

```rust
pub const fn tai_to_gpst_nanos(tai: TaiNanos) -> GpstNanos
```

Convert TAI nanoseconds to GPST nanoseconds. Always exact.

## Example

```rust
use leap_sec::{tai_to_gpst_nanos, TaiNanos, GpstNanos};

let gpst = tai_to_gpst_nanos(TaiNanos(1_700_000_037_500_000_000));
assert_eq!(gpst, GpstNanos(1_700_000_018_500_000_000));
```

## See Also

- [`gpst_to_tai_nanos`](gpst_to_tai_nanos.md) — the inverse
- [`tai_to_gpst`](tai_to_gpst.md) — seconds variant

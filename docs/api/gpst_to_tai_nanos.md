# `gpst_to_tai_nanos`

```rust
pub const fn gpst_to_tai_nanos(gpst: GpstNanos) -> TaiNanos
```

Convert GPST nanoseconds to TAI nanoseconds. Always exact.

## Example

```rust
use leap_sec::{gpst_to_tai_nanos, TaiNanos, GpstNanos};

let tai = gpst_to_tai_nanos(GpstNanos(1_700_000_018_500_000_000));
assert_eq!(tai, TaiNanos(1_700_000_037_500_000_000));
```

## See Also

- [`tai_to_gpst_nanos`](tai_to_gpst_nanos.md) — the inverse
- [`gpst_to_tai`](gpst_to_tai.md) — seconds variant

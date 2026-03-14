# `gpst_to_tai`

```rust
pub const fn gpst_to_tai(gpst: GpstSeconds) -> TaiSeconds
```

Convert GPS Time to TAI seconds. `TAI = GPST + 19s`.

This is a **free function** — the 19-second offset is a physical constant.

## Example

```rust
use leap_sec::{gpst_to_tai, TaiSeconds, GpstSeconds};

let tai = gpst_to_tai(GpstSeconds(1_700_000_018));
assert_eq!(tai, TaiSeconds(1_700_000_037));
```

## See Also

- [`tai_to_gpst`](tai_to_gpst.md) — the inverse
- [`gpst_to_tai_nanos`](gpst_to_tai_nanos.md) — nanosecond variant

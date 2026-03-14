# `tai_to_gpst`

```rust
pub const fn tai_to_gpst(tai: TaiSeconds) -> GpstSeconds
```

Convert TAI seconds to GPS Time. `GPST = TAI − 19s`.

This is a **free function**, not a method on `LeapSeconds`, because the
19-second offset is a physical constant — always valid, no table required.

## Example

```rust
use leap_sec::{tai_to_gpst, TaiSeconds, GpstSeconds};

let gpst = tai_to_gpst(TaiSeconds(1_700_000_037));
assert_eq!(gpst, GpstSeconds(1_700_000_018));
```

## See Also

- [`gpst_to_tai`](gpst_to_tai.md) — the inverse
- [`tai_to_gpst_nanos`](tai_to_gpst_nanos.md) — nanosecond variant

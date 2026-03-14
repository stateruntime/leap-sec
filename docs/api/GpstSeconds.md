# `GpstSeconds`

```rust
pub struct GpstSeconds(pub i64);
```

Continuous seconds count in the GPS time scale.

GPST is offset from TAI by exactly 19 seconds: `GPST = TAI − 19`.

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::GpstSeconds;
assert_eq!(format!("{}", GpstSeconds(1_700_000_018)), "1700000018 GPST");
```

## Conversions

```rust
use leap_sec::{GpstSeconds, GpstNanos};

let ns: GpstNanos = GpstSeconds(1_700_000_018).into();
assert_eq!(ns, GpstNanos(1_700_000_018_000_000_000));
```

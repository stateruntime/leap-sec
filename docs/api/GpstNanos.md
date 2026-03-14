# `GpstNanos`

```rust
pub struct GpstNanos(pub i128);
```

Continuous nanoseconds in the GPS time scale.

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::GpstNanos;
assert_eq!(format!("{}", GpstNanos(1_700_000_018_000_000_000)), "1700000018000000000 GPST");
```

## Methods

### `to_seconds_floor`

```rust
pub const fn to_seconds_floor(self) -> GpstSeconds
```

Truncate to whole seconds, rounding toward negative infinity.

# `TaiNanos`

```rust
pub struct TaiNanos(pub i128);
```

Continuous nanoseconds in the TAI scale.

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::TaiNanos;
assert_eq!(format!("{}", TaiNanos(1_700_000_037_000_000_000)), "1700000037000000000 TAI");
```

## Methods

### `to_seconds_floor`

```rust
pub const fn to_seconds_floor(self) -> TaiSeconds
```

Truncate to whole seconds, rounding toward negative infinity.

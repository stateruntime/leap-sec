# `UtcUnixNanos`

```rust
pub struct UtcUnixNanos(pub i128);
```

Unix-epoch nanoseconds in the UTC scale. Uses `i128` to hold the full range
without overflow.

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::UtcUnixNanos;
assert_eq!(format!("{}", UtcUnixNanos(1_700_000_000_000_000_000)), "1700000000000000000 UTC");
```

## Methods

### `to_seconds_floor`

```rust
pub const fn to_seconds_floor(self) -> UtcUnixSeconds
```

Truncate to whole seconds, rounding toward negative infinity.

```rust
use leap_sec::{UtcUnixNanos, UtcUnixSeconds};

let ns = UtcUnixNanos(1_700_000_000_999_999_999);
assert_eq!(ns.to_seconds_floor(), UtcUnixSeconds(1_700_000_000));
```

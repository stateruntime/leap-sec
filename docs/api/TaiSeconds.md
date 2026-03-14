# `TaiSeconds`

```rust
pub struct TaiSeconds(pub i64);
```

Continuous seconds count in the TAI scale (no leap seconds).

TAI is ahead of UTC by a varying number of whole seconds (37 as of 2017-01-01).

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::TaiSeconds;
assert_eq!(format!("{}", TaiSeconds(1_700_000_037)), "1700000037 TAI");
```

## Conversions

```rust
use leap_sec::{TaiSeconds, TaiNanos};

let ns: TaiNanos = TaiSeconds(1_700_000_037).into();
assert_eq!(ns, TaiNanos(1_700_000_037_000_000_000));
```

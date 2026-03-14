# `Error`

```rust
pub enum Error {
    OutOfRange { requested: i64, valid_start: i64, valid_end: i64 },
    TableExpired { expires_at: i64 },
    InvalidTable { detail: &'static str },
}
```

Errors that can occur during leap-second conversions or table construction.

## Variants

### `OutOfRange`

The requested timestamp is before the first entry in the leap-second table.
Contains the requested value and the valid range for diagnostics.

```rust
use leap_sec::prelude::*;

let err = LeapSeconds::known().utc_to_tai(UtcUnixSeconds(0)).unwrap_err();
assert!(matches!(err, Error::OutOfRange { .. }));
println!("{err}");
// "timestamp 0 is outside the leap-second table range [63072000, 1483228800]"
```

### `TableExpired`

The leap-second table has expired. Contains the expiration timestamp.
The built-in `known()` table never produces this error.

### `InvalidTable`

The table data is invalid. Produced by [`LeapSecondsBuilder::build()`](LeapSecondsBuilder.md)
when the table is empty or has non-monotonic timestamps.

```rust
use leap_sec::prelude::*;

let err = LeapSeconds::builder().build().unwrap_err();
assert!(matches!(err, Error::InvalidTable { .. }));
```

## Traits

- `Display` — human-readable error messages via `core::fmt`
- `std::error::Error` — behind `#[cfg(feature = "std")]`
- `Debug`, `Clone`, `PartialEq`, `Eq`

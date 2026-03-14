# `LeapSecondsBuilder`

```rust
pub struct LeapSecondsBuilder { /* ... */ }
```

A builder for constructing custom [`LeapSeconds`](LeapSeconds.md) tables.
Requires the `std` feature.

## Methods

### `new` / `LeapSeconds::builder`

```rust
pub const fn new() -> Self
```

Create an empty builder. Also available as `LeapSeconds::builder()`.

### `add`

```rust
pub fn add(self, utc: UtcUnixSeconds, tai_minus_utc: i32) -> Self
```

Add a leap-second entry. `utc` is when the offset takes effect,
`tai_minus_utc` is the cumulative offset from this point forward.

### `expires_at`

```rust
pub const fn expires_at(self, at: UtcUnixSeconds) -> Self
```

Set an expiration timestamp for the table.

### `build`

```rust
pub fn build(self) -> Result<LeapSeconds, Error>
```

Build the table. Returns [`Error::InvalidTable`](Error.md) if the table
is empty or timestamps are not monotonically increasing.

## Example

```rust
use leap_sec::prelude::*;

let table = LeapSeconds::builder()
    .add(UtcUnixSeconds(63_072_000), 10)   // 1972-01-01
    .add(UtcUnixSeconds(78_796_800), 11)   // 1972-07-01
    .build()
    .unwrap();

let tai = table.utc_to_tai(UtcUnixSeconds(70_000_000)).unwrap();
assert_eq!(tai, TaiSeconds(70_000_010));
```

## Validation Errors

```rust
use leap_sec::prelude::*;

// Empty table
assert!(LeapSeconds::builder().build().is_err());

// Non-monotonic timestamps
assert!(LeapSeconds::builder()
    .add(UtcUnixSeconds(200), 10)
    .add(UtcUnixSeconds(100), 11)
    .build()
    .is_err());
```

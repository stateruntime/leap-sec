# `LeapSeconds::tai_to_utc`

```rust
pub fn tai_to_utc(&self, tai: TaiSeconds) -> Result<UtcUnixSeconds, Error>
```

Convert TAI seconds to a UTC Unix timestamp.

## Logic

Binary-searches the table in TAI space (each entry's TAI boundary is
`utc_unix + tai_minus_utc`), then returns `tai - offset`.

## Errors

Returns [`Error::OutOfRange`](Error.md) if `tai` is before the first TAI
boundary (63,072,010 for the built-in table).

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let utc = leaps.tai_to_utc(TaiSeconds(1_700_000_037)).unwrap();
assert_eq!(utc, UtcUnixSeconds(1_700_000_000));
```

## Roundtrip

`tai_to_utc(utc_to_tai(x)) == x` for all valid UTC timestamps.

## See Also

- [`utc_to_tai`](utc_to_tai.md) — the inverse
- [`tai_to_utc_nanos`](tai_to_utc_nanos.md) — nanosecond variant

# `LeapSeconds::expires_at`

```rust
pub fn expires_at(&self) -> Option<UtcUnixSeconds>
```

Returns the expiration timestamp, if one was set.

## Details

- The built-in `known()` table returns `None`
- Tables built with [`LeapSecondsBuilder::expires_at()`](LeapSecondsBuilder.md)
  return `Some(timestamp)`
- Future IERS-parsed tables will carry the file's expiration date

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();
assert_eq!(leaps.expires_at(), None);

let custom = LeapSeconds::builder()
    .add(UtcUnixSeconds(100), 10)
    .expires_at(UtcUnixSeconds(1_000))
    .build()
    .unwrap();
assert_eq!(custom.expires_at(), Some(UtcUnixSeconds(1_000)));
```

## See Also

- [`is_expired`](is_expired.md) — check if the table has expired

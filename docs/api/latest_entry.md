# `LeapSeconds::latest_entry`

```rust
pub fn latest_entry(&self) -> (UtcUnixSeconds, i32)
```

Returns the most recent leap-second entry as `(effective_utc, tai_minus_utc)`.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();
let (date, offset) = leaps.latest_entry();

assert_eq!(date, UtcUnixSeconds(1_483_228_800));  // 2017-01-01
assert_eq!(offset, 37);
```

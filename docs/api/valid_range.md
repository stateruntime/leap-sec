# `LeapSeconds::valid_range`

```rust
pub fn valid_range(&self) -> (UtcUnixSeconds, UtcUnixSeconds)
```

Returns the first and last entry timestamps in the table.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();
let (start, end) = leaps.valid_range();

assert_eq!(start, UtcUnixSeconds(63_072_000));     // 1972-01-01
assert_eq!(end, UtcUnixSeconds(1_483_228_800));     // 2017-01-01
```

## Note

Timestamps *after* `end` are still valid for conversion — they use the last
known offset. Only timestamps *before* `start` return
[`Error::OutOfRange`](Error.md).

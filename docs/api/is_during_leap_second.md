# `LeapSeconds::is_during_leap_second`

```rust
pub fn is_during_leap_second(&self, utc: UtcUnixSeconds) -> bool
```

Returns `true` if the given UTC timestamp falls exactly on a positive
leap-second insertion.

## The Ambiguity Problem

When a leap second is inserted, the UTC clock reads `23:59:60` — but POSIX
timestamps cannot represent this. The timestamp for `23:59:60` has the same
integer value as the following `00:00:00`. This method detects that ambiguous
instant.

## Returns `false` For

- The 1972-01-01 epoch (not an insertion — it's the initial offset)
- Normal timestamps (not on a leap-second boundary)
- Negative leap seconds (the offset *decreases* — no extra second exists)

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// 2017-01-01 — a leap second was inserted here
assert!(leaps.is_during_leap_second(UtcUnixSeconds(1_483_228_800)));

// Normal timestamp
assert!(!leaps.is_during_leap_second(UtcUnixSeconds(1_700_000_000)));

// 1972-01-01 epoch — not an insertion
assert!(!leaps.is_during_leap_second(UtcUnixSeconds(63_072_000)));
```

## Note

When `is_during_leap_second` returns `true`, [`utc_to_tai`](utc_to_tai.md)
still succeeds — it returns the TAI instant *after* the insertion (matching
the POSIX convention).

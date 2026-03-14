# `LeapSeconds::known`

```rust
pub fn known() -> &'static LeapSeconds
```

Returns the built-in table with all historical leap seconds through 2017-01-01.

## Details

- Contains 28 entries: the 1972-01-01 epoch (offset 10) plus 27 insertions
- Works in `no_std` — no allocation, no I/O, fully deterministic
- Returns a `&'static` reference — zero-cost, no cloning
- Timestamps after 2017-01-01 use the last known offset (37)
- Never expires — [`is_expired()`](is_expired.md) always returns `false`

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();
let (_, offset) = leaps.latest_entry();
assert_eq!(offset, 37);
```

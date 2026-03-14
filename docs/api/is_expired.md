# `LeapSeconds::is_expired`

```rust
pub fn is_expired(&self) -> bool
```

Returns whether the table has expired.

## Current Behavior

Always returns `false` in v0.1. The built-in `known()` table has no
expiration concept. Future versions with IERS file parsing will use clock
access to check against the table's expiration timestamp.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();
assert!(!leaps.is_expired());
```

## See Also

- [`expires_at`](expires_at.md) — get the expiration timestamp

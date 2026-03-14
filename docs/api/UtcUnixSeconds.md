# `UtcUnixSeconds`

```rust
pub struct UtcUnixSeconds(pub i64);
```

Unix-like seconds count in the UTC scale.

This is the standard POSIX timestamp — seconds since 1970-01-01T00:00:00 UTC.
Leap seconds are folded: the 61st second (`23:59:60`) shares the same value
as the following `00:00:00`.

## Derives

`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

## Display

```rust
use leap_sec::UtcUnixSeconds;
assert_eq!(format!("{}", UtcUnixSeconds(1_700_000_000)), "1700000000 UTC");
```

## Conversions

```rust
use leap_sec::{UtcUnixSeconds, UtcUnixNanos};

// Promote to nanoseconds (lossless)
let ns: UtcUnixNanos = UtcUnixSeconds(1_700_000_000).into();
assert_eq!(ns, UtcUnixNanos(1_700_000_000_000_000_000));
```

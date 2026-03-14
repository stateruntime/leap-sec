//! Sub-second precision conversions using nanosecond types.
//!
//! Run with: `cargo run --example nanoseconds`

#![allow(clippy::print_stdout)]

use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    let leaps = LeapSeconds::known();

    // A UTC timestamp with sub-second precision (0.5s past the epoch second)
    let utc_ns = UtcUnixNanos(1_700_000_000_500_000_000);

    // Convert to TAI — the offset (37s) is applied in whole seconds,
    // the fractional part is preserved exactly
    let tai_ns = leaps.utc_to_tai_nanos(utc_ns)?;
    println!("{utc_ns} -> {tai_ns}");

    // Roundtrip preserves precision
    let back = leaps.tai_to_utc_nanos(tai_ns)?;
    assert_eq!(back, utc_ns);
    println!("Roundtrip: {back} (exact match)");

    // Lossless promotion from seconds to nanoseconds
    let utc_sec = UtcUnixSeconds(1_700_000_000);
    let promoted: UtcUnixNanos = utc_sec.into();
    println!("{utc_sec} promoted to {promoted}");

    // Truncate nanoseconds back to seconds (floor)
    let truncated = utc_ns.to_seconds_floor();
    println!("{utc_ns} floored to {truncated}");

    Ok(())
}

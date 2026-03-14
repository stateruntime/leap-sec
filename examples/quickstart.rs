//! Quick start: convert UTC to TAI and back.
//!
//! Run with: `cargo run --example quickstart`

#![allow(clippy::print_stdout)]

use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    // Get the built-in leap-second table
    let leaps = LeapSeconds::known();

    // Convert a UTC timestamp to TAI
    let utc = UtcUnixSeconds(1_700_000_000); // 2023-11-14 22:13:20 UTC
    let tai = leaps.utc_to_tai(utc)?;

    println!("UTC: {utc}");
    println!("TAI: {tai}");
    println!("Offset: {} seconds", tai.0 - utc.0);

    // Convert back — exact roundtrip
    let back = leaps.tai_to_utc(tai)?;
    assert_eq!(back, utc);
    println!("Roundtrip: {back} (matches original)");

    // Check the current offset
    let offset = leaps.tai_utc_offset(utc)?;
    println!("TAI-UTC offset at {utc}: {offset}s");

    Ok(())
}

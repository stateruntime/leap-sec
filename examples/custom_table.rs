//! Building a custom leap-second table for testing.
//!
//! Run with: `cargo run --example custom_table`

#![allow(clippy::print_stdout, clippy::unwrap_used)]

use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    // Build a small custom table
    let table = LeapSeconds::builder()
        .add(UtcUnixSeconds(1000), 10) // offset 10 from t=1000
        .add(UtcUnixSeconds(2000), 11) // offset 11 from t=2000
        .add(UtcUnixSeconds(3000), 12) // offset 12 from t=3000
        .expires_at(UtcUnixSeconds(5000))
        .build()?;

    // Conversions work just like the built-in table
    let tai = table.utc_to_tai(UtcUnixSeconds(1500))?;
    println!("UTC 1500 -> {tai} (offset 10)");

    let tai = table.utc_to_tai(UtcUnixSeconds(2500))?;
    println!("UTC 2500 -> {tai} (offset 11)");

    // Table inspection
    let (start, end) = table.valid_range();
    println!("Range: {start} to {end}");
    println!("Expires at: {:?}", table.expires_at());

    // Validation catches bad input
    let empty = LeapSeconds::builder().build();
    println!("Empty table: {}", empty.unwrap_err());

    let non_monotonic = LeapSeconds::builder()
        .add(UtcUnixSeconds(2000), 10)
        .add(UtcUnixSeconds(1000), 11)
        .build();
    println!("Non-monotonic: {}", non_monotonic.unwrap_err());

    Ok(())
}

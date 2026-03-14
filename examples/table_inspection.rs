//! Inspecting the leap-second table.
//!
//! Run with: `cargo run --example table_inspection`

#![allow(clippy::print_stdout)]

use leap_sec::prelude::*;

fn main() {
    let leaps = LeapSeconds::known();

    // Valid range
    let (start, end) = leaps.valid_range();
    println!("Table covers: {start} to {end}");

    // Latest entry
    let (date, offset) = leaps.latest_entry();
    println!("Latest leap second: {date}, TAI-UTC = {offset}s");

    // Expiration
    println!("Expired: {}", leaps.is_expired());
    println!("Expires at: {:?}", leaps.expires_at());
}

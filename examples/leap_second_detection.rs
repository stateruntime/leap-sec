//! Detecting the ambiguous 61st second (23:59:60).
//!
//! Run with: `cargo run --example leap_second_detection`

#![allow(clippy::print_stdout)]

use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    let leaps = LeapSeconds::known();

    // 2017-01-01T00:00:00 UTC — a leap second was inserted here.
    // In POSIX time, the leap second (2016-12-31T23:59:60) folds to
    // the same integer as 2017-01-01T00:00:00.
    let ambiguous = UtcUnixSeconds(1_483_228_800);

    if leaps.is_during_leap_second(ambiguous) {
        println!("{ambiguous} is during a leap-second insertion.");
        println!("The UTC wall clock reads 23:59:60 at this instant.");
    }

    // The conversion still works — it returns the TAI instant after the insertion
    let tai = leaps.utc_to_tai(ambiguous)?;
    println!("{ambiguous} -> {tai}");

    // Normal timestamps are not ambiguous
    let normal = UtcUnixSeconds(1_700_000_000);
    println!(
        "{normal} during leap second: {}",
        leaps.is_during_leap_second(normal)
    );

    Ok(())
}

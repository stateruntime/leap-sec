//! TAI ↔ GPST conversions using the constant 19-second offset.
//!
//! Run with: `cargo run --example gps_time`

#![allow(clippy::print_stdout)]

use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    // TAI and GPST differ by a constant 19 seconds — no table needed.
    let tai = TaiSeconds(1_700_000_037);
    let gpst = tai_to_gpst(tai);

    println!("{tai} -> {gpst}");
    println!("Difference: {} seconds", tai.0 - gpst.0);

    // Roundtrip
    let back = gpst_to_tai(gpst);
    assert_eq!(back, tai);

    // UTC → GPST requires the table (goes through TAI)
    let leaps = LeapSeconds::known();
    let utc = UtcUnixSeconds(1_700_000_000);
    let gpst = leaps.utc_to_gpst(utc)?;
    println!("{utc} -> {gpst} (via TAI)");

    // And back
    let back = leaps.gpst_to_utc(gpst)?;
    assert_eq!(back, utc);
    println!("Roundtrip: {back}");

    Ok(())
}

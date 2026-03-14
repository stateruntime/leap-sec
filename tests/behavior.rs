//! Behavior tests for the `leap-sec` public surface.
//!
//! These tests use the `behave` DSL so they read like specifications.

#![allow(clippy::unwrap_used, clippy::no_effect_underscore_binding)]

use behave::prelude::*;
use leap_sec::prelude::*;

behave! {
    "leap-sec crate" {
        // =======================================================================
        // Leap-second boundary tests — all 27 insertions
        // =======================================================================

        "utc_to_tai at all 27 leap-second boundaries" {
            setup {
                let leaps = LeapSeconds::known();

                // (utc_unix, expected_offset_before, expected_offset_at)
                // The first entry (1972-01-01, offset 10) is the epoch, not an insertion.
                // Entries 2..28 are the 27 actual insertions.
                let boundaries: Vec<(i64, i32, i32)> = vec![
                    // 1972-07-01 offset goes from 10 to 11
                    (78_796_800, 10, 11),
                    // 1973-01-01 offset goes from 11 to 12
                    (94_694_400, 11, 12),
                    // 1974-01-01
                    (126_230_400, 12, 13),
                    // 1975-01-01
                    (157_766_400, 13, 14),
                    // 1976-01-01
                    (189_302_400, 14, 15),
                    // 1977-01-01
                    (220_924_800, 15, 16),
                    // 1978-01-01
                    (252_460_800, 16, 17),
                    // 1979-01-01
                    (283_996_800, 17, 18),
                    // 1980-01-01
                    (315_532_800, 18, 19),
                    // 1981-07-01
                    (362_793_600, 19, 20),
                    // 1982-07-01
                    (394_329_600, 20, 21),
                    // 1983-07-01
                    (425_865_600, 21, 22),
                    // 1985-07-01
                    (489_024_000, 22, 23),
                    // 1988-01-01
                    (567_993_600, 23, 24),
                    // 1990-01-01
                    (631_152_000, 24, 25),
                    // 1991-01-01
                    (662_688_000, 25, 26),
                    // 1992-07-01
                    (709_948_800, 26, 27),
                    // 1993-07-01
                    (741_484_800, 27, 28),
                    // 1994-07-01
                    (773_020_800, 28, 29),
                    // 1996-01-01
                    (820_454_400, 29, 30),
                    // 1997-07-01
                    (867_715_200, 30, 31),
                    // 1999-01-01
                    (915_148_800, 31, 32),
                    // 2006-01-01
                    (1_136_073_600, 32, 33),
                    // 2009-01-01
                    (1_230_768_000, 33, 34),
                    // 2012-07-01
                    (1_341_100_800, 34, 35),
                    // 2015-07-01
                    (1_435_708_800, 35, 36),
                    // 2017-01-01
                    (1_483_228_800, 36, 37),
                ];
            }

            "one second before each insertion uses the previous offset" {
                for &(utc, offset_before, _) in &boundaries {
                    let before = UtcUnixSeconds(utc - 1);
                    let tai = leaps.utc_to_tai(before).unwrap();
                    expect!(tai.0).to_equal(before.0 + i64::from(offset_before))?;
                }
            }

            "at each insertion timestamp uses the new offset" {
                for &(utc, _, offset_at) in &boundaries {
                    let at = UtcUnixSeconds(utc);
                    let tai = leaps.utc_to_tai(at).unwrap();
                    expect!(tai.0).to_equal(at.0 + i64::from(offset_at))?;
                }
            }

            "one second after each insertion uses the new offset" {
                for &(utc, _, offset_at) in &boundaries {
                    let after = UtcUnixSeconds(utc + 1);
                    let tai = leaps.utc_to_tai(after).unwrap();
                    expect!(tai.0).to_equal(after.0 + i64::from(offset_at))?;
                }
            }
        }

        // =======================================================================
        // The initial 1972-01-01 epoch
        // =======================================================================

        "1972-01-01 epoch" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "utc_to_tai at 1972-01-01 applies offset 10" {
                let utc = UtcUnixSeconds(63_072_000);
                let tai = leaps.utc_to_tai(utc).unwrap();
                expect!(tai).to_equal(TaiSeconds(63_072_010))?;
            }

            "tai_utc_offset at 1972-01-01 is 10" {
                let offset = leaps.tai_utc_offset(UtcUnixSeconds(63_072_000)).unwrap();
                expect!(offset).to_equal(10)?;
            }
        }

        // =======================================================================
        // Pre-1972 range
        // =======================================================================

        "pre-1972 timestamps" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "utc_to_tai returns OutOfRange for timestamps before 1972-01-01" {
                let result = leaps.utc_to_tai(UtcUnixSeconds(0));
                expect!(result).to_be_err()?;
            }

            "tai_to_utc returns OutOfRange for TAI before first entry" {
                // TAI boundary of first entry = 63_072_000 + 10 = 63_072_010
                let result = leaps.tai_to_utc(TaiSeconds(63_072_009));
                expect!(result).to_be_err()?;
            }

            "error is OutOfRange variant" {
                let err = leaps.utc_to_tai(UtcUnixSeconds(0)).unwrap_err();
                expect!(matches!(err, Error::OutOfRange { requested: 0, .. })).to_be_true()?;
            }
        }

        // =======================================================================
        // Post-2017 range — last offset (37) applies
        // =======================================================================

        "post-2017 timestamps" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "utc_to_tai uses offset 37 for timestamps after 2017-01-01" {
                let utc = UtcUnixSeconds(1_700_000_000);
                let tai = leaps.utc_to_tai(utc).unwrap();
                expect!(tai).to_equal(TaiSeconds(1_700_000_037))?;
            }

            "tai_utc_offset is 37 for far-future timestamps" {
                let offset = leaps.tai_utc_offset(UtcUnixSeconds(2_000_000_000)).unwrap();
                expect!(offset).to_equal(37)?;
            }

            "tai_to_utc uses offset 37 for far-future TAI" {
                let tai = TaiSeconds(2_000_000_037);
                let utc = leaps.tai_to_utc(tai).unwrap();
                expect!(utc).to_equal(UtcUnixSeconds(2_000_000_000))?;
            }
        }

        // =======================================================================
        // Roundtrip: utc_to_tai |> tai_to_utc == identity
        // =======================================================================

        "roundtrip conversions" {
            setup {
                let leaps = LeapSeconds::known();
                let samples: Vec<i64> = vec![
                    63_072_000,       // 1972-01-01
                    78_796_800,       // 1972-07-01 (leap second boundary)
                    100_000_000,      // 1973 mid-year
                    500_000_000,      // 1985
                    1_000_000_000,    // 2001
                    1_483_228_800,    // 2017-01-01 (last leap second)
                    1_700_000_000,    // 2023
                    2_000_000_000,    // 2033
                ];
            }

            "utc -> tai -> utc roundtrips exactly" {
                for &ts in &samples {
                    let utc = UtcUnixSeconds(ts);
                    let tai = leaps.utc_to_tai(utc).unwrap();
                    let back = leaps.tai_to_utc(tai).unwrap();
                    expect!(back).to_equal(utc)?;
                }
            }

            "tai -> utc -> tai roundtrips exactly" {
                for &ts in &samples {
                    let utc = UtcUnixSeconds(ts);
                    let tai = leaps.utc_to_tai(utc).unwrap();
                    let back_utc = leaps.tai_to_utc(tai).unwrap();
                    let back_tai = leaps.utc_to_tai(back_utc).unwrap();
                    expect!(back_tai).to_equal(tai)?;
                }
            }
        }

        // =======================================================================
        // is_during_leap_second
        // =======================================================================

        "is_during_leap_second" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "returns true at each of the 27 insertion timestamps" {
                let insertion_timestamps: Vec<i64> = vec![
                    78_796_800,       // 1972-07-01
                    94_694_400,       // 1973-01-01
                    126_230_400,      // 1974-01-01
                    157_766_400,      // 1975-01-01
                    189_302_400,      // 1976-01-01
                    220_924_800,      // 1977-01-01
                    252_460_800,      // 1978-01-01
                    283_996_800,      // 1979-01-01
                    315_532_800,      // 1980-01-01
                    362_793_600,      // 1981-07-01
                    394_329_600,      // 1982-07-01
                    425_865_600,      // 1983-07-01
                    489_024_000,      // 1985-07-01
                    567_993_600,      // 1988-01-01
                    631_152_000,      // 1990-01-01
                    662_688_000,      // 1991-01-01
                    709_948_800,      // 1992-07-01
                    741_484_800,      // 1993-07-01
                    773_020_800,      // 1994-07-01
                    820_454_400,      // 1996-01-01
                    867_715_200,      // 1997-07-01
                    915_148_800,      // 1999-01-01
                    1_136_073_600,    // 2006-01-01
                    1_230_768_000,    // 2009-01-01
                    1_341_100_800,    // 2012-07-01
                    1_435_708_800,    // 2015-07-01
                    1_483_228_800,    // 2017-01-01
                ];
                for &ts in &insertion_timestamps {
                    let utc = UtcUnixSeconds(ts);
                    expect!(leaps.is_during_leap_second(utc)).to_be_true()?;
                }
            }

            "returns false for the 1972-01-01 epoch (not an insertion)" {
                let utc = UtcUnixSeconds(63_072_000);
                expect!(leaps.is_during_leap_second(utc)).to_be_false()?;
            }

            "returns false for normal timestamps" {
                let normals = vec![
                    UtcUnixSeconds(100_000_000),
                    UtcUnixSeconds(1_000_000_000),
                    UtcUnixSeconds(1_700_000_000),
                ];
                for utc in normals {
                    expect!(leaps.is_during_leap_second(utc)).to_be_false()?;
                }
            }
        }

        // =======================================================================
        // TAI ↔ GPST free functions
        // =======================================================================

        "tai_to_gpst and gpst_to_tai" {
            "tai_to_gpst subtracts 19 seconds" {
                let tai = TaiSeconds(1_700_000_037);
                let gpst = tai_to_gpst(tai);
                expect!(gpst).to_equal(GpstSeconds(1_700_000_018))?;
            }

            "gpst_to_tai adds 19 seconds" {
                let gpst = GpstSeconds(1_700_000_018);
                let tai = gpst_to_tai(gpst);
                expect!(tai).to_equal(TaiSeconds(1_700_000_037))?;
            }

            "roundtrip tai -> gpst -> tai" {
                let tai = TaiSeconds(1_000_000_000);
                let back = gpst_to_tai(tai_to_gpst(tai));
                expect!(back).to_equal(tai)?;
            }

            "nanos variant subtracts 19e9" {
                let tai = TaiNanos(1_700_000_037_500_000_000);
                let gpst = tai_to_gpst_nanos(tai);
                expect!(gpst).to_equal(GpstNanos(1_700_000_018_500_000_000))?;
            }

            "nanos roundtrip gpst -> tai -> gpst" {
                let gpst = GpstNanos(1_000_000_000_123_456_789);
                let back = tai_to_gpst_nanos(gpst_to_tai_nanos(gpst));
                expect!(back).to_equal(gpst)?;
            }
        }

        // =======================================================================
        // UTC ↔ GPST via table
        // =======================================================================

        "utc_to_gpst and gpst_to_utc" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "utc_to_gpst composes utc->tai->gpst" {
                let utc = UtcUnixSeconds(1_700_000_000);
                let gpst = leaps.utc_to_gpst(utc).unwrap();
                // TAI = 1_700_000_037, GPST = TAI - 19 = 1_700_000_018
                expect!(gpst).to_equal(GpstSeconds(1_700_000_018))?;
            }

            "gpst_to_utc roundtrips with utc_to_gpst" {
                let utc = UtcUnixSeconds(1_700_000_000);
                let gpst = leaps.utc_to_gpst(utc).unwrap();
                let back = leaps.gpst_to_utc(gpst).unwrap();
                expect!(back).to_equal(utc)?;
            }
        }

        // =======================================================================
        // Nanosecond conversions
        // =======================================================================

        "nanosecond conversions" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "utc_to_tai_nanos preserves sub-second precision" {
                let utc = UtcUnixNanos(1_700_000_000_500_000_000);
                let tai = leaps.utc_to_tai_nanos(utc).unwrap();
                expect!(tai).to_equal(TaiNanos(1_700_000_037_500_000_000))?;
            }

            "tai_to_utc_nanos preserves sub-second precision" {
                let tai = TaiNanos(1_700_000_037_500_000_000);
                let utc = leaps.tai_to_utc_nanos(tai).unwrap();
                expect!(utc).to_equal(UtcUnixNanos(1_700_000_000_500_000_000))?;
            }

            "nanos roundtrip through utc -> tai -> utc" {
                let utc = UtcUnixNanos(1_700_000_000_123_456_789);
                let tai = leaps.utc_to_tai_nanos(utc).unwrap();
                let back = leaps.tai_to_utc_nanos(tai).unwrap();
                expect!(back).to_equal(utc)?;
            }

            "utc_to_gpst_nanos works" {
                let utc = UtcUnixNanos(1_700_000_000_250_000_000);
                let gpst = leaps.utc_to_gpst_nanos(utc).unwrap();
                // TAI = utc + 37s in nanos, GPST = TAI - 19s in nanos
                expect!(gpst).to_equal(GpstNanos(1_700_000_018_250_000_000))?;
            }

            "gpst_to_utc_nanos roundtrips" {
                let utc = UtcUnixNanos(1_700_000_000_750_000_000);
                let gpst = leaps.utc_to_gpst_nanos(utc).unwrap();
                let back = leaps.gpst_to_utc_nanos(gpst).unwrap();
                expect!(back).to_equal(utc)?;
            }
        }

        // =======================================================================
        // From<Seconds> for Nanos and to_seconds_floor
        // =======================================================================

        "seconds-nanos conversions" {
            "UtcUnixSeconds promotes to UtcUnixNanos" {
                let sec = UtcUnixSeconds(1_700_000_000);
                let ns: UtcUnixNanos = sec.into();
                expect!(ns).to_equal(UtcUnixNanos(1_700_000_000_000_000_000))?;
            }

            "TaiSeconds promotes to TaiNanos" {
                let sec = TaiSeconds(1_700_000_037);
                let ns: TaiNanos = sec.into();
                expect!(ns).to_equal(TaiNanos(1_700_000_037_000_000_000))?;
            }

            "GpstSeconds promotes to GpstNanos" {
                let sec = GpstSeconds(1_700_000_018);
                let ns: GpstNanos = sec.into();
                expect!(ns).to_equal(GpstNanos(1_700_000_018_000_000_000))?;
            }

            "to_seconds_floor truncates fractional nanoseconds" {
                let ns = UtcUnixNanos(1_700_000_000_999_999_999);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(UtcUnixSeconds(1_700_000_000))?;
            }

            "to_seconds_floor with exact seconds" {
                let ns = TaiNanos(1_700_000_037_000_000_000);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(TaiSeconds(1_700_000_037))?;
            }
        }

        // =======================================================================
        // Display trait
        // =======================================================================

        "Display trait" {
            "UtcUnixSeconds shows scale label" {
                let utc = UtcUnixSeconds(1_700_000_000);
                let s = format!("{utc}");
                expect!(s).to_equal("1700000000 UTC".to_string())?;
            }

            "TaiSeconds shows scale label" {
                let tai = TaiSeconds(1_700_000_037);
                let s = format!("{tai}");
                expect!(s).to_equal("1700000037 TAI".to_string())?;
            }

            "GpstSeconds shows scale label" {
                let gpst = GpstSeconds(1_700_000_018);
                let s = format!("{gpst}");
                expect!(s).to_equal("1700000018 GPST".to_string())?;
            }

            "UtcUnixNanos shows scale label" {
                let utc = UtcUnixNanos(1_700_000_000_000_000_000);
                let s = format!("{utc}");
                expect!(s).to_equal("1700000000000000000 UTC".to_string())?;
            }

            "TaiNanos shows scale label" {
                let tai = TaiNanos(1_700_000_037_000_000_000);
                let s = format!("{tai}");
                expect!(s).to_equal("1700000037000000000 TAI".to_string())?;
            }

            "GpstNanos shows scale label" {
                let gpst = GpstNanos(1_700_000_018_000_000_000);
                let s = format!("{gpst}");
                expect!(s).to_equal("1700000018000000000 GPST".to_string())?;
            }
        }

        // =======================================================================
        // Table inspection
        // =======================================================================

        "table inspection" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "valid_range covers 1972-01-01 through 2017-01-01" {
                let (start, end) = leaps.valid_range();
                expect!(start).to_equal(UtcUnixSeconds(63_072_000))?;
                expect!(end).to_equal(UtcUnixSeconds(1_483_228_800))?;
            }

            "is_expired returns false for known table" {
                expect!(leaps.is_expired()).to_be_false()?;
            }

            "expires_at returns None for known table" {
                expect!(leaps.expires_at()).to_be_none()?;
            }

            "latest_entry returns 2017-01-01 with offset 37" {
                let (date, offset) = leaps.latest_entry();
                expect!(date).to_equal(UtcUnixSeconds(1_483_228_800))?;
                expect!(offset).to_equal(37)?;
            }
        }

        // =======================================================================
        // tai_utc_offset and tai_utc_offset_at_tai
        // =======================================================================

        "offset queries" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "tai_utc_offset returns 10 at 1972-01-01" {
                let offset = leaps.tai_utc_offset(UtcUnixSeconds(63_072_000)).unwrap();
                expect!(offset).to_equal(10)?;
            }

            "tai_utc_offset returns 37 at 2017-01-01" {
                let offset = leaps.tai_utc_offset(UtcUnixSeconds(1_483_228_800)).unwrap();
                expect!(offset).to_equal(37)?;
            }

            "tai_utc_offset returns 37 for far future" {
                let offset = leaps.tai_utc_offset(UtcUnixSeconds(2_000_000_000)).unwrap();
                expect!(offset).to_equal(37)?;
            }

            "tai_utc_offset_at_tai returns 10 at first TAI boundary" {
                // First TAI boundary = 63_072_000 + 10 = 63_072_010
                let offset = leaps.tai_utc_offset_at_tai(TaiSeconds(63_072_010)).unwrap();
                expect!(offset).to_equal(10)?;
            }

            "tai_utc_offset_at_tai returns 37 for modern TAI" {
                let offset = leaps.tai_utc_offset_at_tai(TaiSeconds(1_700_000_037)).unwrap();
                expect!(offset).to_equal(37)?;
            }
        }

        // =======================================================================
        // Custom table builder
        // =======================================================================

        "custom table builder" {
            "builds a small table and converts correctly" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 10)
                    .add(UtcUnixSeconds(200), 11)
                    .add(UtcUnixSeconds(300), 12)
                    .build()
                    .unwrap();

                // Before second entry: offset 10
                let tai = table.utc_to_tai(UtcUnixSeconds(150)).unwrap();
                expect!(tai).to_equal(TaiSeconds(160))?;

                // At second entry: offset 11
                let tai = table.utc_to_tai(UtcUnixSeconds(200)).unwrap();
                expect!(tai).to_equal(TaiSeconds(211))?;

                // After third entry: offset 12
                let tai = table.utc_to_tai(UtcUnixSeconds(400)).unwrap();
                expect!(tai).to_equal(TaiSeconds(412))?;
            }

            "builder with expires_at sets the expiration" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 10)
                    .expires_at(UtcUnixSeconds(1_000))
                    .build()
                    .unwrap();

                expect!(table.expires_at()).to_be_some_with(UtcUnixSeconds(1_000))?;
            }

            "builder roundtrip works" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 10)
                    .add(UtcUnixSeconds(200), 11)
                    .build()
                    .unwrap();

                let utc = UtcUnixSeconds(150);
                let tai = table.utc_to_tai(utc).unwrap();
                let back = table.tai_to_utc(tai).unwrap();
                expect!(back).to_equal(utc)?;
            }
        }

        // =======================================================================
        // Builder validation errors
        // =======================================================================

        "builder validation" {
            "empty table returns InvalidTable error" {
                let err = LeapSeconds::builder().build().unwrap_err();
                expect!(matches!(err, Error::InvalidTable { .. })).to_be_true()?;
            }

            "non-monotonic timestamps return InvalidTable error" {
                let err = LeapSeconds::builder()
                    .add(UtcUnixSeconds(200), 10)
                    .add(UtcUnixSeconds(100), 11)
                    .build()
                    .unwrap_err();
                expect!(matches!(err, Error::InvalidTable { .. })).to_be_true()?;
            }

            "duplicate timestamps return InvalidTable error" {
                let err = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 10)
                    .add(UtcUnixSeconds(100), 11)
                    .build()
                    .unwrap_err();
                expect!(matches!(err, Error::InvalidTable { .. })).to_be_true()?;
            }
        }

        // =======================================================================
        // Error Display
        // =======================================================================

        "error display" {
            "OutOfRange error displays correctly" {
                let err = Error::OutOfRange {
                    requested: 0,
                    valid_start: 63_072_000,
                    valid_end: 1_483_228_800,
                };
                let s = format!("{err}");
                expect!(s).to_contain_substr("outside the leap-second table range")?;
            }

            "InvalidTable error displays correctly" {
                let err = Error::InvalidTable {
                    detail: "table must contain at least one entry",
                };
                let s = format!("{err}");
                expect!(s).to_contain_substr("invalid leap-second table")?;
            }
        }

        // =======================================================================
        // Prelude re-exports
        // =======================================================================

        "prelude re-exports all public types" {
            {
                // Verify all types are accessible via the prelude.
                let _leaps: &LeapSeconds = LeapSeconds::known();
                let _utc = UtcUnixSeconds(0);
                let _tai = TaiSeconds(0);
                let _gpst = GpstSeconds(0);
                let _utc_ns = UtcUnixNanos(0);
                let _tai_ns = TaiNanos(0);
                let _gpst_ns = GpstNanos(0);
                let _f = tai_to_gpst;
                let _g = gpst_to_tai;
                let _fn = tai_to_gpst_nanos;
                let _gn = gpst_to_tai_nanos;
                let _: Result<(), Error> = Ok(());
            }
            expect!(true).to_be_true()?;
        }

        // =======================================================================
        // Edge cases: negative leap seconds (custom table)
        // =======================================================================

        "negative leap seconds via custom table" {
            setup {
                // Simulate a negative leap second: offset drops from 11 to 10
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(1000), 10)
                    .add(UtcUnixSeconds(2000), 11)
                    .add(UtcUnixSeconds(3000), 10)  // negative leap second
                    .build()
                    .unwrap();
            }

            "utc_to_tai uses decreased offset after negative leap second" {
                let tai = table.utc_to_tai(UtcUnixSeconds(3500)).unwrap();
                expect!(tai).to_equal(TaiSeconds(3510))?;
            }

            "roundtrip through negative leap second" {
                let utc = UtcUnixSeconds(3500);
                let tai = table.utc_to_tai(utc).unwrap();
                let back = table.tai_to_utc(tai).unwrap();
                expect!(back).to_equal(utc)?;
            }

            "is_during_leap_second returns false for negative leap second" {
                // At the negative insertion point, offset decreased — no ambiguous second
                expect!(table.is_during_leap_second(UtcUnixSeconds(3000))).to_be_false()?;
            }

            "is_during_leap_second returns true for the positive insertion" {
                // The offset increase at t=2000 is a normal positive insertion
                expect!(table.is_during_leap_second(UtcUnixSeconds(2000))).to_be_true()?;
            }
        }

        // =======================================================================
        // Edge cases: is_during_leap_second boundary (±1)
        // =======================================================================

        "is_during_leap_second boundary precision" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "returns false one second before each insertion" {
                let insertions: Vec<i64> = vec![
                    78_796_800, 94_694_400, 1_483_228_800,
                ];
                for ts in insertions {
                    expect!(leaps.is_during_leap_second(UtcUnixSeconds(ts - 1))).to_be_false()?;
                }
            }

            "returns false one second after each insertion" {
                let insertions: Vec<i64> = vec![
                    78_796_800, 94_694_400, 1_483_228_800,
                ];
                for ts in insertions {
                    expect!(leaps.is_during_leap_second(UtcUnixSeconds(ts + 1))).to_be_false()?;
                }
            }
        }

        // =======================================================================
        // Edge cases: negative nanoseconds floor division
        // =======================================================================

        "negative nanos floor division" {
            "UtcUnixNanos(-1) floors to UtcUnixSeconds(-1)" {
                let ns = UtcUnixNanos(-1);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(UtcUnixSeconds(-1))?;
            }

            "UtcUnixNanos(-999_999_999) floors to UtcUnixSeconds(-1)" {
                let ns = UtcUnixNanos(-999_999_999);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(UtcUnixSeconds(-1))?;
            }

            "UtcUnixNanos(-1_000_000_000) floors to UtcUnixSeconds(-1)" {
                let ns = UtcUnixNanos(-1_000_000_000);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(UtcUnixSeconds(-1))?;
            }

            "UtcUnixNanos(-1_000_000_001) floors to UtcUnixSeconds(-2)" {
                let ns = UtcUnixNanos(-1_000_000_001);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(UtcUnixSeconds(-2))?;
            }

            "TaiNanos(-500_000_000) floors to TaiSeconds(-1)" {
                let ns = TaiNanos(-500_000_000);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(TaiSeconds(-1))?;
            }
        }

        // =======================================================================
        // Edge cases: nanosecond conversions at leap-second boundaries
        // =======================================================================

        "nanosecond conversions at leap-second boundaries" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "nanos at 1972-07-01 insertion uses new offset" {
                // The 1972-07-01 insertion: offset changes from 10 to 11
                let utc_ns = UtcUnixNanos(78_796_800_000_000_000); // exactly at boundary
                let tai_ns = leaps.utc_to_tai_nanos(utc_ns).unwrap();
                // New offset 11 applies
                expect!(tai_ns).to_equal(TaiNanos(78_796_811_000_000_000))?;
            }

            "nanos one nanosecond before 1972-07-01 insertion uses old offset" {
                let utc_ns = UtcUnixNanos(78_796_799_999_999_999);
                let tai_ns = leaps.utc_to_tai_nanos(utc_ns).unwrap();
                // Old offset 10 applies
                expect!(tai_ns).to_equal(TaiNanos(78_796_809_999_999_999))?;
            }

            "nanos roundtrip at leap-second boundary" {
                let utc_ns = UtcUnixNanos(1_483_228_800_500_000_000); // 0.5s after 2017-01-01
                let tai_ns = leaps.utc_to_tai_nanos(utc_ns).unwrap();
                let back = leaps.tai_to_utc_nanos(tai_ns).unwrap();
                expect!(back).to_equal(utc_ns)?;
            }
        }

        // =======================================================================
        // Edge cases: tai_to_utc at exact TAI boundaries
        // =======================================================================

        "tai_to_utc at TAI boundaries" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "tai at old boundary returns old offset" {
                // At 1972-07-01: UTC=78_796_800, old offset=10, new offset=11
                // Last TAI with old offset = 78_796_800 + 10 - 1 = 78_796_809
                let utc = leaps.tai_to_utc(TaiSeconds(78_796_809)).unwrap();
                expect!(utc).to_equal(UtcUnixSeconds(78_796_799))?; // one second before boundary
            }

            "tai at new boundary returns new offset" {
                // First TAI with new offset = 78_796_800 + 11 = 78_796_811
                let utc = leaps.tai_to_utc(TaiSeconds(78_796_811)).unwrap();
                expect!(utc).to_equal(UtcUnixSeconds(78_796_800))?; // exactly at boundary
            }

            "tai in gap between old and new boundary" {
                // TAI 78_796_810 = 78_796_800 + 10 is the TAI boundary of old entry
                // This is the "leap second moment" — the table says offset 10 applies here
                let utc = leaps.tai_to_utc(TaiSeconds(78_796_810)).unwrap();
                expect!(utc).to_equal(UtcUnixSeconds(78_796_800))?;
            }
        }

        // =======================================================================
        // Edge cases: single-entry custom table
        // =======================================================================

        "single entry custom table" {
            "builds and converts with one entry" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(1000), 10)
                    .build()
                    .unwrap();

                let tai = table.utc_to_tai(UtcUnixSeconds(5000)).unwrap();
                expect!(tai).to_equal(TaiSeconds(5010))?;
            }

            "single entry table rejects timestamps before entry" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(1000), 10)
                    .build()
                    .unwrap();

                let result = table.utc_to_tai(UtcUnixSeconds(999));
                expect!(result).to_be_err()?;
            }

            "single entry table valid_range returns same start and end" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(1000), 10)
                    .build()
                    .unwrap();

                let (start, end) = table.valid_range();
                expect!(start).to_equal(end)?;
            }
        }

        // =======================================================================
        // Edge cases: gpst_to_utc error path
        // =======================================================================

        "gpst_to_utc error path" {
            "gpst_to_utc returns OutOfRange for GPST before table range" {
                let leaps = LeapSeconds::known();
                // GPST 0 → TAI 19 → before first TAI boundary (63_072_010)
                let result = leaps.gpst_to_utc(GpstSeconds(0));
                expect!(result).to_be_err()?;
            }
        }

        // =======================================================================
        // Edge cases: TableExpired error Display
        // =======================================================================

        "TableExpired error display" {
            "formats correctly" {
                let err = Error::TableExpired { expires_at: 1_700_000_000 };
                let s = format!("{err}");
                expect!(s.contains("expired at 1700000000") && s.contains("load a newer table")).to_be_true()?;
            }
        }

        // =======================================================================
        // Edge cases: custom table inspection
        // =======================================================================

        "custom table inspection" {
            "valid_range on custom table" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 5)
                    .add(UtcUnixSeconds(500), 6)
                    .build()
                    .unwrap();

                let (start, end) = table.valid_range();
                expect!(start).to_equal(UtcUnixSeconds(100))?;
                expect!(end).to_equal(UtcUnixSeconds(500))?;
            }

            "latest_entry on custom table" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 5)
                    .add(UtcUnixSeconds(500), 6)
                    .build()
                    .unwrap();

                let (date, offset) = table.latest_entry();
                expect!(date).to_equal(UtcUnixSeconds(500))?;
                expect!(offset).to_equal(6)?;
            }
        }

        // =======================================================================
        // Missing error paths
        // =======================================================================

        "error paths for all conversion methods" {
            setup {
                let leaps = LeapSeconds::known();
            }

            "tai_utc_offset returns OutOfRange for pre-1972 UTC" {
                let result = leaps.tai_utc_offset(UtcUnixSeconds(0));
                expect!(result).to_be_err()?;
            }

            "tai_utc_offset_at_tai returns OutOfRange for pre-range TAI" {
                let result = leaps.tai_utc_offset_at_tai(TaiSeconds(0));
                expect!(result).to_be_err()?;
            }

            "utc_to_tai_nanos returns OutOfRange for pre-1972 nanos" {
                let result = leaps.utc_to_tai_nanos(UtcUnixNanos(0));
                expect!(result).to_be_err()?;
            }

            "tai_to_utc_nanos returns OutOfRange for pre-range TAI nanos" {
                let result = leaps.tai_to_utc_nanos(TaiNanos(0));
                expect!(result).to_be_err()?;
            }

            "utc_to_gpst returns OutOfRange for pre-1972 UTC" {
                let result = leaps.utc_to_gpst(UtcUnixSeconds(0));
                expect!(result).to_be_err()?;
            }

            "utc_to_gpst_nanos returns OutOfRange for pre-1972 nanos" {
                let result = leaps.utc_to_gpst_nanos(UtcUnixNanos(0));
                expect!(result).to_be_err()?;
            }

            "gpst_to_utc_nanos returns OutOfRange for pre-range GPST nanos" {
                let result = leaps.gpst_to_utc_nanos(GpstNanos(0));
                expect!(result).to_be_err()?;
            }
        }

        // =======================================================================
        // Missing roundtrip directions
        // =======================================================================

        "additional roundtrip directions" {
            "tai_nanos -> utc_nanos -> tai_nanos roundtrips" {
                let leaps = LeapSeconds::known();
                let tai = TaiNanos(1_700_000_037_500_000_000);
                let utc = leaps.tai_to_utc_nanos(tai).unwrap();
                let back = leaps.utc_to_tai_nanos(utc).unwrap();
                expect!(back).to_equal(tai)?;
            }

            "gpst -> utc -> gpst roundtrips (seconds)" {
                let leaps = LeapSeconds::known();
                let gpst = GpstSeconds(1_700_000_018);
                let utc = leaps.gpst_to_utc(gpst).unwrap();
                let back = leaps.utc_to_gpst(utc).unwrap();
                expect!(back).to_equal(gpst)?;
            }

            "gpst_nanos -> utc_nanos -> gpst_nanos roundtrips" {
                let leaps = LeapSeconds::known();
                let gpst = GpstNanos(1_700_000_018_500_000_000);
                let utc = leaps.gpst_to_utc_nanos(gpst).unwrap();
                let back = leaps.utc_to_gpst_nanos(utc).unwrap();
                expect!(back).to_equal(gpst)?;
            }

            "seconds -> nanos -> floor roundtrip for UtcUnixSeconds" {
                let sec = UtcUnixSeconds(1_700_000_000);
                let ns: UtcUnixNanos = sec.into();
                let back = ns.to_seconds_floor();
                expect!(back).to_equal(sec)?;
            }

            "seconds -> nanos -> floor roundtrip for GpstSeconds" {
                let sec = GpstSeconds(1_700_000_018);
                let ns: GpstNanos = sec.into();
                let back = ns.to_seconds_floor();
                expect!(back).to_equal(sec)?;
            }

            "offset invariant: tai_utc_offset equals utc_to_tai minus utc" {
                let leaps = LeapSeconds::known();
                let timestamps: Vec<i64> = vec![
                    63_072_000, 78_796_800, 500_000_000, 1_000_000_000,
                    1_483_228_800, 1_700_000_000, 2_000_000_000,
                ];
                for ts in timestamps {
                    let utc = UtcUnixSeconds(ts);
                    let offset = leaps.tai_utc_offset(utc).unwrap();
                    let tai = leaps.utc_to_tai(utc).unwrap();
                    expect!(tai.0 - utc.0).to_equal(i64::from(offset))?;
                }
            }
        }

        // =======================================================================
        // GpstNanos::to_seconds_floor
        // =======================================================================

        "GpstNanos to_seconds_floor" {
            "truncates correctly" {
                let ns = GpstNanos(1_700_000_018_999_999_999);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(GpstSeconds(1_700_000_018))?;
            }

            "exact seconds" {
                let ns = GpstNanos(1_700_000_018_000_000_000);
                let sec = ns.to_seconds_floor();
                expect!(sec).to_equal(GpstSeconds(1_700_000_018))?;
            }
        }

        // =======================================================================
        // Trait impls: Ord, Hash, Clone, Debug
        // =======================================================================

        "trait impls" {
            "UtcUnixSeconds ordering" {
                expect!(UtcUnixSeconds(100) < UtcUnixSeconds(200)).to_be_true()?;
                expect!(UtcUnixSeconds(200) > UtcUnixSeconds(100)).to_be_true()?;
                expect!(UtcUnixSeconds(100) <= UtcUnixSeconds(100)).to_be_true()?;
            }

            "TaiSeconds ordering" {
                expect!(TaiSeconds(100) < TaiSeconds(200)).to_be_true()?;
            }

            "GpstSeconds ordering" {
                expect!(GpstSeconds(100) < GpstSeconds(200)).to_be_true()?;
            }

            "nanos types ordering" {
                expect!(UtcUnixNanos(100) < UtcUnixNanos(200)).to_be_true()?;
                expect!(TaiNanos(100) < TaiNanos(200)).to_be_true()?;
                expect!(GpstNanos(100) < GpstNanos(200)).to_be_true()?;
            }

            "Debug formatting for seconds types" {
                let s = format!("{:?}", UtcUnixSeconds(42));
                expect!(s).to_equal("UtcUnixSeconds(42)".to_string())?;

                let s = format!("{:?}", TaiSeconds(42));
                expect!(s).to_equal("TaiSeconds(42)".to_string())?;

                let s = format!("{:?}", GpstSeconds(42));
                expect!(s).to_equal("GpstSeconds(42)".to_string())?;
            }

            "Debug formatting for nanos types" {
                let s = format!("{:?}", UtcUnixNanos(42));
                expect!(s).to_equal("UtcUnixNanos(42)".to_string())?;

                let s = format!("{:?}", TaiNanos(42));
                expect!(s).to_equal("TaiNanos(42)".to_string())?;

                let s = format!("{:?}", GpstNanos(42));
                expect!(s).to_equal("GpstNanos(42)".to_string())?;
            }

            "Error equality and clone" {
                let err1 = Error::OutOfRange { requested: 0, valid_start: 10, valid_end: 100 };
                let err2 = err1.clone();
                expect!(err1.clone()).to_equal(err2)?;

                let err3 = Error::InvalidTable { detail: "test" };
                expect!(err1 != err3).to_be_true()?;
            }
        }

        // =======================================================================
        // is_during_leap_second edge case
        // =======================================================================

        "is_during_leap_second with pre-table timestamp" {
            "returns false for timestamp before 1972" {
                let leaps = LeapSeconds::known();
                expect!(leaps.is_during_leap_second(UtcUnixSeconds(0))).to_be_false()?;
            }
        }

        // =======================================================================
        // Builder edge cases
        // =======================================================================

        "builder edge cases" {
            "expires_at called before add" {
                let table = LeapSeconds::builder()
                    .expires_at(UtcUnixSeconds(9999))
                    .add(UtcUnixSeconds(100), 10)
                    .build()
                    .unwrap();

                expect!(table.expires_at()).to_be_some_with(UtcUnixSeconds(9999))?;
                let tai = table.utc_to_tai(UtcUnixSeconds(200)).unwrap();
                expect!(tai).to_equal(TaiSeconds(210))?;
            }

            "builder with zero offset" {
                let table = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 0)
                    .build()
                    .unwrap();

                let tai = table.utc_to_tai(UtcUnixSeconds(200)).unwrap();
                expect!(tai).to_equal(TaiSeconds(200))?;
            }

            "Default impl matches new" {
                let a = LeapSecondsBuilder::default();
                let b = LeapSecondsBuilder::new();
                // Both should be empty builders that fail to build
                let result_a = a.build();
                let result_b = b.build();
                expect!(result_a).to_be_err()?;
                expect!(result_b).to_be_err()?;
            }
        }

        // =======================================================================
        // Integration: chaining multiple operations
        // =======================================================================

        "multi-operation integration" {
            "utc -> tai -> gpst -> nanos -> back to seconds" {
                let leaps = LeapSeconds::known();
                let utc = UtcUnixSeconds(1_700_000_000);

                // UTC -> TAI -> GPST
                let tai = leaps.utc_to_tai(utc).unwrap();
                let gpst = tai_to_gpst(tai);

                // GPST seconds -> nanos -> floor back to seconds
                let gpst_ns: GpstNanos = gpst.into();
                let back_sec = gpst_ns.to_seconds_floor();
                expect!(back_sec).to_equal(gpst)?;

                // GPST -> UTC via table
                let back_utc = leaps.gpst_to_utc(gpst).unwrap();
                expect!(back_utc).to_equal(utc)?;
            }

            "two tables produce different results for same input" {
                let known = LeapSeconds::known();
                let custom = LeapSeconds::builder()
                    .add(UtcUnixSeconds(100), 5)
                    .build()
                    .unwrap();

                let utc = UtcUnixSeconds(1_700_000_000);
                let tai_known = known.utc_to_tai(utc).unwrap();
                let tai_custom = custom.utc_to_tai(utc).unwrap();

                // known uses offset 37, custom uses offset 5
                expect!(tai_known).to_equal(TaiSeconds(1_700_000_037))?;
                expect!(tai_custom).to_equal(TaiSeconds(1_700_000_005))?;
            }

            "mixed seconds and nanos paths converge" {
                let leaps = LeapSeconds::known();
                let utc_sec = UtcUnixSeconds(1_700_000_000);

                // Path 1: seconds conversion
                let tai_sec = leaps.utc_to_tai(utc_sec).unwrap();

                // Path 2: promote to nanos, convert, floor back
                let utc_ns: UtcUnixNanos = utc_sec.into();
                let tai_ns = leaps.utc_to_tai_nanos(utc_ns).unwrap();
                let tai_sec_from_ns = tai_ns.to_seconds_floor();

                expect!(tai_sec_from_ns).to_equal(tai_sec)?;
            }
        }

        // =======================================================================
        // Well-known reference tests — verified against IANA leap-seconds.list
        //
        // Source: https://data.iana.org/time-zones/tzdb/leap-seconds.list
        // NTP epoch = 1900-01-01, Unix epoch = 1970-01-01
        // NTP-to-Unix offset = 2_208_988_800
        //
        // Each Unix timestamp below = (NTP timestamp) - 2_208_988_800
        // =======================================================================

        "IANA/IERS cross-reference" {
            setup {
                let leaps = LeapSeconds::known();
            }

            // Verify all 28 entries against the IANA leap-seconds.list NTP timestamps.
            // NTP_OFFSET = 2_208_988_800 (seconds between 1900-01-01 and 1970-01-01).
            "all 28 entries match IANA NTP timestamps" {
                // (NTP timestamp, expected Unix timestamp, expected TAI-UTC offset)
                let iana_reference: Vec<(i64, i64, i32)> = vec![
                    (2_272_060_800, 63_072_000,    10), // 1972-01-01
                    (2_287_785_600, 78_796_800,    11), // 1972-07-01
                    (2_303_683_200, 94_694_400,    12), // 1973-01-01
                    (2_335_219_200, 126_230_400,   13), // 1974-01-01
                    (2_366_755_200, 157_766_400,   14), // 1975-01-01
                    (2_398_291_200, 189_302_400,   15), // 1976-01-01
                    (2_429_913_600, 220_924_800,   16), // 1977-01-01
                    (2_461_449_600, 252_460_800,   17), // 1978-01-01
                    (2_492_985_600, 283_996_800,   18), // 1979-01-01
                    (2_524_521_600, 315_532_800,   19), // 1980-01-01
                    (2_571_782_400, 362_793_600,   20), // 1981-07-01
                    (2_603_318_400, 394_329_600,   21), // 1982-07-01
                    (2_634_854_400, 425_865_600,   22), // 1983-07-01
                    (2_698_012_800, 489_024_000,   23), // 1985-07-01
                    (2_776_982_400, 567_993_600,   24), // 1988-01-01
                    (2_840_140_800, 631_152_000,   25), // 1990-01-01
                    (2_871_676_800, 662_688_000,   26), // 1991-01-01
                    (2_918_937_600, 709_948_800,   27), // 1992-07-01
                    (2_950_473_600, 741_484_800,   28), // 1993-07-01
                    (2_982_009_600, 773_020_800,   29), // 1994-07-01
                    (3_029_443_200, 820_454_400,   30), // 1996-01-01
                    (3_076_704_000, 867_715_200,   31), // 1997-07-01
                    (3_124_137_600, 915_148_800,   32), // 1999-01-01
                    (3_345_062_400, 1_136_073_600, 33), // 2006-01-01
                    (3_439_756_800, 1_230_768_000, 34), // 2009-01-01
                    (3_550_089_600, 1_341_100_800, 35), // 2012-07-01
                    (3_644_697_600, 1_435_708_800, 36), // 2015-07-01
                    (3_692_217_600, 1_483_228_800, 37), // 2017-01-01
                ];

                let ntp_offset: i64 = 2_208_988_800;

                for (ntp, expected_unix, expected_offset) in &iana_reference {
                    // Verify NTP-to-Unix conversion
                    let computed_unix = ntp - ntp_offset;
                    expect!(computed_unix).to_equal(*expected_unix)?;

                    // Verify our table returns the correct offset at this timestamp
                    let utc = UtcUnixSeconds(*expected_unix);
                    let offset = leaps.tai_utc_offset(utc).unwrap();
                    expect!(offset).to_equal(*expected_offset)?;
                }
            }
        }

        // =======================================================================
        // Well-known historical dates — independently verifiable
        // =======================================================================

        "well-known dates" {
            setup {
                let leaps = LeapSeconds::known();
            }

            // GPS epoch: 1980-01-06 00:00:00 UTC (Unix: 315964800)
            // At GPS epoch, TAI-UTC = 19, so TAI-GPS = 19 by definition.
            // GPS time was zero at this point, meaning GPST = UTC at GPS epoch.
            "GPS epoch 1980-01-06 has TAI-UTC offset 19" {
                let gps_epoch_utc = UtcUnixSeconds(315_964_800); // 1980-01-06 00:00:00 UTC
                let offset = leaps.tai_utc_offset(gps_epoch_utc).unwrap();
                expect!(offset).to_equal(19)?;

                // TAI at GPS epoch = 315964800 + 19 = 315964819
                let tai = leaps.utc_to_tai(gps_epoch_utc).unwrap();
                expect!(tai).to_equal(TaiSeconds(315_964_819))?;

                // GPST at GPS epoch = TAI - 19 = UTC (by definition of GPST epoch)
                let gpst = leaps.utc_to_gpst(gps_epoch_utc).unwrap();
                expect!(gpst).to_equal(GpstSeconds(315_964_800))?;
            }

            // Y2K: 2000-01-01 00:00:00 UTC (Unix: 946684800)
            // Between 1999-01-01 (offset 32) and 2006-01-01 (offset 33)
            "Y2K 2000-01-01 has TAI-UTC offset 32" {
                let y2k = UtcUnixSeconds(946_684_800);
                let offset = leaps.tai_utc_offset(y2k).unwrap();
                expect!(offset).to_equal(32)?;
                let tai = leaps.utc_to_tai(y2k).unwrap();
                expect!(tai).to_equal(TaiSeconds(946_684_832))?;
            }

            // Unix epoch rollover risk date: 2038-01-19 03:14:07 UTC (Unix: 2147483647 = i32::MAX)
            // After 2017-01-01, offset = 37
            "Unix i32 max (2038-01-19) has TAI-UTC offset 37" {
                let i32_max = UtcUnixSeconds(2_147_483_647);
                let offset = leaps.tai_utc_offset(i32_max).unwrap();
                expect!(offset).to_equal(37)?;
                let tai = leaps.utc_to_tai(i32_max).unwrap();
                expect!(tai).to_equal(TaiSeconds(2_147_483_684))?;
            }

            // Apollo 11 landing: 1969-07-20 — before our table (pre-1972)
            "Apollo 11 1969-07-20 is out of range (pre-1972)" {
                let apollo11 = UtcUnixSeconds(-14_182_940); // 1969-07-20 20:17:40 UTC
                let result = leaps.utc_to_tai(apollo11);
                expect!(result.is_err()).to_be_true()?;
            }

            // The last leap second insertion: 2016-12-31 23:59:60 UTC
            // In POSIX, this maps to 2017-01-01 00:00:00 (Unix 1483228800)
            "last leap second 2016-12-31 23:59:60 is detected" {
                let last_leap = UtcUnixSeconds(1_483_228_800);
                expect!(leaps.is_during_leap_second(last_leap)).to_be_true()?;

                // One second before: 2016-12-31 23:59:59 — still offset 36
                let before = UtcUnixSeconds(1_483_228_799);
                let offset_before = leaps.tai_utc_offset(before).unwrap();
                expect!(offset_before).to_equal(36)?;

                // At the boundary: offset is now 37
                let offset_at = leaps.tai_utc_offset(last_leap).unwrap();
                expect!(offset_at).to_equal(37)?;
            }

            // 2023-11-14 22:13:20 UTC (Unix: 1700000000) — README example value
            "README example: 1700000000 UTC converts to TAI + 37" {
                let utc = UtcUnixSeconds(1_700_000_000);
                let tai = leaps.utc_to_tai(utc).unwrap();
                expect!(tai).to_equal(TaiSeconds(1_700_000_037))?;

                let gpst = leaps.utc_to_gpst(utc).unwrap();
                expect!(gpst).to_equal(GpstSeconds(1_700_000_018))?;

                // Full roundtrip
                let back = leaps.tai_to_utc(tai).unwrap();
                expect!(back).to_equal(utc)?;
            }

            // 1972-01-01 00:00:00 UTC — first entry, offset 10 (start of modern UTC)
            "1972-01-01 epoch: offset 10, not a leap second insertion" {
                let epoch = UtcUnixSeconds(63_072_000);
                let offset = leaps.tai_utc_offset(epoch).unwrap();
                expect!(offset).to_equal(10)?;

                // This is NOT a leap second — it's the initial offset
                expect!(leaps.is_during_leap_second(epoch)).to_be_false()?;
            }
        }

        // =======================================================================
        // Full roundtrip across every leap-second era
        // =======================================================================

        "roundtrip across all eras" {
            "utc -> tai -> utc roundtrips for one timestamp per era" {
                let leaps = LeapSeconds::known();

                // One timestamp per era (between each pair of leap-second entries)
                let era_timestamps: Vec<i64> = vec![
                    63_072_000,      // 1972-01-01 (era start, offset 10)
                    70_000_000,      // mid-1972 (offset 10)
                    90_000_000,      // late 1972 (offset 11)
                    110_000_000,     // mid-1973 (offset 12)
                    150_000_000,     // mid-1974 (offset 13)
                    170_000_000,     // mid-1975 (offset 14)
                    200_000_000,     // mid-1976 (offset 15)
                    240_000_000,     // mid-1977 (offset 16)
                    270_000_000,     // mid-1978 (offset 17)
                    300_000_000,     // mid-1979 (offset 18)
                    340_000_000,     // mid-1980 (offset 19)
                    380_000_000,     // mid-1982 (offset 20)
                    410_000_000,     // late 1982 (offset 21)
                    450_000_000,     // mid-1984 (offset 22)
                    500_000_000,     // mid-1985 (offset 23)
                    600_000_000,     // mid-1989 (offset 24)
                    650_000_000,     // mid-1990 (offset 25)
                    680_000_000,     // mid-1991 (offset 26)
                    720_000_000,     // late 1992 (offset 27)
                    760_000_000,     // mid-1994 (offset 28)
                    800_000_000,     // mid-1995 (offset 29)
                    850_000_000,     // mid-1996 (offset 30)
                    900_000_000,     // mid-1998 (offset 31)
                    1_000_000_000,   // 2001-09-09 (offset 32)
                    1_200_000_000,   // 2008-01-10 (offset 33)
                    1_300_000_000,   // 2011-03-13 (offset 34)
                    1_400_000_000,   // 2014-05-13 (offset 35)
                    1_450_000_000,   // 2015-12-13 (offset 36)
                    1_500_000_000,   // 2017-07-14 (offset 37)
                    2_000_000_000,   // 2033-05-18 (offset 37, far future)
                ];

                for ts in era_timestamps {
                    let utc = UtcUnixSeconds(ts);
                    let tai = leaps.utc_to_tai(utc).unwrap();
                    let back = leaps.tai_to_utc(tai).unwrap();
                    expect!(back).to_equal(utc)?;
                }
            }
        }

        // =======================================================================
        // GPST cross-checks with known constant
        // =======================================================================

        "GPST constant offset cross-check" {
            "TAI - GPST is always exactly 19 seconds" {
                // Verify the constant relationship across a range of values
                let test_values: Vec<i64> = vec![
                    0, 1, 100, 315_964_819, 1_000_000_000, 1_700_000_037, i64::MAX / 2,
                ];
                for tai_val in test_values {
                    let tai = TaiSeconds(tai_val);
                    let gpst = tai_to_gpst(tai);
                    expect!(tai.0 - gpst.0).to_equal(19)?;

                    let back = gpst_to_tai(gpst);
                    expect!(back).to_equal(tai)?;
                }
            }

            "nanos: TAI - GPST is always exactly 19_000_000_000 nanoseconds" {
                let test_values: Vec<i128> = vec![
                    0, 1, 500_000_000, 19_000_000_000, 1_700_000_037_000_000_000,
                ];
                for tai_val in test_values {
                    let tai = TaiNanos(tai_val);
                    let gpst = tai_to_gpst_nanos(tai);
                    expect!(tai.0 - gpst.0).to_equal(19_000_000_000)?;

                    let back = gpst_to_tai_nanos(gpst);
                    expect!(back).to_equal(tai)?;
                }
            }
        }
    }
}

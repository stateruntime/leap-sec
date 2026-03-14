# API Reference

Quick index of all public types, methods, and functions in `leap-sec`.

## Types

| Type | Description |
|------|-------------|
| [`LeapSeconds`](LeapSeconds.md) | An immutable leap-second schedule |
| [`UtcUnixSeconds`](UtcUnixSeconds.md) | Unix seconds in the UTC scale |
| [`TaiSeconds`](TaiSeconds.md) | Continuous seconds in the TAI scale |
| [`GpstSeconds`](GpstSeconds.md) | Continuous seconds in the GPS time scale |
| [`UtcUnixNanos`](UtcUnixNanos.md) | Unix nanoseconds in the UTC scale |
| [`TaiNanos`](TaiNanos.md) | Continuous nanoseconds in the TAI scale |
| [`GpstNanos`](GpstNanos.md) | Continuous nanoseconds in the GPS time scale |
| [`Error`](Error.md) | Conversion and validation errors |
| [`LeapSecondsBuilder`](LeapSecondsBuilder.md) | Builder for custom tables (requires `std`) |

## Conversion Methods on `LeapSeconds`

| Method | Description |
|--------|-------------|
| [`utc_to_tai`](utc_to_tai.md) | Convert UTC seconds to TAI seconds |
| [`tai_to_utc`](tai_to_utc.md) | Convert TAI seconds to UTC seconds |
| [`utc_to_gpst`](utc_to_gpst.md) | Convert UTC seconds to GPST seconds |
| [`gpst_to_utc`](gpst_to_utc.md) | Convert GPST seconds to UTC seconds |
| [`utc_to_tai_nanos`](utc_to_tai_nanos.md) | Convert UTC nanos to TAI nanos |
| [`tai_to_utc_nanos`](tai_to_utc_nanos.md) | Convert TAI nanos to UTC nanos |
| [`utc_to_gpst_nanos`](utc_to_gpst_nanos.md) | Convert UTC nanos to GPST nanos |
| [`gpst_to_utc_nanos`](gpst_to_utc_nanos.md) | Convert GPST nanos to UTC nanos |

## Offset & Detection Methods

| Method | Description |
|--------|-------------|
| [`tai_utc_offset`](tai_utc_offset.md) | Get TAI−UTC offset at a UTC instant |
| [`tai_utc_offset_at_tai`](tai_utc_offset_at_tai.md) | Get TAI−UTC offset at a TAI instant |
| [`is_during_leap_second`](is_during_leap_second.md) | Detect the ambiguous 61st second |

## Table Inspection

| Method | Description |
|--------|-------------|
| [`known`](known.md) | Get the built-in historical table |
| [`valid_range`](valid_range.md) | First and last entry timestamps |
| [`is_expired`](is_expired.md) | Whether the table has expired |
| [`expires_at`](expires_at.md) | Expiration timestamp, if any |
| [`latest_entry`](latest_entry.md) | Most recent leap-second entry |

## Free Functions

| Function | Description |
|----------|-------------|
| [`tai_to_gpst`](tai_to_gpst.md) | TAI → GPST (constant 19s offset) |
| [`gpst_to_tai`](gpst_to_tai.md) | GPST → TAI (constant 19s offset) |
| [`tai_to_gpst_nanos`](tai_to_gpst_nanos.md) | TAI → GPST in nanoseconds |
| [`gpst_to_tai_nanos`](gpst_to_tai_nanos.md) | GPST → TAI in nanoseconds |

# Changelog

All notable changes to `leap-sec` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] — 2026-03-14

### Fixed

- Switch `behave` dev-dependency from git source to crates.io registry.

## [0.1.1] — 2026-03-14

### Fixed

- Fix `cargo fmt` formatting in `table.rs` and `types.rs`.

## [0.1.0] — 2025-06-01

### Added

- `LeapSeconds` struct with embedded table of all 28 historical leap-second entries (1972-01-01 through 2017-01-01).
- `LeapSeconds::known()` — returns the built-in static table. Works in `no_std`, no allocation.
- Six newtype wrappers: `UtcUnixSeconds`, `TaiSeconds`, `GpstSeconds`, `UtcUnixNanos`, `TaiNanos`, `GpstNanos`.
- Step-based conversions: `utc_to_tai`, `tai_to_utc`, `utc_to_gpst`, `gpst_to_utc` (seconds and nanosecond variants).
- `tai_to_gpst()` / `gpst_to_tai()` free functions for the constant 19-second TAI↔GPST offset.
- `tai_utc_offset()` and `tai_utc_offset_at_tai()` offset queries.
- `is_during_leap_second()` to detect the ambiguous 61st second (`23:59:60`).
- Table inspection: `valid_range()`, `is_expired()`, `expires_at()`, `latest_entry()`.
- `LeapSecondsBuilder` for constructing custom tables (feature-gated on `std`).
- `Display` trait for all newtypes with explicit scale labels (e.g., `"1700000000 UTC"`).
- `From<XxxSeconds> for XxxNanos` and `to_seconds_floor()` for lossless precision conversions.
- `Error` enum with `OutOfRange`, `TableExpired`, and `InvalidTable` variants.
- Comprehensive behave test suite (57 specs) covering all boundary conditions, roundtrips, and edge cases.
- CI pipeline (format, clippy, test, docs, MSRV) and tag-triggered release workflow.
- `no_std` support (default feature `std` can be disabled).

[Unreleased]: https://github.com/stateruntime/leap-sec/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/stateruntime/leap-sec/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/stateruntime/leap-sec/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/stateruntime/leap-sec/releases/tag/v0.1.0

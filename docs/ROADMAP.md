# Roadmap

Release plan from v0.1.0 to v1.0.

Guiding principles:

1. **Table-driven.** Leap-second behavior is always driven by explicit data, not hidden assumptions.
2. **Deterministic.** Same table + same input = same output.
3. **Explicit errors.** Never silently guess about unknown future leap seconds.

---

## v0.1.0 — Table + Step Conversions (Foundation)

**Theme:** The core conversion kernel that everything else builds on.

**[MUST]**
- `LeapSeconds` struct with embedded "known table" (all historical leap seconds through 2017-01-01; 27 entries)
- `UtcUnixSeconds`, `TaiSeconds`, `GpstSeconds` newtypes
- Sub-second precision variants: `UtcUnixNanos(i128)`, `TaiNanos(i128)`, `GpstNanos(i128)` with lossless conversion to/from second-precision types
- Deterministic step-based conversions: UTC ↔ TAI, UTC ↔ GPST (both second and nanosecond variants)
- `is_during_leap_second(utc: UtcUnixSeconds) -> bool` to detect the ambiguous 61st second (23:59:60)
- `tai_to_gpst()` / `gpst_to_tai()` as free functions (constant 19-second offset, no table needed); nanosecond variants included
- `tai_utc_offset()` at any UTC or TAI instant
- `valid_range()` and `is_expired()` on the table
- Explicit `Error::TableExpired` and `Error::OutOfRange` for out-of-range inputs
- Behave test specs covering:
  - All 27 leap-second boundaries (before, during, after each insertion)
  - Pre-1972 range (before first leap second)
  - Post-2017 range (latest known state)
  - Roundtrip: `utc_to_tai(x) |> tai_to_utc == x` for all valid inputs
  - `is_during_leap_second` at each of the 27 insertion points
  - `tai_to_gpst` / `gpst_to_tai` roundtrip
- Custom table builder for testing with synthetic leap-second schedules

**[SHOULD]**
- `expires_at() -> Option<UtcUnixSeconds>` returning the IERS file expiration (or `None` for the built-in table)
- `latest_entry() -> (UtcUnixSeconds, i32)` returning the most recent leap-second date and offset
- `Display` trait for all newtypes showing the scale label (e.g. `"1704067200 UTC"`, `"1704067237 TAI"`)
- `no_std` verified on a real embedded target
- Documentation of the leap-second history and the 2030s UTC change (CGPM Resolution 4, 2022)
- Documentation of negative leap-second handling (offset decreasing by 1; none historical, but ITU-R allows them)

## v0.2.0 — Table Parsing + Relaxed Mode

**Theme:** Load leap-second tables from standard IERS/NTP files. Add opt-in extrapolation for simulation use cases.

**[MUST]**
- Feature-gated `iers_parse` module (requires `std` or `alloc`)
- Parse NTP/IERS `leap-seconds.list` format
- Table validation: check monotonicity, date ordering, expiration
- `from_iers_list(data: &str) -> Result<LeapSeconds, Error>`

**[SHOULD]**
- `LeapSeconds::with_policy(ExpirationPolicy::Extrapolate)` — suppresses `TableExpired` and assumes the last known offset continues; documented as simulation-only
- `ExpirationPolicy` enum (`Strict` default, `Extrapolate`)
- Support for IERS Bulletin C format (announcement bulletins)
- Documentation of where to obtain leap-second files and their update frequency

## v0.3.0 — Smear Policies + Bulk Conversions

**Theme:** Explicit, opt-in smear for systems that don't want steps. Batch processing for large datasets.

**[MUST]**
- `SmearPolicy` enum with `Linear24h` and `LinearCustom` variants
- `utc_to_smeared()` and `smeared_to_utc()` conversions
- Smear ↔ step comparison tests (verify they agree outside the smear window)
- Documentation making it clear that smeared output is NOT UTC and NOT TAI

**[SHOULD]**
- `utc_to_tai_batch(&[UtcUnixSeconds]) -> Result<Vec<TaiSeconds>, Error>` and `tai_to_utc_batch` — feature-gated on `std`/`alloc`
- Quadratic smear option (smoother transition at window edges)
- Visual documentation showing step vs smear behavior at a leap-second boundary

## v1.0 — Stability

Lock down:

- Public types and error taxonomy (semver-stable)
- Embedded "known table" update policy (how and when new leap seconds are added to the crate)
- Conversion semantics documented with reference values
- Post‑transition expectations: table stops growing, existing conversions remain correct
- Compatibility commitments: `no_std` profile, MSRV policy
- Integration contract with `space-clock` (how `space-clock` consumes `leap-sec`)

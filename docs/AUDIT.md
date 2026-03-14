# Audit

A competitive and product audit for `leap-sec`.

This crate exists because leap seconds are one of those problems that are easy to “mostly ignore” until
they quietly corrupt timelines, ordering, and correlation logic.

---

## What We’re Building

`leap-sec` is a **leap-second conversion kernel**:

- represent a leap-second schedule as explicit data
- convert between UTC and continuous time scales (TAI/GPS-like) deterministically
- fail explicitly when a conversion is outside the table’s coverage

This crate is a building block for `space-clock`. It is not a civil-time/date library.

## The Real Problem We Solve

Leap seconds are not just a “formatting issue”. They create discontinuities that break assumptions in:

- ordering (“time never goes backward”)
- indexing (“timestamp uniquely identifies a second”)
- correlation (“two systems agree on what ‘now’ means”)

`leap-sec` makes these discontinuities explicit and forces callers to supply the leap-second knowledge
they are using.

## What We Do Right (Non-Negotiables)

1. **Table-driven conversions:** leap seconds are input data, not hidden globals.
2. **Explicit unknowns:** conversions outside supported range return structured errors.
3. **No OS clock assumptions:** correctness must not depend on host configuration.
4. **Policy separation:** smear (if offered) is opt-in and explicit.

## Where We Can Lose

- **Ambiguous semantics:** “Unix time”, “UTC”, and “POSIX time” are not interchangeable.
- **Smear confusion:** smeared time is not UTC. It’s a policy-defined mapping.
- **Future uncertainty:** leap seconds are being phased out by international agreement, and tables expire.

If we are sloppy here, downstream mission software inherits the ambiguity.

## Future Pressures

Things that will be true in the future and must not break the design:

- **UTC is changing:** CGPM has adopted a path to relax the `|UT1−UTC|` limit in the 2030s, which is expected
  to eliminate the need for leap seconds for a long time. Historical leap seconds still remain.
- **Tables expire:** leap-second lists have validity windows and must be treated as such.
- **Smearing exists:** some systems will continue to use smear policies for operational reasons.
- **Negative leap seconds are possible:** none have happened historically, but the standards allow them.

Our response is: treat tables as explicit inputs, make out-of-range behavior a first-class error, and keep
smear strategies as explicit opt-in mappings rather than hidden behavior.

## Competitive Audit (What Exists Today)

### hifitime

`hifitime` provides a broad time-scale and epoch system (space/astronomy focus), including leap seconds.

**Takeaway for us:** we should stay small and composable. `leap-sec` is the conversion kernel, and
`space-clock` can decide whether to interop with `hifitime` for higher-level needs.

### tai_time

`tai_time` focuses on a TAI-based representation and conversion between TAI and UTC, with an embedded-friendly
design center.

**Takeaway for us:** keep the types boring and the conversions explicit. Embedded users should not be
forced into a heavy calendar ecosystem.

### leap-seconds list parsers

The `leap_seconds` crate focuses on parsing the NTP/IERS `leap-seconds.list` file and is usable in `no_std`.

**Takeaway for us:** parsing and conversion are different layers. Parsing should be optional; conversion
semantics should be testable independent of how the table was obtained.

## Developer Experience Bar

We should not ship an API that forces users to guess:

- what time scale a value is in
- whether leap seconds are represented, ignored, or smeared
- whether an “unknown future” conversion silently assumes “no more leap seconds”

The pit of success is: a user cannot perform a UTC↔TAI conversion without supplying a leap-second schedule.

## v1.0 Acceptance Criteria

We do not ship v1.0 until:

- conversion semantics and edge cases are documented (including “table expired” behavior)
- tests cover historical leap-second boundaries and known tricky cases
- public types are small, explicit, and stable enough for long-lived missions

## References

- USNO leap seconds status: https://maia.usno.navy.mil/products/leap-seconds
- IERS Bulletins (leap second announcements): https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html
- ITU-R / CGPM 2022 discontinuation decision context: https://www.bipm.org/en/committees/cg/cgpm/meeting-27
- `hifitime` docs: https://docs.rs/hifitime
- `tai_time` docs: https://docs.rs/tai-time
- `leap_seconds` docs: https://docs.rs/leap_seconds

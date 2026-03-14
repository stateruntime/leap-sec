# Landscape

Where `leap-sec` fits in the Rust ecosystem, and why it exists.

## Identity

`leap-sec` is a **leap-second conversion kernel**.

It is narrower than “a time library” and broader than “a leap-second list parser”.
The core promise is making UTC ↔ continuous-time conversions explicit and testable.

## Related Crates (Today)

### Time libraries with leap-second awareness

- `hifitime` (astronomy/space time scales, epochs, and leap seconds)

### Focused time-scale / table crates

- `tai_time` (TAI-oriented time representations)
- `leap_seconds` (parsing NTP/IERS `leap-seconds.list`)
- `timelane` (time conversions with embedded leap seconds)

## Differentiation

`leap-sec` is optimized for:

- **Small, composable core**: “table + conversion” without committing to a full datetime stack
- **Explicit unknowns**: out-of-range conversions fail deterministically
- **Embedded friendliness**: offline tables and `no_std`-first ergonomics
- **Policy separation**: smearing (if used) is an opt-in layer, not a hidden behavior

## Practical “Which One?” Guidance

- If you need a broad space/astronomy time system (many scales, epochs, higher-level features),
  `hifitime` may already be the right choice.
- If you primarily need a TAI-based representation with UTC conversion in an embedded-friendly package,
  `tai_time` is worth evaluating.
- If you need a parser for the NTP/IERS leap seconds list file, `leap_seconds` is focused and `no_std`.
- If you want a small kernel focused specifically on leap-second semantics and explicit error behavior,
  designed to power `space-clock`, `leap-sec` is the gap.

## References

- USNO leap seconds status: https://maia.usno.navy.mil/products/leap-seconds
- IERS Bulletins (leap second announcements): https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html
- `hifitime` docs: https://docs.rs/hifitime
- `tai_time` docs: https://docs.rs/tai-time
- `leap_seconds` docs: https://docs.rs/leap_seconds
- `timelane` docs: https://docs.rs/timelane

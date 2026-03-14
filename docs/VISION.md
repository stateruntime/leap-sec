# Vision

`leap-sec` is a **leap-second conversion kernel** — a small library that does one thing: convert between UTC (civil time, with leap seconds) and continuous time scales (TAI/GPS, without leap seconds).

## The One-Sentence Pitch

A table-driven, deterministic converter between UTC and TAI/GPS time that never silently guesses — it either gives you the right answer or tells you it doesn't know.

## Problem Statement

### What are leap seconds?

The Earth's rotation is irregular and gradually slowing. To keep UTC (civil time) within 0.9 seconds of the Earth's actual rotational position (UT1), the IERS occasionally inserts an extra second into UTC. Since 1972, this has happened 27 times — always at the end of June 30 or December 31.

This means UTC is **discontinuous**: the sequence `23:59:59` → `23:59:60` → `00:00:00` has an "extra" second that doesn't exist in continuous time scales like TAI or GPS time.

### Why is this a problem?

Most software ignores leap seconds entirely. Unix timestamps pretend they don't exist. This creates three categories of bugs:

1. **Silent offset drift**: the UTC↔TAI/GPS offsets change over time. As of **2017-01-01** (the most recent leap second insertion to date), **TAI−UTC = 37s** and **GPS−UTC = 18s** (because **TAI−GPS = 19s** by definition). If code doesn't account for these offsets, timestamps from different systems are silently wrong by seconds — enough to miscorrelate telemetry, misidentify satellite passes, or break time-series indexing.

2. **Ambiguous seconds**: During a leap second insertion, the UTC second `23:59:60` cannot be represented in Unix time. Systems either repeat the previous second (causing duplicates) or skip ahead (causing gaps). Either way, event ordering breaks.

3. **Inconsistent conversions**: Different systems handle leap seconds differently — some step, some smear (spread the extra second over hours), some ignore. When these systems exchange timestamps, they silently disagree about what time it is.

### Why not just use an existing library?

Existing solutions either:
- **Do too much**: `hifitime` is a full space-time system; `chrono`/`jiff` are civil-time libraries. If you just need UTC↔TAI, you're importing a large dependency surface.
- **Do too little**: hardcoding “today’s offset” (e.g., `TAI = UTC + 37`) is wrong for historical data and will be wrong again if another leap second is ever inserted.
- **Hide the table**: some libraries embed leap-second knowledge as implicit global state, making it impossible to test with custom or expired tables.

## Core Thesis

**Treat leap seconds as data, and make the mapping between timescales an explicit, testable function.**

1. A leap-second table is an **input** — embedded or parsed, never hidden global state
2. Conversions are **pure functions**: `(table, utc) → tai` and `(table, tai) → utc`
3. Unknown ranges produce **explicit errors**, never silent guesses

## Target Users

- **Space systems** that operate in TAI/GPS time but must ingest or emit UTC timestamps
- **Ground systems** that correlate data from multiple timekeeping regimes
- **Embedded systems** (`no_std`) that cannot rely on OS timekeeping correctness
- **`space-clock`** — this library is the leap-second foundation that `space-clock` builds on

## Non-Goals

- Time zones, calendar arithmetic, locale formatting
- High-level NTP/PTP clock synchronization
- Automatically downloading leap-second tables (networking is opt-in)
- Being a datetime library

## What Must Stay True

1. **Table-driven**: conversions use a leap-second schedule, not hidden global state
2. **Deterministic**: same table + same input = same output, always
3. **Explicit unknowns**: conversions outside table coverage fail loudly with `Error::TableExpired`
4. **Small surface area**: this crate stays a focused building block

## Future‑Proofing

UTC is changing. In 2022, the 27th CGPM adopted Resolution 4, setting a path to increase the maximum
allowed value of `|UT1−UTC|` (currently enforced via leap seconds) at a date to be determined, with key
decisions targeted by or before 2035.

The future is not “leap seconds vanish overnight” — it’s:

- Missions spanning the transition (historical data has leaps, new data doesn't)
- Long-lived archives requiring correct historical UTC handling for decades
- Conversion code that must be deterministic even when tables expire

`leap-sec` is designed so the leap-second policy is always **data-driven and testable**. When leap seconds stop, the table simply stops growing. No code changes needed.

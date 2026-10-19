# Calculate on open

- **Status:** Accepted

## Context

Prayer times could be saved in a configuration file or recalculated whenever
the application opens. The choice affects accuracy, startup time, daylight
saving transitions, and implementation complexity.

## Options considered

### Calculate on open

- Always uses the current date, location, and timezone.
- Avoids stale prayer-time data and an additional file format.
- Adds a small amount of work to application startup.

### Save a pre-calculated schedule

- Could reduce startup calculation time.
- Could become stale when the date, timezone, or daylight-saving rules change.
- Adds file I/O and schedule invalidation logic.

## Decision

Calculate prayer times when the application opens. The performance cost is
negligible on modern devices, and recalculation keeps the result current.

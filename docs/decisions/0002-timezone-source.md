# ADR 0002: Timezone source

- Status: Accepted
- Date: 2026-10-06

## Context

Prayer times depend on local civil time, daylight-saving rules, coordinates,
and a reliable timezone identifier.

## Decision

Prefer an IANA timezone lookup based on the selected city's coordinates.
Retain a stored UTC offset as a fallback when lookup fails.

## Consequences

- Daylight-saving behavior can be correct when the IANA lookup succeeds.
- A static offset fallback may become stale in regions with daylight saving.
- The application should make fallback behavior diagnosable.

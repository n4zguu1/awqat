# ADR 0001: Initial Hijri date strategy

- Status: Accepted
- Date: 2026-10-06

## Context

Hijri dates vary by local moon sighting, country policy, and calculation
method. Awqat needs a deterministic offline default while allowing correction.

## Decision

Use Umm al-Qura as the initial offline default. Store a user-applied day
correction in configuration. Do not claim that the result represents local
moon-sighting decisions.

## Consequences

- The application works offline.
- Users can correct dates locally.
- The displayed date may differ from local religious authorities.
- Network synchronization can be considered later without changing the UI
  contract.

# Bundle and update the database

- **Status:** Accepted

## Context

The application is local and single-user. The database is valuable application
data, but it is read-only at runtime and must be available offline.

## Decision

Bundle the database with the application binary. Update it as part of the
application release process.

## Consequences

- Deployment remains simple.
- Users receive database updates with application updates.
- The bundled data can be extracted from the binary, which is acceptable
  because it is not treated as secret data.

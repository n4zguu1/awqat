# Use a database instead of JSON

- **Status:** Accepted

## Context

The dataset needs efficient lookup and flexible search across many cities.

## Decision

Use a database rather than JSON. A database provides indexed lookups and
search capabilities without requiring the application to load the entire
dataset into memory.

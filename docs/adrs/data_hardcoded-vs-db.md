# Store data in a database

- **Status:** Accepted

## Context

The application needs a large city dataset and regular data updates.
Hardcoding every record would make updates difficult and increase maintenance
cost.

## Decision

Store the dataset in a database. Database bundling and update behavior are
defined in [Bundle and update the database](./bundle-and-update-db.md).

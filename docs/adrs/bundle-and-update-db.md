# bundle-and-update

## Status: Accepted

## Context

how do we bundle and update the database for our application? We need a strategy to ensure that the database is properly
packaged with the application and can be updated seamlessly when new versions are released.
we need to consider the following aspects:

- security: is the data valuable, embedded db into binary, makes it ez to reverse.
- concurrency: is the db have multiple users ?

## Proposed Solution

since tha app is local and runs for single user, and its read-only, we can bundle the db into the binary.
for updates, it follows the app updates. which means that when a new version of the app is released, the database will
be updated as well. This approach simplifies the deployment process and ensures that users always have the latest
version of the database.

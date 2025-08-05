# ADR-0004: Use SQLite as the Primary Database

- Date: 2025-08-05
- Status: Accepted

## Context

We need to select a primary database for the application. A key requirement for the application is to be lightweight and easy to install, with as few external dependencies as possible. The target audience primarily consists of users who will run the application in a single-instance, non-distributed environment. The database must be able to handle moderate workloads efficiently without requiring a dedicated database administrator or complex setup procedures.

We considered several options, including client-server databases like PostgreSQL and MySQL, and embedded databases like SQLite.

## Decision

We will use SQLite as the primary database for the application.

SQLite is an in-process library that implements a self-contained, serverless, zero-configuration, transactional SQL database engine. The database will be stored as a single file alongside the application, which fulfills our primary goal of simplicity and ease of deployment.

## Consequences

### Positive

* Simplicity and Ease of Deployment: There is no separate database server to install, configure, or manage. The application remains a self-contained unit.
* Portability: The entire database is a single file on disk, which is easy to back up, copy, and move between environments.
* Reduced Dependencies: Eliminates the need for users to install and maintain a separate database system, significantly lowering the barrier to entry.
* Sufficient Performance: For the intended use case of a single-instance application, SQLite's performance is more than adequate.

### Negative

* Limited Concurrency: SQLite is not well-suited for applications with high write concurrency, as it locks the entire database file during writes. This is considered an acceptable trade-off, as the application's primary workload is read-heavy, and write operations are infrequent.
* No Network Access: Being an embedded library, SQLite does not natively support remote network access, which restricts the architecture to a single machine.
* Scalability Constraints: This choice inherently limits the application's ability to scale out. It is not suitable for a distributed or clustered environment. If the application's requirements grow to need such a setup, a migration to a client-server database would be a significant undertaking.
* Fewer Advanced Features: Lacks some of the advanced capabilities and specialized data types found in systems like PostgreSQL.

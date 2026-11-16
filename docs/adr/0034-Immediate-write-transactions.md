# ADR 0034: Immediate write transactions

- Date: 2026-11-15
- Status: accepted

## Context

The backend, the queue consumer, the cron jobs, and the chatbot daemons share one SQLite file in WAL mode. Every mutating API request ran in one deferred transaction opened by the request middleware, held until the response is ready.

A deferred transaction takes no lock at `BEGIN`. The handler reads first (permission checks, the row it is about to modify), which establishes a WAL read snapshot. The first write then has to promote that snapshot to a write transaction. If any other process committed since the read began, SQLite fails the promotion with `SQLITE_BUSY_SNAPSHOT`, which surfaces as `database is locked`. Waiting cannot fix it: the snapshot is stale and only a new transaction can proceed. This produced intermittent 500s on fast, isolated POST requests while no writer was actually blocked.

## Decision

Every write transaction opened for an API request begins `IMMEDIATE`; the write lock is taken before the first read.

- The request middleware starts the session with `AppState::session_immediate()`. The lock is held from before the handler runs, so contention becomes a normal lock wait covered by the existing 5 s `busy_timeout`, and `BUSY_SNAPSHOT` cannot occur.
- The retry policy applies only to creating the transaction, wrapping `connect` and `BEGIN IMMEDIATE` in the existing busy retry. Each attempt opens a fresh connection, so a failed begin leaves nothing open, and no statement has executed, so a retry cannot duplicate work.
- CLI commands keep the deferred `session()`. Manual jobs may span minutes of work, and they should not take the write lock earlier than their first write.
- A remote (Turso) database falls back to a deferred transaction, because libsql's remote transport does not guarantee immediate behaviour.
- Database errors keep the SQLite extended result code so `BUSY` (5), `LOCKED` (6) and `BUSY_SNAPSHOT` (517) are distinguishable in error reports.

## Consequences

- Handlers hold the single writer lock for their whole run, including any external I/O before the first write. The known cases are the operator-only panorama multipart handlers, which upload to S3 before writing, and the hourly `osm-pull` job, which now fetches from Overpass before opening its session.
- Write concurrency is serialized at request start rather than at the first write. Correctness no longer depends on whether a handler read before writing.
- `is_transient_lock()` remains valid only for autocommit statements. Retrying a statement inside an open transaction cannot recover a stale snapshot, so the transaction has to restart instead.
- New code in the backend and the chatbot should follow the same rule: a write transaction begins immediately, unless it is a long manual job that starts with reads.
- The lock window is observable in request durations; the request metrics show where it actually costs time.

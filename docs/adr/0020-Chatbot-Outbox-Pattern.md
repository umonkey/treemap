# ADR 0020: Chatbot Outbox Pattern

- Date: 2026-08-24
- Status: accepted

## Context

The Telegram chatbot service previously dispatched alert notifications synchronously within the alert dispatcher daemon, risking delivery failures, network timeouts, or partial failures across multiple recipients. We need a reliable outbox pattern for chatbot notifications that decouples alert reporting from message delivery and ensures automatic retries with exponential backoff.

## Decision

We have introduced a database-backed outbox pattern for the chatbot service with the following architecture:

- Database outbox queue: all outgoing chatbot messages are stored in the `chatbot_outbox` table with pending status before being sent.
- Producer and consumer decoupling: the `AlertDispatcher` daemon enqueues alert messages for all configured recipients, while a dedicated `OutboxDispatcher` daemon polls pending outbox messages and dispatches them via Telegram.
- Retry mechanism: failed message deliveries increment attempts and schedule future retries using exponential backoff (`10 * 2^attempts` seconds).
- Maximum attempts limit: messages exceeding 5 delivery attempts are marked as permanently failed.

This decision is based on:

- Reliability: guaranteed delivery of alert notifications even during transient Telegram API outages.
- Fault isolation: separating alert detection from message dispatching prevents bottlenecks and cascading failures.
- Consistency: transactional state transitions ensure messages are tracked reliably from pending to sent or failed.

## Consequences

- Operational requirement: production deployments must run the `dispatch-outbox` supervisor process alongside `dispatch-alerts` and `chatbot`.
- Database schema expansion: addition of the `chatbot_outbox` table and corresponding indexes in shared SQLite storage.
- Increased traceability: comprehensive tracking of delivery attempts, errors, and retry timestamps.

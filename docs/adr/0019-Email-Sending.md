# ADR 0019: Email Sending Architecture and Dispatch Split

- Date: 2026-08-01
- Status: accepted

## Context

The application needs to send transactional emails (such as notifications for panorama processing and transcoding results). We require a reliable, portable, and decoupled email delivery mechanism that does not block web requests and can operate independently of external mail server availability or credentials during regular API operations.

## Decision

We have established an asynchronous email sending architecture with the following design choices:

- SMTP protocol: we use SMTP via the `lettre` library for standard email delivery, ensuring high portability across providers.
- Transactional Outbox pattern: all outgoing emails are enqueued into an internal database table (`EmailRepository`) as pending records before dispatching.
- Producer and consumer decoupling: the API backend (Producer) only enqueues emails and does not require SMTP configuration or secrets to operate. A separate CLI worker (Consumer / EmailDispatcher) polls and dispatches pending emails.
- Handlebars templating: we use Handlebars templates for rendering both HTML and text email bodies.
- Email configuration: all SMTP and email settings are stored in secrets exclusively to keep configuration centralized and secure.

This decision is based on:

- Fault tolerance: API request handlers remain resilient and succeed even if SMTP servers are temporarily unreachable or unconfigured.
- Operational security: production secrets such as SMTP credentials can be isolated to the dedicated background worker process.
- Reliability: database-backed queuing ensures no email notifications are lost due to transient network failures.

## Consequences

- Operational split: deployments must include both the web server process and the email dispatcher CLI worker process when email notifications are required.
- Configuration flexibility: application startup (`AppState`) succeeds even when SMTP configuration is missing, allowing local development and testing without mail server setup.
- Database overhead: transactional email queueing introduces minor database write overhead, which is negligible for scale.

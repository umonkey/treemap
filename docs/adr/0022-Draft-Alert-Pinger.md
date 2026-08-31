# ADR 0022: Draft Alert Pinger

- Date: 2026-08-31
- Status: accepted

## Context

Draft citizen feedback reports can remain incomplete if users do not provide all required fields (photo, location, description). Previously, draft alerts remained without prompts or reminders, relying solely on user initiative. We need an automated pinger daemon that prompts users with reminders when necessary information is missing, while ensuring draft alerts never automatically transition to new status due to passage of time.

## Decision

We have introduced the `dispatch-pings` daemon and draft alert reminder workflow:

- draft alerting logic: draft alerts track a `ping_at` timestamp (initially set to 10 minutes after creation or update).
- pinger worker: a dedicated background daemon (`dispatch-pings`) polls for pending pings where `status = 'draft'` and `ping_at <= unixepoch()`.
- outbox integration: reminder messages are enqueued into `chatbot_outbox` and delivered reliably via `dispatch-outbox`.
- ping frequency: after sending a reminder, `ping_at` is set to `NULL` to disable further reminders for this draft unless the user actively provides new information (photos, location, or description), which resets `ping_at` to 10 minutes in the future.
- strict isolation: draft alerts remain in `draft` status indefinitely until the user actively provides all required fields.

## Consequences

- operational requirement: production deployments must run the `dispatch-pings` supervisor process alongside `chatbot`, `dispatch-alerts`, and `dispatch-outbox`.
- database schema expansion: addition of the `ping_at` column and `idx_chatbot_alerts_status_ping` index.
- improved engagement: proactive prompts assist users in completing feedback reports correctly.

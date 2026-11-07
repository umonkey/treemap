---
name: queue-commands
description: Checklist for adding, changing, or debugging backend queue commands. Use when touching services/backend/src/services/queue_consumer/** or services/backend/src/services/queue_encoder/**.
---

# Skill: Queue Commands

## Contract

A command exists in three hand-maintained places:

| Concern                                            | File                                                      |
| :------------------------------------------------- | :-------------------------------------------------------- |
| Message struct, `encode()`, `QueueCommand` variant | `services/backend/src/services/queue_consumer/schemas.rs` |
| String decode arm                                  | `services/backend/src/services/queue_encoder/mod.rs`      |
| Processing arm                                     | `services/backend/src/services/queue_consumer/service.rs` |

The compiler does not cross-check them. `QueueCommand::decode` returns `Ok(None)` for an unknown command and `Err` for a malformed payload. `QueueConsumer::process_message` logs both and returns `Ok`, so `run` deletes the message. A missing arm loses the message permanently.

## Adding or Changing a Command

1. Add or update the struct in `schemas.rs` and its `encode()`.
2. Add the `QueueCommand` variant.
3. Add the decode arm. Parse each parameter explicitly (`as_u64`, `as_str`) and return `Error::Queue` when a field is missing.
4. Add the arm to `process_message`.
5. Add a decode test and keep the all-commands decode test in sync; it fails when a variant has no arm.
6. Choose the failure policy:
   - Return `Err` from the handler to delay and retry the message.
   - Record a terminal state and return `Ok` when retries cannot help.

## Debugging

- `Could not parse message` from `queue_encoder` means there is no decode arm for that command string.
- `Unknown message` in `queue_consumer` is the consumer deleting the message.
- Compare the payload with the command's `encode()` before changing parsing.

## Verification

`cd services/backend && make check`

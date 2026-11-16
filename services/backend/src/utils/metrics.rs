//! Sentry Application Metrics for request and queue processing.
//!
//! Metrics carry bounded attributes only: route patterns, methods, status
//! codes, command names and outcomes. Never raw paths or entity ids, because
//! every distinct value becomes a time series in Sentry.
//!
//! Emission is a no-op when Sentry has no client, so the helpers can be called
//! unconditionally.

use sentry::metrics::{counter, distribution};
use sentry::protocol::Unit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Requests at or above this duration are also counted as slow.
pub const SLOW_REQUEST: Duration = Duration::from_secs(5);

/// Route attribute used when the router could not match a pattern.
pub const UNMATCHED_ROUTE: &str = "unmatched";

/// Set by `utils::sentry::init` from `SENTRY_METRICS`. The SDK has no switch
/// of its own, so the helpers check this before emitting.
static ENABLED: AtomicBool = AtomicBool::new(true);

/// Enables or disables metric emission for the whole process.
pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Rewrites a route pattern to a stable, low-cardinality form.
///
/// The regex of a capturing segment is dropped, so
/// `/v1/trees/{id:\\d+}/photos` becomes `/v1/trees/{id}/photos`. Segments
/// without a regex are kept as they are.
pub fn normalize_route(pattern: &str) -> String {
    let mut normalized = String::with_capacity(pattern.len());
    let mut rest = pattern;

    while let Some(open) = rest.find('{') {
        normalized.push_str(&rest[..open]);
        rest = &rest[open..];

        // A regex can contain braces itself (`\d{2}`), so find the matching
        // closing brace by depth instead of taking the first one.
        let Some(close) = matching_brace(rest) else {
            // Unbalanced braces: keep the remainder unchanged.
            normalized.push_str(rest);
            return normalized;
        };

        let segment = &rest[1..close];
        let name = segment.split_once(':').map_or(segment, |(name, _)| name);

        normalized.push('{');
        normalized.push_str(name);
        normalized.push('}');

        rest = &rest[close + 1..];
    }

    normalized.push_str(rest);

    normalized
}

/// Returns the byte index of the brace closing the one at position zero.
fn matching_brace(segment: &str) -> Option<usize> {
    let mut depth = 0usize;

    for (index, character) in segment.char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;

                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

/// Records how long a request took and counts it when it was slow.
pub fn record_request(method: &str, route: &str, status: u16, elapsed: Duration) {
    if !enabled() {
        return;
    }

    let route = route.to_string();
    let method = method.to_string();

    distribution(
        "http.server.request.duration",
        elapsed.as_secs_f64() * 1000.0,
    )
    .unit(Unit::Millisecond)
    .attribute("http.route", route.clone())
    .attribute("http.request.method", method.clone())
    .attribute("http.response.status_code", status)
    .capture();

    if elapsed >= SLOW_REQUEST {
        counter("http.server.request.slow", 1)
            .attribute("http.route", route)
            .attribute("http.request.method", method)
            .attribute("http.response.status_code", status)
            .capture();
    }
}

/// Records that a queue message was processed, how long it took and how long
/// it waited in the queue.
///
/// `error` is a bounded failure kind (`locked`, `database`, `other`) and is
/// attached only when processing failed, so successful traffic does not grow
/// the attribute set.
pub fn record_queue_message(
    command: &str,
    outcome: &str,
    error: Option<&str>,
    waited: Duration,
    elapsed: Duration,
) {
    if !enabled() {
        return;
    }

    let command = command.to_string();
    let outcome = outcome.to_string();
    let error = error.map(str::to_string);

    let mut processed = counter("queue.message.processed", 1)
        .attribute("queue.command", command.clone())
        .attribute("queue.outcome", outcome.clone());

    if let Some(error) = error.clone() {
        processed = processed.attribute("queue.error", error);
    }

    processed.capture();

    let mut duration = distribution("queue.message.duration", elapsed.as_secs_f64() * 1000.0)
        .unit(Unit::Millisecond)
        .attribute("queue.command", command.clone())
        .attribute("queue.outcome", outcome);

    if let Some(error) = error {
        duration = duration.attribute("queue.error", error);
    }

    duration.capture();

    distribution("queue.message.wait", waited.as_secs_f64())
        .unit(Unit::Second)
        .attribute("queue.command", command)
        .capture();
}

/// Records that a queue message had already been attempted at least once.
pub fn record_queue_retry(command: &str) {
    if !enabled() {
        return;
    }

    counter("queue.message.retried", 1)
        .attribute("queue.command", command.to_string())
        .capture();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_plain_path() {
        assert_eq!(normalize_route("/health"), "/health");
    }

    #[test]
    fn normalizes_single_capture() {
        assert_eq!(
            normalize_route("/v1/trees/{id:\\d+}/photos"),
            "/v1/trees/{id}/photos"
        );
    }

    #[test]
    fn normalizes_several_captures() {
        assert_eq!(
            normalize_route("/v1/comments/{entity_type}/{id:\\d+}"),
            "/v1/comments/{entity_type}/{id}"
        );
        assert_eq!(
            normalize_route("/v1/{first:\\w+}/{second:\\d+}"),
            "/v1/{first}/{second}"
        );
    }

    #[test]
    fn keeps_captures_without_regex() {
        assert_eq!(
            normalize_route("/v1/trees/{id}/photos"),
            "/v1/trees/{id}/photos"
        );
    }

    #[test]
    fn keeps_regex_with_braces() {
        assert_eq!(normalize_route("/v1/trees/{id:\\d{2}}"), "/v1/trees/{id}");
    }

    #[test]
    fn normalizes_path_without_captures() {
        assert_eq!(normalize_route("/v1/trees"), "/v1/trees");
        assert_eq!(normalize_route(""), "");
    }

    #[test]
    fn keeps_unbalanced_braces() {
        assert_eq!(normalize_route("/v1/trees/{id"), "/v1/trees/{id");
    }
}

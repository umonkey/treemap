use sentry::protocol::{Event, Request};
use sentry::ClientInitGuard;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

/// Initializes Sentry and logging.
/// Returns a guard that must be kept alive for the duration of the program.
pub fn init() -> Option<ClientInitGuard> {
    let dsn = std::env::var("SENTRY_DSN")
        .ok()
        .filter(|value| !value.is_empty());

    if dsn.is_none() {
        // Since we haven't initialized the logger yet, we use eprintln.
        eprintln!("SENTRY_DSN is not set; Sentry reporting is disabled.");
        env_logger::try_init().ok();
        return None;
    }

    let release = std::env::var("SENTRY_RELEASE")
        .ok()
        .filter(|value| !value.is_empty())
        .map(Cow::Owned)
        .or_else(|| sentry::release_name!());

    let traces_sample_rate = std::env::var("SENTRY_TRACES_SAMPLE_RATE")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.1);

    let guard = sentry::init(sentry::ClientOptions {
        dsn: dsn.and_then(|value| value.parse().ok()),
        release,
        traces_sample_rate,
        before_send: Some(Arc::new(scrub_event)),
        ..Default::default()
    });

    // Initialize the logger with Sentry integration.
    // We wrap env_logger so we still get console output.
    let mut builder = env_logger::Builder::from_default_env();
    let logger = sentry_log::SentryLogger::with_dest(builder.build());

    log::set_boxed_logger(Box::new(logger)).ok();
    // sentry_log ignores Debug and Trace; this only re-enables the env_logger
    // output that RUST_LOG allows.
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("Sentry reporting enabled.");

    Some(guard)
}

/// Flushes queued Sentry events.
///
/// Call before `std::process::exit`, which skips the `ClientInitGuard` drop
/// that would otherwise flush. A no-op when Sentry has no client.
pub fn flush() {
    if let Some(client) = sentry::Hub::current().client() {
        client.flush(Some(Duration::from_secs(2)));
    }
}

/// Removes credentials from the request data attached by `sentry-actix`.
fn scrub_event(mut event: Event<'static>) -> Option<Event<'static>> {
    if let Some(request) = event.request.as_mut() {
        scrub_request(request);
    }

    Some(event)
}

fn scrub_request(request: &mut Request) {
    request.headers.retain(|name, _| {
        !name.eq_ignore_ascii_case("authorization") && !name.eq_ignore_ascii_case("cookie")
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_request_removes_credentials() {
        let mut request = Request {
            headers: [
                ("authorization".to_string(), "Bearer secret".to_string()),
                ("Cookie".to_string(), "session=secret".to_string()),
                ("content-type".to_string(), "application/json".to_string()),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        scrub_request(&mut request);

        assert_eq!(request.headers.len(), 1);
        assert_eq!(
            request.headers.get("content-type").map(String::as_str),
            Some("application/json")
        );
    }
}

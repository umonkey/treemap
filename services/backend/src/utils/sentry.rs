use sentry::ClientInitGuard;

/// Initializes Sentry for CLI commands.
/// Returns a guard that must be kept alive for the duration of the program.
pub fn init() -> Option<ClientInitGuard> {
    let dsn = std::env::var("SENTRY_DSN").ok();

    if dsn.is_none() {
        // Since we haven't initialized the logger yet, we use eprintln.
        eprintln!("SENTRY_DSN is not set; Sentry reporting is disabled.");

        // But we still want logging to work, so we initialize env_logger here.
        env_logger::try_init().ok();
        return None;
    }

    let guard = sentry::init(sentry::ClientOptions {
        dsn: dsn.and_then(|d| d.parse().ok()),
        release: sentry::release_name!(),
        ..Default::default()
    });

    // Initialize the logger with Sentry integration.
    // We wrap env_logger so we still get console output.
    let mut builder = env_logger::Builder::from_default_env();
    let logger = sentry_log::SentryLogger::with_dest(builder.build());

    log::set_boxed_logger(Box::new(logger)).ok();
    log::set_max_level(log::LevelFilter::Info);

    Some(guard)
}

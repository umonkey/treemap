# Logging

The backend application uses crates `log` and `env_logger` for logging.
It sends all messages to `stderr` by default.
There is no built in support for special log targets in the application.
It is the responsibility of the user to catch the log messages and redirect them to a file or other target.

## Example setup

The pre-built Docker image can be found in the `container` folder.
We use `supervisor` to run the application and redirect the log messages to a file.

Supervisor is configured to write logs to the `var/logs` directory, separate files for each component of the application.
There are files `app.log`, `queue.log` and `cron.log`.

Then we use the `logrotate` tool to rotate the log files, compress them and keep a certain number of old log files.
The configuration for that is in `container/rootfs/etc/logrodate.d/treemap`, and it keeps compressed daily logs for 1 year.

## Sentry

Error reporting to Sentry is optional and is disabled unless the `SENTRY_DSN` environment variable is set.
With a DSN, the backend wraps the existing `env_logger` chain in `sentry_log`, so every record logged at `error` level is uploaded as a Sentry event while the console and file output stays the same.
The web server and the CLI commands now use one shared initialiser, and the same rules apply to both.
Before an event is sent, the `Authorization` and `Cookie` request headers are removed so that credentials never leave the process.
When reporting is active the application logs `Sentry reporting enabled.` at startup; without a DSN it prints a note to `stderr` and continues with plain `env_logger`.

`SENTRY_RELEASE` tags events with the deployed git revision.
The pre-built images take it from the `SENTRY_RELEASE` build argument, which the deployment workflow sets to the commit SHA.
`SENTRY_TRACES_SAMPLE_RATE` (default 0.1) controls how many API requests are traced.

Note that the legacy `container/build-yerevan.sh` build sets `PUBLIC_API_ROOT=https://yerevan.treemaps.app/`, which is not listed in the frontend `tracePropagationTargets` in `services/frontend/src/hooks.client.ts`.
Distributed trace linking between the browser and the backend therefore only works for builds that use `services/frontend/.env.production`, where the API root is `https://api.treemaps.app/`.

## More complex usage

For a more complex and busy system, you will probably need a more sophisticated logging setup, such as using a log aggregator like `Fluentd`, `Logstash`, or `Graylog`.

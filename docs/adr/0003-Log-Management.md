# ADR-0003: Backend Log Management and Rotation

- Status: Accepted
- Date: 2025-07-19

## Context

The application backend is a Rust service that runs inside a Docker container. We require a robust, long-term solution for managing the application's logs. The specific requirements are:

* Logs should be rotated on a daily basis.
* Rotated log files must be compressed to save disk space.
* Historical log files should be retained indefinitely for audit and analysis purposes.
* The solution must be reliable and suitable for a production environment.

## Decision Drivers

* Flexibility: The ability to modify the log rotation and retention policy without needing to change, recompile, and redeploy the core application.
* Separation of Concerns: Adherence to the [12-Factor App methodology](https://12factor.net/logs), which recommends that applications treat logs as event streams and not concern themselves with log routing or storage.
* Robustness: The chosen solution should be battle-tested and reliable, minimizing the risk of log loss or application crashes due to logging failures.
* Simplicity of Application Code: The core application logic should remain as simple as possible, focusing on its primary business domain rather than operational concerns.

## Considered Options

### Option 1: In-Application Log Rotation

This approach involves using a dedicated Rust logging crate (e.g., `log4rs`, `flexi_logger`) to handle file rotation, naming, and compression directly within the Rust application process.

Pros:

* The application is fully self-contained.
* No external dependencies like `cron` or `logrotate` are needed in the Docker image.

Cons:

* Log policy is hardcoded. Any change requires a full application redeployment.
* Adds complexity and potential bugs to the application's core code.
* Log rotation and compression can cause performance spikes within the application process.
* Violates the principle of separating application and operational concerns.

### Option 2: External Management via `supervisor` + `logrotate`

This approach involves the application writing all log events to standard streams (`stdout` and `stderr`). External, standard Linux utilities are then responsible for managing the log stream.

Implementation:

1.  The Rust application uses a simple logger like `env_logger` to write to `stdout`/`stderr`.
2.  `supervisor` manages the application process and is configured to capture the output stream into a single, stable log file (e.g., `/var/log/yourapp/app.log`).
3.  `cron` and `logrotate` are installed in the Docker container. A cron job runs `logrotate` daily.
4.  `logrotate` is configured to rotate, compress, and manage the retention of the `app.log` file using the `copytruncate` method.

Pros:

* Extremely flexible; log policy can be changed via configuration files without touching the application.
* Follows the 12-Factor App standard.
* Uses highly reliable, battle-tested system utilities.
* Keeps the application code simple and focused.

Cons:

* The Docker image has a slightly larger footprint due to `cron` and `logrotate`.
* The container's entrypoint is more complex, as it must manage starting both the application and the `crond` process.

## Decision

We have decided to adopt Option 2: External Management via `supervisor` + `logrotate`.

This approach provides the greatest flexibility and aligns with modern best practices for cloud-native and containerized applications. The operational benefits of being able to manage logging policy independently of application deployments far outweigh the minor complexity added to the container's setup.

## Consequences

Positive:

* The application's logging code is minimal and robust.
* The operations team can adjust log rotation, compression, and retention policies easily.
* The architecture is clear and follows well-understood industry patterns.

Negative:

* The final Docker image will be slightly larger.
* An entrypoint script is required to run multiple processes (`crond` and the Rust app) within the container, which requires careful implementation to handle signals correctly.

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


## More complex usage

For a more complex and busy system, you will probably need a more sophisticated logging setup, such as using a log aggregator like `Fluentd`, `Logstash`, or `Graylog`.

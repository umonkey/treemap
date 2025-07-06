# ADR-0002: Adopt TOML for Configuration and File-Based Secrets

* Date: 2025-07-07
* Status: Accepted

## Context

The application has historically relied on environment variables for all configuration, including service endpoints, feature flags, and sensitive data like API tokens and database credentials.

As the application has grown, this approach has presented several challenges:

* Scalability: The number of environment variables has become large and difficult to manage. There is no structure or grouping, making it hard to understand the full configuration landscape.

* Security: Placing secrets directly into environment variables increases the risk of accidental exposure through logs, error reporting tools, or shell history.

* Developer Experience: Local development requires developers to maintain complex `.env` files or shell scripts, which can be cumbersome and error-prone.

* Consistency: Ensuring that all necessary variables are set correctly across different environments (development, staging, production) is a manual and fragile process.

## Decision

We will adopt a hybrid approach for managing configuration and secrets, moving away from an exclusive reliance on environment variables.

1.  Configuration File: All non-sensitive configuration parameters will be moved into a `config.toml` file. This provides a structured, version-controllable, and human-readable format for application settings. The application will load this file on startup.

2.  File-Based Secrets: All sensitive values (API keys, tokens, passwords, etc.) will be removed from environment variables. Instead, they will be provided to the application as individual files in a designated secrets directory (e.g., `/run/secrets/`). This pattern is compatible with modern container orchestration systems like Docker Swarm and Kubernetes.

The application will read the configuration from `config.toml` and then load secrets from the specified file paths. Environment variables will be reserved only for overriding fundamental settings, such as the path to the configuration file or the secrets directory itself.

## Consequences

### Positive:

* Improved Security: Secrets are no longer exposed in the application's environment, significantly reducing the attack surface. Access to secrets can be tightly controlled via file system permissions.

* Better Organization: Configuration is now structured and self-documenting within the `config.toml` file.

* Enhanced Developer Experience: Developers can easily manage a local `config.toml` for their environment without polluting their system with numerous environment variables.

* Clear Separation: There is a clean and explicit distinction between non-sensitive configuration and sensitive secrets.

### Negative:

* Increased Complexity: The application's startup logic is now more complex, as it must read and parse the configuration file and then read multiple secret files.

* Deployment Overhead: Deployment scripts and CI/CD pipelines must be updated to handle the creation and placement of both the `config.toml` and the various secret files.

* Migration Effort: A one-time effort is required to migrate all existing configuration and secrets from environment variables to the new file-based system.

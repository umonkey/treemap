#!/bin/sh

# Initialize development-only Docker Compose secrets.
#
# Never overwrite existing secrets.
# Never copy credentials from the standard .secrets directory.
# All generated values are intended for local development only.

set -eu

# Restrict access to newly created secrets.
umask 077

# Resolve the repository root regardless of the current directory.
ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
SECRETS_DIR="$ROOT_DIR/.local-secrets"

mkdir -p "$SECRETS_DIR"

# Create a secret file only if it does not already exist.
write_if_missing() {
    name="$1"
    value="$2"
    path="$SECRETS_DIR/$name.txt"

    if [ ! -e "$path" ]; then
        printf '%s\n' "$value" > "$path"
    fi
}

# AWS credentials are required to initialize the AWS Batch client.
# These placeholders do not grant access to any AWS resources.
write_if_missing aws_key "local-placeholder"
write_if_missing aws_secret "local-placeholder"
write_if_missing aws_region "us-east-1"

# Placeholder credentials for file storage.
write_if_missing files_key "local-placeholder"
write_if_missing files_secret "local-placeholder"

# Placeholder credentials for unused cloud integrations.
write_if_missing sqs_key "local-placeholder"
write_if_missing sqs_secret "local-placeholder"
write_if_missing osm_token "local-placeholder"

# Local SMTP settings.
# No external SMTP server is configured.
write_if_missing smtp_host "localhost"
write_if_missing smtp_user "local-placeholder"
write_if_missing smtp_secret "local-placeholder"
write_if_missing from_email "dev@localhost.invalid"

# Optional chatbot settings.
# The chatbot is not started in the default local environment.
write_if_missing chatbot_token "local-placeholder"
write_if_missing report_recipients ""

# Generate a unique JWT signing secret for the local installation.
# Never replace an existing JWT secret, as doing so invalidates tokens.
JWT_FILE="$SECRETS_DIR/jwt_secret.txt"

if [ ! -e "$JWT_FILE" ]; then
    if command -v openssl >/dev/null 2>&1; then
        JWT_SECRET="$(openssl rand -hex 32)"
    else
        # Use the operating system random source if OpenSSL is unavailable.
        JWT_SECRET="$(od -An -N32 -tx1 /dev/urandom | tr -d '[:space:]')"
    fi

    printf '%s\n' "$JWT_SECRET" > "$JWT_FILE"
fi

echo "Local development secrets initialized in .local-secrets/"
# Configuration

The application is configured using a configuration file named `config.toml` and file-based secrets to set access tokens.
Please look in the example `backend/config.toml.dist` file for a list of available options and their meaning.
The application has built in sensible defaults and can run without a config file.
But normally you rename the file as `config.toml`, make edits and run the backend.

Sensitive data, such as S3 storage access keys, and the JWT secret, are read from file based secrets.
Those secrets are normally read from the `.secrets` directory, which can be changed in the config file.

A script to run the application with the config file and secrets mounted can look like this:

``` sh
#!/bin/sh
mkdir -p var
docker run --detach -p 8002:8000 \
    --volume $PWD/var:/app/var \
    --volume $PWD/config.toml:/app/config.toml \
    --volume $PWD/.secrets:/app/.secrets \
    --env-file .env \
    --restart unless-stopped \
    --name treemap \
    ghcr.io/umonkey/treemap:latest
```

## Database setup

The database is read from an SQLite file `var/database.sqlite`.
(The name of this file can be changed in the config file.)

To initialize the file when creating a new server, run the following script:

``` sh
sqlite3 var/database.sqlite < dev/schema-sqlite.sql
```

This will create an empty database with the correct schema.

# Tree map

An app for urban tree mapping.  Contains a backend and a HTML web app.


## Features

- List trees around you.
- Add new trees to the map.


## Architecture

The project uses the [monorepo](https://en.wikipedia.org/wiki/Monorepo) approach to keep all the code in one place, for simplicity.  The backend and the frontend are in separate directories.

The backend is a REST API written in Rust.  It uses the [Actix](https://actix.rs/) web framework.  The database is [SQLite](https://www.sqlite.org/), used via [async-sqlite](https://docs.rs/async-sqlite/latest/async_sqlite/).  Long running tasks are dispatched to an SQS queue.  The API documentation can be found in the [wiki](https://github.com/umonkey/treemap/wiki/API).


## Getting started

There will be a Docker image with everything compiled.


## Motivation

This is a pet project.  I love growing trees, [crowdmapping](https://en.wikipedia.org/wiki/Crowdmapping) and programming.  For me this is also an exercise in advancing my Rust skills, by doing something other than I do on my day job.

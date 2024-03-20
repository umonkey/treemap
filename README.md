# Tree Map

**Tree Map** is an application for urban tree mapping.  It allows tree loving enthusiasts to collect data about trees and plants in their city, such as location, height, species, and more.  The data is then used to create a map of the trees in the city, which can be used for various purposes, such as urban planning, environmental studies, and more.

The application is heavily inspired by the [New York City Street Tree Map](https://tree-map.nycgovparks.org/), which is a great example of how tree mapping can be done, and [Treezilla](https://www.treezilla.org/), which is a similar project in the UK.


## Features

- List trees around you.
- Add new trees to the map.
- Add photos, comments to trees.


## Architecture

The project uses the [monorepo](https://en.wikipedia.org/wiki/Monorepo) approach to keep all the code in one place, for simplicity.  The backend and the frontend are in separate directories.

The backend is a REST API written in Rust.  It uses the [Actix](https://actix.rs/) web framework.  The database is [SQLite](https://www.sqlite.org/), used via [async-sqlite](https://docs.rs/async-sqlite/latest/async_sqlite/).  Long running tasks are dispatched to an SQS queue.  The API documentation can be found in the [wiki](https://github.com/umonkey/treemap/wiki/API).

The frontend is written in TypeScript, using the [React](https://reactjs.org/) library.  The UI is built with [Material-UI](https://material-ui.com/).  The map is provided by [Leaflet](https://leafletjs.com/).  The frontend is a [Progressive Web App](https://web.dev/progressive-web-apps/), which means it can be installed on a mobile device and used offline.


## Getting started

There will be a Docker image with everything compiled.


## Motivation

This is a pet project.  I love growing trees, [crowdmapping](https://en.wikipedia.org/wiki/Crowdmapping) and programming.  For me this is also an exercise in advancing my Rust skills, by doing something other than I do on my day job.

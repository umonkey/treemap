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

I love growing trees and [crowdmapping](https://en.wikipedia.org/wiki/Crowdmapping).  I always wanted to have a map of all the trees in my city, so that when it's time to collect seeds for the next season, I know exactly where to go.  I could not find any such map, so I decided to make one myself.

I also love programming.  So I decided to make the app myself, as a pet project.  I chose Rust for the backend, because it's a language I want to learn better.  I chose TypeScript for the frontend, because it's a language I already know well, and I like it.

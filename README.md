# Tree Map

**Tree Map** is an application for urban tree mapping.  It allows tree loving enthusiasts to collect data about trees and plants in their city, such as location, height, species, and more.  The data is then used to create a map of the trees in the city, which can be used for various purposes, such as urban planning, environmental studies, recreational and more.

The application is heavily inspired by the [New York City Street Tree Map](https://tree-map.nycgovparks.org/) (870k trees), which is a great example of how tree mapping can be done, and [Treezilla](https://www.treezilla.org/) (1M trees), which is a similar project in the UK.  There might be other similar projects, but none that you can run easily and use in your own city, as far as I know.

[About](https://github.com/umonkey/treemap/wiki/Home) - [API Documentation](https://github.com/umonkey/treemap/wiki/API) - [TODO](https://github.com/umonkey/treemap/wiki/TODO) - [Demo](https://treemap.umonkey.net/)


## Current status

There is a backend and frontend prototype.  One can [run this all](https://github.com/umonkey/treemap/wiki/Installation#running-with-docker) on a local machine in a Docker container, use a browser to see the map and add new trees with no details.  Authentication, edits, comments, files, search and everything else is missing.


## Features

- [x] List trees around you.
- [x] Show details for a single tree.
- [ ] Find trees by common or latin name.
- [x] Add new trees to the map.
- [ ] Add photos, comments to trees.
- [x] Mobile first.
- [ ] Exchange data with OSM.


## Architecture

The backend is a REST API written in Rust, using the [Actix](https://actix.rs/) web framework.  The database is [SQLite](https://www.sqlite.org/), used via [async-sqlite](https://docs.rs/async-sqlite/latest/async_sqlite/).  The API documentation can be found in the [wiki](https://github.com/umonkey/treemap/wiki/API).  Rust is chosen for the backend to make it fast and safe, being able to handle a lot of requests in tiny environments.

The frontend is written in TypeScript, using the [React](https://reactjs.org/) library.  The UI is built with [Material-UI](https://material-ui.com/).  The map is provided by [Leaflet](https://leafletjs.com/), with vector tiles from [MapTiler](https://www.maptiler.com/) for better zooming.  The frontend is a [Progressive Web App](https://web.dev/progressive-web-apps/), which means it will be installable on a mobile device and used offline one day.

The project uses the [monorepo](https://en.wikipedia.org/wiki/Monorepo) approach to keep all the code in one place, for simplicity.  The backend and the frontend are in separate directories.


## Getting started

You can run the app locally using a pre-composed Docker image.  You will need [Docker](https://www.docker.com/) installed on your machine.  The command line is:

```
$ docker run -p 8000:8000 -v $PWD/var:/app/var ghcr.io/umonkey/treemap:latest
```

With this command you'll be running the application, both backend and frontend, on port [localhost:8000](http://localhost:8000/).  The SQLite database will be stored in `./var` directory, and should persist container restarts.  The database will be created automatically on the first run.  To run a public service, this would need to be hidden behind a CDN.  (The service is not yet ready for public use.)

Please see the [installation instructions](https://github.com/umonkey/treemap/wiki/Installation) for more details.


## Demo

A demo version of the current state of the project is available at [treemap.umonkey.net](https://treemap.umonkey.net/).  The demo is running on a small server, so it might be slow or unresponsive.  The demo is not meant for production use, but for testing and development purposes only.  The demo is not guaranteed to be available at all times, and might be taken down at any time.


## Motivation

I love growing trees and [crowdmapping](https://en.wikipedia.org/wiki/Crowdmapping).  I always wanted to have a map of all the trees in my city, so that when it's time to collect seeds for the next season, I know exactly where to go.  Or to check how a rare species feels in our climate.  I could not find any such map, so I decided to make one myself.

I also love programming.  So I decided to make the app myself, as a pet project.  I chose Rust for the backend, because it's a language I want to learn better.  I chose TypeScript for the frontend, because it's a language I already know well, and I like it.

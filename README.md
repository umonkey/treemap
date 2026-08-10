# Tree Map

**Tree Map** is an application for urban tree mapping.
It helps local communities and enthusiasts collect and publish data about trees and plants around them.
The application has a database of trees, their coordinates, current state, photos, and changes of that data over time.
The data can be used for various purposes, such as urban planning, environmental studies, recreational, and more.

The application is heavily inspired by the [New York City Street Tree Map](https://tree-map.nycgovparks.org/) (870k trees), which is a great example of how tree mapping can be done, and [Treezilla](https://www.treezilla.org/) (1M trees), which is a similar project in the UK.
There might be other similar projects, but none that you can run easily and use in your own city, as far as I know.

[Features](docs/Features.md) - [Demo](https://yerevan.treemaps.app/) - [TODO](https://github.com/umonkey/treemap/issues) 


## Current status

The application is fully working an used by the [Yerevan Tree Map](https://yerevan.treemaps.app/) project, although some features are still being developed, see [Feature Map](docs/Features.md). The application can be quickly deployed on any server using Docker Compose.


## Architecture

The backend is a REST API written in Rust, using the [Actix](https://actix.rs/) web framework.
The database is [SQLite](https://www.sqlite.org/), used via [async-sqlite](https://docs.rs/async-sqlite/latest/async_sqlite/).
Rust is chosen for the backend to make it fast and safe, being able to handle a lot of requests in tiny environments.

The frontend is written in TypeScript, using the [Svelte](https://svelte.dev/) framework.
The map is provided by [MapLibre GL](https://maplibre.org/maplibre-gl-js/docs/).
The frontend is a [Progressive Web App](https://web.dev/progressive-web-apps/), which means it will be installable on a mobile device and used offline one day.

The app works with any S3 compatible storage.
For computational heavy tasks, such as processing [360 street panoramas](docs/Panoramas.md), we use [AWS Batch](https://aws.amazon.com/batch/).

The project uses the [monorepo](https://en.wikipedia.org/wiki/Monorepo) approach to keep all the code in one place, for simplicity.
The services (backend, frontend, proxy) are located in the `services/` directory.
Architectural decisions are logged in the [ADR folder](docs/adr/).


## Getting started

You can run the app locally using [Docker Compose](https://docs.docker.com/compose/), like this:

```
docker compose up
```

With this command you'll be running the application, the frontend is available at [localhost:5173](http://localhost:5173/).
The SQLite database will be stored in a Docker volume, and should persist container restarts.
The database will be created automatically on the first run.
To run a public service, this would need a CDN.


## Demo

There is a fully working public installation at [Yerevan Tree Map](https://yerevan.treemaps.app/).


## Help wanted

If you are an UI/UX designer, we would love some feedback on how to improve the user experience.
The focus should be on people doing the mapping work in the fields: adding new trees, measuring existing ones, and so on.

If you are into machine learning and computer vision, we would love some help on automating the process of initial mapping.
Manually adding trees is the most time consuming thing so far.
We would like to use video records of streets from a car mounted camera, to automatically detect trees and add them to the OSM map.
We have [a ticket](https://github.com/umonkey/treemap/issues/61) for this, please join.

# Database configuration

The application uses SQLite for the data storage.  The reason is that it's very lightweight and simplifies deployment of small apps; you can read ADR-0004 for details.  For an app that mostly reads the data, SQLite can withstand quite heavy loads.

Whenever the app runs in the cloud, where local storage is not available, or needs to be scaled by running multiple API replicas, a cloud hosted version of SQLite can be used, like [Turso](https://turso.tech/).

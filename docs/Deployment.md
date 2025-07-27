# Deployment

This document describes some tested deployment options.


## Fly.io

[Fly.io][1] is a cloud platform which is focused on running dockerized applications.
The platform offers blue-green deployments and health checks, so it is pretty robust and easy to use.
The best option for a small sized community is to pack the whole application into a single Docker container and deploy it to Fly.io.

You can look at the `fly.toml` file in this repository for an example of how to do that.

[1]: https://fly.io/

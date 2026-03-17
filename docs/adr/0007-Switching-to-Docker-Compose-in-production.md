# ADR-0007: Switching to Docker Compose for Production Hosting

- Date: 2026-03-17
- Status: Accepted

## Context

The current frontend hosting setup does not allow for dynamic metadata injection in the HTML headers. This is a significant issue for social media platforms and messaging apps like Telegram, which rely on these tags to generate visual link previews. The absence of these previews negatively impacts the application publicity and user engagement.

To support metadata injection, the frontend needs to be served through a proxy that can modify the response based on the requested URL.

Additionally, we previously used Turso as a managed database, which incurred monthly costs. A small server at DigitalOcean is capable of hosting our app and its database at a lower cost than we paid for Turso.

## Decision

We will switch to a Docker Compose based setup for production hosting.

- Proxying: a reverse proxy will be used to serve the frontend and inject dynamic metadata into the HTML.
- Containerization: both the frontend and backend will be containerized and managed via Docker Compose, matching our local development environment.
- Server: the application will be hosted on a small DigitalOcean server.

## Consequences

Positive:

- Social media engagement: visual cards in Telegram and other platforms will work correctly, improving the project publicity.
- Cost efficiency: the cost of a small DigitalOcean server is lower than the previous cost of Turso.
- Environment parity: the production environment will closely match the local development setup.

Negative:

- Server management: we now need to run and maintain a server, which adds operational overhead.

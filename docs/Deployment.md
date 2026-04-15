# Deployment

## Frontend

The frontend is built into static files and deployed to Cloudflare Pages (a hosting solution for static websites). The environment is read from `.env.production` while building, but can be overridden using real environment variables.

The build and deployment process is fully orchestrated by the `.github/workflows/deploy-frontend.yml` workflow file. It needs a Cloudflare API token, issued [here](https://dash.cloudflare.com/profile/api-tokens) and stored in the repo secrets [here](https://github.com/umonkey/treemap/settings/secrets/actions).

The current configuration can be reviewed in the [Cloudflare console](https://dash.cloudflare.com/d98a16a0928f8e884bf6bd54edc13181/pages/view/trees-of-yerevan), the last deployed version can be accessed via [trees-of-yerevan.pages.dev](https://trees-of-yerevan.pages.dev/).

## Server Setup

The application runs under Docker Compose, using two containers: (1) the application, both frontend and backend, and (2) caddy, a reverse proxy, which handles SSL for us via Cloudflare. To have this running, you set it up like this.

1. Create a new server, e.g. a DigitalOcean droplet, with Ubuntu as the operating system.
2. Install packages: `docker.io docker-compose-v2`
3. Create a user named `app`: `adduser app`
4. Add that user to the `docker` group: `usermod -aG docker app`
5. Login to that user: `su app`

# Deployment

## Frontend

The frontend is built into static files and deployed to Cloudflare Pages (a hosting solution for static websites).  The environment is read from `.env.production` while building, but can be overridden using real environment variables.

The build and deployment process is fully orchestrated by the `.github/workflows/deploy-frontend.yml` workflow file.

The current configuration can be reviewed in the [Cloudflare console](https://dash.cloudflare.com/d98a16a0928f8e884bf6bd54edc13181/pages/view/trees-of-yerevan), the last deployed version can be accessed via [trees-of-yerevan.pages.dev](https://trees-of-yerevan.pages.dev/).

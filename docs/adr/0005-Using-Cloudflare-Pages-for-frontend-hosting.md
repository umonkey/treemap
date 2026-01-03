# ADR-0004: Use Cloudflare Pages for Static Frontend Hosting

- Date: 2026-01-04
- Status: Accepted

## Context

Trees of Yerevan requires a hosting solution for its static frontend application. The current traffic projections are low, allowing us to leverage free tier quotas of various PaaS providers.

Our primary engineering constraints for this selection are:

- Minimizing Operational Overhead: We want to avoid managing underlying infrastructure or complex deployment pipelines.
- Cost Efficiency: The solution should be cost-effective for our current low-volume traffic.
- Simplicity: The setup and ongoing management should be straightforward to prevent "configuration drift" or excessive cognitive load on the team.

We initially considered a self-hosted Nginx setup and GitHub Pages. However, we identified that self-hosting introduces unnecessary maintenance liabilities (patching, config management), and GitHub Pages presented configuration complexities that do not align with our goal of simplicity.

## Decision

We will utilize Cloudflare Pages to host the static frontend of Trees of Yerevan.

We are choosing Cloudflare Pages because it offers the optimal balance of operational simplicity and performance for our requirements.

- Rejection of Self-Hosted Nginx: Rejected due to the high operational cost of maintaining servers, patching OS/software, and configuring scalability for a currently low-volume application.
- Rejection of GitHub Pages: Rejected due to the comparative complexity in configuration and management overhead required to achieve the desired deployment workflow.

## Consequences

Positive:

- Zero Operational Overhead: We eliminate the need for OS patching, web server configuration (Nginx), and infrastructure maintenance.
- Cost Efficiency: The application fits comfortably within Cloudflare's free tier, incurring zero hosting costs at current traffic levels.
- Global Performance: The static assets will be served via Cloudflare's global edge network, ensuring low latency without additional CDN configuration.
- Simplified Deployment: Deployment is handled via direct integration, reducing the cognitive load on the team compared to maintaining custom deployment scripts or complex GitHub Pages workflows.

Negative:

- Vendor Lock-in: We become dependent on the Cloudflare ecosystem. Migrating away in the future would require setting up a new hosting provider and potentially re-configuring domain DNS and build settings.
- Traffic Limits: While currently sufficient, should traffic spike unexpectedly beyond the free tier limits, we may face immediate cost implications or rate limiting (though this is a low risk based on current projections).

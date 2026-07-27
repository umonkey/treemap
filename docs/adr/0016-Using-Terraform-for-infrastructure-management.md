# ADR 0016: Using Terraform for Infrastructure Management

- Date: 2026-07-27
- Status: accepted

## Context

The application infrastructure has grown to span multiple cloud providers, specifically AWS and DigitalOcean. This multi-cloud setup includes several complex components:

- Storage: multiple buckets across both providers with specific CDN and CORS configurations.
- Compute: AWS Batch resources (compute environments, job queues, and job definitions) for processing panoramas and other data.

Managing these resources manually through the respective cloud consoles has become a significant operational burden. It is prone to human error, difficult to track changes over time, and makes it nearly impossible to reproduce the environment for testing or disaster recovery.

## Decision

We will use Terraform to manage the application infrastructure as code (IaC).

Terraform was chosen because:

- Provider support: it has excellent first-class support for both AWS and DigitalOcean, as well as many other vendors.
- State management: it maintains a state file that maps the configuration to real-world resources.
- Remote state: we use an S3 bucket to store the Terraform state file, ensuring it is accessible to all team members.
- Location: all Terraform files are in the `infra` folder.
- Deployment: changes are applied manually by an operator with enough permissions, there is no CI/CD integration for this for security reasons (it doesn't happen too often and doesn't take much time so there is no point in extra risks).
- Reproducibility: configurations can be versioned and applied to different environments consistently.
- Visibility: `terraform plan` allows us to see exactly what changes will be made before they are applied.

## Consequences

- Consistency: infrastructure will be defined in code, ensuring that all environments are configured identically.
- Auditability: all infrastructure changes will be tracked via version control (Git).
- Efficiency: automated provisioning reduces the time spent on manual configuration.
- Learning curve: team members will need to be familiar with HCL (HashiCorp Configuration Language) and Terraform workflows.
- State management: we must securely manage the Terraform state file (e.g., using a remote backend with locking).

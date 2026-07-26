# Infrastructure Setup

This folder contains Terraform files to set up the required AWS environment.  This includes resources needed to run AWS Batch tasks.

## Configuration

### DigitalOcean

1. Go to <https://cloud.digitalocean.com/account/api/tokens> and get an API access token.
2. Put it in `TF_VAR_do_token`
3. Go to <https://cloud.digitalocean.com/spaces/access_keys> and get a Spaces API key/secret.
4. Put them in `TF_VAR_do_spaces_access_key` and `TF_VAR_do_spaces_secret_key`.

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.56.0"
    }
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.96.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.region
}

# DigitalOcean Spaces is S3-compatible, but the DigitalOcean Terraform provider 
# does not currently support bucket policies. We use the AWS provider with 
# a custom endpoint to manage these policies.
provider "aws" {
  alias                       = "spaces"
  region                      = "us-east-1" # DigitalOcean ignores this when using custom endpoints
  access_key                  = var.do_spaces_access_id
  secret_key                  = var.do_spaces_secret_key
  s3_use_path_style           = false
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_region_validation      = true # Necessary because 'fra1' is not a standard AWS region
  skip_requesting_account_id  = true

  endpoints {
    s3 = "https://fra1.digitaloceanspaces.com"
  }
}

provider "digitalocean" {
  token             = var.do_token
  spaces_access_id  = var.do_spaces_access_id
  spaces_secret_key = var.do_spaces_secret_key
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

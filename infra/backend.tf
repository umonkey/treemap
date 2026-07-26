terraform {
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }

  backend "s3" {
    bucket = "treemap-terraform-7dswpe5g"
    key    = "terraform.tfstate"
    region = "eu-central-1"
  }
}

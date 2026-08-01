terraform {
  backend "s3" {
    bucket = "treemap-terraform-7dswpe5g"
    key    = "terraform.tfstate"
    region = "eu-central-1"
  }
}

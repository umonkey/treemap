variable "region" {
  type        = string
  description = "AWS region"
  default     = "eu-central-1"
}

variable "batch_instance_type" {
  type        = list(string)
  description = "EC2 instance types for AWS Batch compute environment"
  default = [
    "m6i.xlarge", "m6a.xlarge", "m7i.xlarge", "m7a.xlarge", "m5.xlarge"
  ]
}

variable "do_token" {
  type        = string
  description = "DigitalOcean API token"
  sensitive   = true
}

variable "do_spaces_access_id" {
  type        = string
  description = "DigitalOcean Spaces access ID"
  sensitive   = true
}

variable "do_spaces_secret_key" {
  type        = string
  description = "DigitalOcean Spaces secret key"
  sensitive   = true
}

variable "do_project_id" {
  type        = string
  description = "DigitalOcean project ID"
  default     = "5af28a34-90ae-4738-9c14-ce604676ba4c"
}

variable "domain" {
  type        = string
  description = "Domain name for SES and other services"
  default     = "treemaps.app"
}

variable "cloudflare_api_token" {
  type        = string
  description = "Cloudflare API token"
  sensitive   = true
}

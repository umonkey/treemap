variable "region" {
  type        = string
  description = "AWS region"
  default     = "eu-central-1"
}

variable "batch_instance_type" {
  type        = list(string)
  description = "EC2 instance types for AWS Batch compute environment"
  default     = [
    "c6i.large", "c6a.large", "c5.large", "m6i.large", "m5.large",
    "c6i.xlarge", "c6a.xlarge", "c5.xlarge", "m6i.xlarge", "m5.xlarge",
    "c6i.2xlarge", "c6a.2xlarge", "c5.2xlarge", "m6i.2xlarge", "m5.2xlarge"
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

variable "region" {
  type        = string
  description = "AWS region"
  default     = "eu-central-1"
}

variable "batch_instance_type" {
  type        = string
  description = "EC2 instance type for AWS Batch compute environment"
  default     = "c6i.xlarge"
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

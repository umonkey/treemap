variable "region" {
  type        = string
  description = "AWS region"
  default     = "eu-central-1"
}

variable "batch_instance_type" {
  type        = list(string)
  description = "EC2 instance types for AWS Batch compute environment"
  default     = [
    "c5.xlarge", "c5a.xlarge", "c5d.xlarge", "c6i.xlarge", "c6a.xlarge",
    "m5.xlarge", "m5a.xlarge", "m5d.xlarge", "m6i.xlarge", "m6a.xlarge",
    "r5.xlarge", "r5a.xlarge", "r5d.xlarge", "r6i.xlarge", "r6a.xlarge",
    "c7i.xlarge", "c7a.xlarge", "m7i.xlarge", "m7a.xlarge", "r7i.xlarge", "r7a.xlarge",
    "c5.2xlarge", "c5a.2xlarge", "c6i.2xlarge", "c6a.2xlarge",
    "m5.2xlarge", "m5a.2xlarge", "m6i.2xlarge", "m6a.2xlarge",
    "r5.2xlarge", "r5a.2xlarge", "r6i.2xlarge", "r6a.2xlarge",
    "c7i.2xlarge", "m7i.2xlarge", "r7i.2xlarge"
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

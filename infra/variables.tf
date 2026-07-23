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

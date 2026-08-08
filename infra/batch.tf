# Set up AWS Batch processing pipelines.
#
# We have two types of jobs:
#
# (1) The transcoder job uses ffmpeg to downsample huge videos into 360p,
# normally takes up to 10 minutes, and doesn't need much resources, as
# ffmpeg cannot be parallelized heavily.
#
# (2) The extractor job runs the full OpenSfM pipeline, see `services/extractor`
# for details, the entry point is `bin/process`.  This normally takes 30 to 60 minutes.
#
# We don't use spot instances as the jobs are rather long running and are getting
# killed frequently and we ended up spending more on retries that with on-demand mode.
#
# Note that we almost never run multiple jobs in parallel, so no point in requesting
# nodes larger than we actually need.

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

resource "aws_security_group" "batch_sg" {
  name        = "treemap-batch-sg"
  description = "Security group for AWS Batch compute environment instances"
  vpc_id      = data.aws_vpc.default.id

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_batch_compute_environment" "treemap" {
  name_prefix  = "treemap-ce-"
  type         = "MANAGED"
  service_role = aws_iam_role.aws_batch_service_role.arn

  compute_resources {
    type               = "FARGATE"
    max_vcpus          = 256
    subnets            = data.aws_subnets.default.ids
    security_group_ids = [aws_security_group.batch_sg.id]
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_batch_job_queue" "treemap" {
  name     = "treemap-transcode"
  state    = "ENABLED"
  priority = 1

  compute_environment_order {
    compute_environment = aws_batch_compute_environment.treemap.arn
    order               = 1
  }
}

resource "aws_batch_job_definition" "transcoder" {
  name                  = "treemap-transcoder"
  type                  = "container"
  platform_capabilities = ["FARGATE"]

  retry_strategy {
    attempts = 3
  }

  container_properties = jsonencode({
    image            = "ghcr.io/umonkey/treemap-transcoder:latest"
    executionRoleArn = aws_iam_role.fargate_execution_role.arn
    resourceRequirements = [
      {
        value = "2"
        type  = "VCPU"
      },
      {
        value = "4096"
        type  = "MEMORY"
      }
    ]
    ephemeralStorage = {
      sizeInGiB = 100
    }
    fargatePlatformConfiguration = {
      platformVersion = "LATEST"
    }
    networkConfiguration = {
      assignPublicIp = "ENABLED"
    }
  })
}

resource "aws_batch_job_definition" "extractor" {
  name                  = "treemap-extractor"
  type                  = "container"
  platform_capabilities = ["FARGATE"]

  retry_strategy {
    attempts = 5
  }

  # Use 30 GB Fargate tier with 4 vCPU.
  container_properties = jsonencode({
    image            = "ghcr.io/umonkey/treemap-extractor:latest"
    executionRoleArn = aws_iam_role.fargate_execution_role.arn
    command          = ["bin/process"]
    resourceRequirements = [
      {
        value = "4"
        type  = "VCPU"
      },
      {
        value = "30720"
        type  = "MEMORY"
      }
    ]
    ephemeralStorage = {
      sizeInGiB = 100
    }
    fargatePlatformConfiguration = {
      platformVersion = "LATEST"
    }
    networkConfiguration = {
      assignPublicIp = "ENABLED"
    }
  })
}

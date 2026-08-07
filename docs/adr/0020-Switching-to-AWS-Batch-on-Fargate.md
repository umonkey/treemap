# ADR 0020: Switching to AWS Batch on Fargate

- Date: 2026-08-07
- Status: accepted

## Context

The previous infrastructure implementation used managed EC2 instances for AWS Batch workloads. While cost-effective, this approach introduced several operational challenges during heavy data processing tasks such as OpenSfM reconstruction:

- startup delays: provisioning and joining new EC2 instances often took significant time, leading to prolonged queues between job retries.
- resource mismatch: fixed EC2 instance sizing led to either resource starvation causing host termination or wasted capacity.
- maintenance overhead: managing AMIs and launch templates added unnecessary complexity.

## Decision

We will switch the AWS Batch compute environment and job definitions from EC2 to AWS Fargate.

This solution is selected because:

- instant provisioning: Fargate containers start almost immediately, eliminating boot-time delays.
- granular allocation: we can specify exact CPU and memory requirements without hardware tier constraints.
- reliability: Fargate abstracts the underlying host, avoiding unhandled host terminations and ensuring clean application-level error logs.
- operational simplicity: removes AMI management and instance scaling maintenance.

## Consequences

- cost: Fargate carries a higher per-hour resource cost compared to EC2 instances.
- networking: tasks require public IP assignment or NAT configuration for external image pulls and S3 communication.
- resource predictability: eliminates scheduling bottlenecks and reduces overall pipeline execution time.

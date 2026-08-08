# ADR 0017: Using AWS Batch for Street View Processing

- Date: 2026-07-27
- Status: accepted

## Context

The main application backend is designed to be lightweight and efficient, primarily handling API requests and data management. However, the street view processing pipeline introduces significantly higher resource demands. Specifically:

- transcoding: processing 8K videos into smaller segments or alternative formats.
- scenery reconstruction: running OpenSfM for 3D reconstruction and panorama alignment.

These tasks require substantial CPU and memory resources. Furthermore, high-resolution (8K) image extraction requires high disk I/O throughput, which proved insufficient on Fargate's ephemeral storage. Given that street view data processing happens sporadically and does not follow a predictable or constant load, maintaining dedicated high-performance hardware would be inefficient and costly.

## Decision

We will use AWS Batch with EC2 compute environments and GP3 storage to handle all street view and heavy media processing tasks.

This solution is selected because:

- on-demand scaling: it uses AWS Batch to automatically provision the required compute resources when jobs are submitted and scales down to zero when idle.
- managed infrastructure: it removes the need to manage individual EC2 clusters manually while utilizing launch templates for storage configuration.
- storage throughput: EC2 instances configured with 100 GB gp3 root volumes provide the necessary I/O throughput for high-resolution image extraction where Fargate ephemeral storage limits were exceeded.
- containerization: the processing logic (including ffmpeg and OpenSfM) will be packaged as Docker images, ensuring consistency between environments.

## Consequences

- performance: reliable high-speed I/O for 8K video frame extraction and OpenSfM reconstruction.
- scalability: the system can handle large bursts of data processing by launching multiple parallel jobs.
- operational simplicity: zero maintenance for idle compute resources.
- job management: we need to implement a mechanism in the backend to submit jobs to AWS Batch and monitor their status.

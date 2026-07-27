# ADR 0017: Using AWS Batch for Street View Processing

- Date: 2026-07-27
- Status: accepted

## Context

The main application backend is designed to be lightweight and efficient, primarily handling API requests and data management. However, the street view processing pipeline introduces significantly higher resource demands. Specifically:

- Transcoding: processing 8K videos into smaller segments or alternative formats.
- Scenery reconstruction: running OpenSfM for 3D reconstruction and panorama alignment.

These tasks require substantial CPU and memory resources. Given that street view data processing happens sporadically and does not follow a predictable or constant load, maintaining dedicated high-performance hardware would be inefficient and costly.

## Decision

We will use AWS Batch to handle all street view and heavy media processing tasks.

This solution is selected because:

- On-demand scaling: it uses AWS Batch to automatically provision the required compute resources when jobs are submitted and scales down to zero when idle.
- Managed infrastructure: it removes the need to manage individual EC2 instances or clusters manually.
- Spot instance support: we will utilize EC2 Spot instances to significantly reduce costs, as the processing tasks are batch-oriented and can tolerate interruptions.
- Containerization: the processing logic (including ffmpeg and OpenSfM) will be packaged as Docker images, ensuring consistency between environments.

## Consequences

- Cost efficiency: using Spot instances will provide up to 90% cost savings compared to on-demand pricing.
- Scalability: the system can handle large bursts of data processing by launching multiple parallel jobs.
- Operational simplicity: zero maintenance for idle compute resources.
- Handling interruptions: the application must be designed to handle Spot instance interruptions, potentially by implementing checkpoints or allowing for job retries in AWS Batch.
- Job management: we need to implement a mechanism in the backend to submit jobs to AWS Batch and monitor their status.

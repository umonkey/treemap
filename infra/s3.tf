# AWS S3 bucket for manual archival of panorama source videos.
#
# Source videos are write-once archival data: they transition to Glacier
# Deep Archive immediately and are permanently deleted after one year.
# Deep Archive has a 180-day minimum storage duration, so the 365-day
# expiration avoids early-deletion fees.

resource "aws_s3_bucket" "panoramas_sources" {
  bucket = "treemaps-panoramas-sources-bnu4rf62"
}

resource "aws_s3_bucket_public_access_block" "panoramas_sources" {
  bucket = aws_s3_bucket.panoramas_sources.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_server_side_encryption_configuration" "panoramas_sources" {
  bucket = aws_s3_bucket.panoramas_sources.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "panoramas_sources" {
  bucket = aws_s3_bucket.panoramas_sources.id

  rule {
    id     = "archive-and-expire"
    status = "Enabled"

    filter {}

    transition {
      days          = 0
      storage_class = "DEEP_ARCHIVE"
    }

    expiration {
      days = 365
    }

    abort_incomplete_multipart_upload {
      days_after_initiation = 7
    }
  }
}

# Dedicated uploader for the panorama source video archive.
# Upload + list only: no GetObject (Deep Archive reads need a restore) and no delete.

resource "aws_iam_user" "panoramas_sources_uploader" {
  name = "panoramas-sources-uploader"
}

resource "aws_iam_access_key" "panoramas_sources_uploader" {
  user = aws_iam_user.panoramas_sources_uploader.name
}

resource "aws_iam_user_policy" "panoramas_sources_uploader" {
  name = "panoramas-sources-uploader-policy"
  user = aws_iam_user.panoramas_sources_uploader.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "s3:ListBucket",
          "s3:GetBucketLocation",
          "s3:ListBucketMultipartUploads"
        ]
        Resource = aws_s3_bucket.panoramas_sources.arn
      },
      {
        Effect = "Allow"
        Action = [
          "s3:PutObject",
          "s3:AbortMultipartUpload",
          "s3:ListMultipartUploadParts"
        ]
        Resource = "${aws_s3_bucket.panoramas_sources.arn}/*"
      }
    ]
  })
}

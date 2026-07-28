resource "digitalocean_spaces_bucket" "panoramas" {
  name   = "panoramas-px3qodu3"
  region = "fra1"
  acl    = "public-read"
}

resource "digitalocean_spaces_bucket_cors_configuration" "panoramas" {
  region = digitalocean_spaces_bucket.panoramas.region
  bucket = digitalocean_spaces_bucket.panoramas.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "HEAD"]
    allowed_origins = ["*"]
    max_age_seconds = 3000
  }
}

resource "digitalocean_cdn" "panoramas" {
  origin = digitalocean_spaces_bucket.panoramas.bucket_domain_name
}

resource "digitalocean_project_resources" "treemap" {
  project   = var.do_project_id
  resources = [digitalocean_spaces_bucket.panoramas.urn]
}

resource "aws_s3_bucket_policy" "panoramas" {
  provider = aws.spaces
  bucket   = digitalocean_spaces_bucket.panoramas.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "arn:aws:s3:::panoramas-px3qodu3/*"
      }
    ]
  })
}

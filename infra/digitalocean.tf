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

resource "digitalocean_spaces_bucket" "panoramas_tmp" {
  name   = "treemap-panoramas-tmp-g6dxkoua"
  region = "fra1"
}

resource "digitalocean_spaces_bucket_cors_configuration" "panoramas_tmp" {
  region = digitalocean_spaces_bucket.panoramas_tmp.region
  bucket = digitalocean_spaces_bucket.panoramas_tmp.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "PUT", "POST"]
    allowed_origins = ["*"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

resource "digitalocean_project_resources" "treemap" {
  project = var.do_project_id
  resources = [
    digitalocean_spaces_bucket.panoramas.urn,
    digitalocean_spaces_bucket.panoramas_tmp.urn,
    digitalocean_spaces_bucket.backups.urn,
    digitalocean_spaces_bucket.dataset.urn,
    digitalocean_spaces_bucket.tiles.urn,
    digitalocean_spaces_bucket.treemap.urn,
    digitalocean_droplet.app.urn,
  ]
}

resource "digitalocean_droplet" "app" {
  name   = "cloud.treemaps.app"
  region = "fra1"
  size   = "s-1vcpu-512mb-10gb"
  image  = "195932981"
}

resource "digitalocean_spaces_bucket" "backups" {
  name   = "treemap-backups-uzbmpe2a"
  region = "fra1"
}

resource "digitalocean_spaces_bucket" "dataset" {
  name   = "treemap-dataset"
  region = "fra1"
  acl    = "public-read"
}

resource "aws_s3_bucket_policy" "dataset" {
  provider = aws.spaces
  bucket   = digitalocean_spaces_bucket.dataset.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "arn:aws:s3:::treemap-dataset/*"
      }
    ]
  })
}

resource "digitalocean_spaces_bucket" "tiles" {
  name   = "treemap-tiles"
  region = "fra1"
  acl    = "public-read"
}

resource "digitalocean_spaces_bucket_cors_configuration" "tiles" {
  region = digitalocean_spaces_bucket.tiles.region
  bucket = digitalocean_spaces_bucket.tiles.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "HEAD"]
    allowed_origins = ["*"]
    max_age_seconds = 3000
  }
}

resource "digitalocean_cdn" "tiles" {
  origin = digitalocean_spaces_bucket.tiles.bucket_domain_name
}

resource "aws_s3_bucket_policy" "tiles" {
  provider = aws.spaces
  bucket   = digitalocean_spaces_bucket.tiles.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "arn:aws:s3:::treemap-tiles/*"
      }
    ]
  })
}

resource "digitalocean_spaces_bucket" "treemap" {
  name   = "treemap"
  region = "fra1"
  acl    = "public-read"
}

resource "digitalocean_spaces_bucket_cors_configuration" "treemap" {
  region = digitalocean_spaces_bucket.treemap.region
  bucket = digitalocean_spaces_bucket.treemap.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "PUT", "POST", "HEAD"]
    allowed_origins = ["*"]
    max_age_seconds = 3000
  }
}

resource "digitalocean_cdn" "treemap" {
  origin = digitalocean_spaces_bucket.treemap.bucket_domain_name
}

resource "aws_s3_bucket_policy" "treemap" {
  provider = aws.spaces
  bucket   = digitalocean_spaces_bucket.treemap.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "arn:aws:s3:::treemap/*"
      }
    ]
  })
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

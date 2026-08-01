output "panoramas_cdn_url" {
  value       = "https://${digitalocean_cdn.panoramas.endpoint}"
  description = "The URL of the CDN endpoint for the panoramas bucket"
}

output "tiles_cdn_url" {
  value       = "https://${digitalocean_cdn.tiles.endpoint}"
  description = "The URL of the CDN endpoint for the tiles bucket"
}

output "treemap_cdn_url" {
  value       = "https://${digitalocean_cdn.treemap.endpoint}"
  description = "The URL of the CDN endpoint for the main treemap bucket"
}

output "ses_verification_token" {
  value       = aws_ses_domain_identity.main.verification_token
  description = "The verification token from the domain identity"
}

output "ses_dkim_tokens" {
  value       = aws_ses_domain_dkim.main.dkim_tokens
  description = "The DKIM tokens"
}

output "smtp_username" {
  value       = aws_iam_access_key.ses_smtp_user.id
  description = "The IAM access key ID"
}

output "smtp_password" {
  value       = aws_iam_access_key.ses_smtp_user.ses_smtp_password_v4
  description = "The ses_smtp_password_v4 from the access key"
  sensitive   = true
}

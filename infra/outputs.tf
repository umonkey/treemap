output "panoramas_cdn_url" {
  value       = "https://${digitalocean_cdn.panoramas.endpoint}"
  description = "The URL of the CDN endpoint for the panoramas bucket"
}

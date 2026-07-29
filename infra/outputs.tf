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

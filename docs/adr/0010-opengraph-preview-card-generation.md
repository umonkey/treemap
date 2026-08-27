# ADR-0010: OpenGraph Preview Card Generation

- Date: 2026-08-27
- Status: Accepted

## Context

Social media sharing and messaging platforms require rich OpenGraph preview cards for tree pages. Sharing tree URLs previously relied on static thumbnail images if available, lacking location maps and consistent branding. To improve social sharing engagement and ensure every tree has a high-quality visual preview, we need dynamic OpenGraph preview card generation on the backend.

## Decision

We will implement dynamic OpenGraph preview card generation for trees in the backend using `image`, `imageproc`, and `ab_glyph`.

This feature includes:

- endpoint: A dedicated endpoint at `/v1/trees/{id}/card.jpg` returning 1200x630 JPEG preview cards.
- layout: A minimalist layout featuring a left block for the tree photo (with fallback placeholder) and a right block for the MapTiler static map, along with header branding and footer species/location details.
- caching: Generated cards are cached on local disk under `var/cache/cards/` to ensure sub-5ms subsequent response times.
- meta tags: The `MetaService` automatically populates OpenGraph and Twitter image meta tags pointing to the card endpoint.

## Consequences

Positive:

- rich previews: Every tree page generates a professional, branded preview card with a photo and location map when shared on social media.
- performance: Disk caching ensures minimal CPU overhead and fast response times for repeated requests.
- reliability: Graceful fallbacks handle missing photos and map API errors.

Negative:

- external dependency: Relies on MapTiler Static Maps API and Google Fonts availability for font downloads on first initialization.

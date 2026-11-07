# File Storage

## Overview

Every photo in the platform is stored once and can be attached to a tree, a citizen feedback report, or a water source. The organisation gets a single, predictable place for images, stable links that can be cached and shared, and one moderation control instead of a separate pipeline per feature.

The main trade-off is that images are public. A stored image is served from the deployment's file host to anyone who has its link, and hiding an image removes it from the platform's responses but does not necessarily remove the file from the host. Deployments that need a hard takedown must remove the object from storage as an operational step.

## How it works

A person selects a photo in the app. The app asks the platform for an upload slot, sends the file directly to the deployment's object storage, and then tells the platform that the upload is complete. The platform records the file as awaiting upload, processes it in the background, and marks it available when the derived images exist. The original is kept.

Processing normalises every photo. The platform applies the camera's orientation, removes embedded metadata such as location, and produces two JPEG versions: a thumbnail for lists and map markers, and a display version for the photo viewer. The original is stored unchanged. Because metadata is stripped from the derived versions, a shared thumbnail or display image does not reveal where it was taken.

A file can be attached to a tree, a citizen feedback report, or a water source. Attachment is separate from upload: a person can upload while still filling in a form, and the file is linked when the record is saved. A tree can hold several photos and one of them is its thumbnail. A feedback report can hold up to ten photos. A water source can hold photos that document its condition.

The platform returns stable links for the original, the display version, and the thumbnail. The links do not expire, so the app, the map, and any shared page can cache them. Older links that address a file by its numeric id continue to work during the transition.

## Requirements and availability

File storage is available in the hosted service and in self-hosted deployments.

- Object storage: an S3-compatible bucket for originals and derived images. The local filesystem driver does not support direct browser uploads, so a self-hosted deployment must configure object storage.
- Public file host: a base address that serves the bucket, normally a CDN in front of it. The platform builds every image link from this address.
- Accounts and roles: uploading and attaching require a signed-in account. The role must allow the relevant action: adding tree photos, submitting a feedback report, or managing water sources.
- Devices: any browser with a file picker and camera access can upload. The upload uses a direct connection to object storage, so a stable network helps for large photos.
- Formats: the reliable formats are JPEG, PNG, and WebP. HEIC and HEIF are rejected because the server cannot decode them; some phones produce these by default and must be set to a compatible format.

## Cost and effort

Storage is the main recurring cost. Each photo keeps the original plus a thumbnail and a display JPEG, so budget roughly two to three times the size of the original set. A deployment that receives a few hundred photos a month should expect a few gigabytes a year. Delivery cost depends on the CDN in front of the bucket; thumbnails are small and are cached for a long time, so repeat views are cheap. Processing is background work and adds no meaningful compute. Staff effort is limited to moderation and, for a migration, one planned run of the backfill.

## Operations and limitations

- Hiding an image removes it from the platform's responses but does not delete the object. The link may still resolve until an operator removes the file from storage. Treat the visibility control as delisting, not as a guarantee of erasure.
- Processing is asynchronous. A newly uploaded photo can be attached before its derived images exist; the thumbnail and display versions appear within seconds, and a file that cannot be processed is flagged for attention rather than retried indefinitely.
- The original is retained. This allows re-processing and full-resolution display, but it also means storage cannot be reduced by discarding originals.
- There is no deduplication. Uploading the same photo twice stores it twice.
- Video is not processed in this release. The data model reserves a video kind and a poster image, but there is no transcoding, poster generation, or playback pipeline.
- The legacy numeric file links and the old photo tables remain during the transition. They are read-only compatibility paths and are removed in a later cleanup.
- A migration from the previous photo pipeline is additive and can run while the service is live. It copies objects server-side, is safe to re-run, and leaves the old data in place for rollback. The operator reference describes the exact sequence.

## Migration and rollback

The move from the previous photo pipeline is designed to avoid downtime. The new tables are added alongside the old ones, existing photos are copied to the new layout in the background, and the old links keep working while clients move to the new links.

The sequence is:

- Back up the database and record the current object count.
- Deploy the release that includes the migration runner and the new schema.
- Record the existing schema as applied, then apply the new schema.
- Run the photo backfill against a copy of the database and a staging bucket first, and compare counts and sample links.
- Run the backfill in production. It is idempotent and resumable, so an interrupted run can be repeated.
- Deploy the client that uses the new links.
- Verify that the number of tree-photo links matches the old photo count, that sample legacy links resolve, and that sample new links return the image.
- Freeze writes to the old photo path. Do not delete old objects or columns yet.

Rollback is a redeploy of the previous release. The old tables, columns, and objects are untouched until the cleanup is executed, so the previous version continues to work. The cleanup itself is a separate, later change and is not part of this migration.

The exact commands, configuration keys, and verification queries are in the [file upload operator reference](../../File-uploads.md).

## Questions organisations usually ask

- Are photos public? Yes. Stored images are served from the deployment's file host to anyone with the link. Hiding an image removes it from the platform's responses, but the link may still resolve until the object is removed from storage.
- Can we delete a photo? A user can remove their own file from the platform, and an operator can delist a file so it no longer appears in responses. Neither is a guaranteed erasure from the object store; a hard removal is a separate storage operation.
- Do we keep the original? Yes. The original is stored unchanged, and the platform derives a thumbnail and a display version from it. This costs more storage but allows re-processing and full-resolution viewing.
- What happens to location metadata? It is removed from the thumbnail and display versions. The original keeps its metadata, so treat the original as sensitive if the source photo carried location data.
- Can we use our own storage and CDN? Yes, in a self-hosted deployment. The platform needs an S3-compatible bucket and a public base address for the files. The hosted service uses the operator's storage.
- What does the migration cost? One backfill run that copies existing photos server-side. It runs while the service is live, can be repeated safely, and does not require a maintenance window. The main cost is the additional storage for the new layout until the old objects are cleaned up.

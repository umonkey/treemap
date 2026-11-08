# File Storage

## Overview

Every photo in the platform is stored once and can be attached to a tree, a citizen feedback report, or a water source. The organisation gets a single, predictable place for images, stable links that can be cached and shared, and one moderation control instead of a separate pipeline per feature.

The main trade-off is that images are public. A stored image is served from the deployment's file host to anyone who has its link, and hiding an image removes it from the platform's responses but does not necessarily remove the file from the host. Deployments that need a hard takedown must remove the object from storage as an operational step.

## How it works

A person selects a photo in the app. The app asks the platform for an upload slot, sends the file directly to the deployment's object storage, and then tells the platform that the upload is complete. The platform records the file as awaiting upload, processes it in the background, and marks it available when the derived images exist. The original is kept.

Processing normalises every photo. The platform applies the camera's orientation, removes embedded metadata such as location, and produces two JPEG versions: a thumbnail for lists and map markers, and a display version for the photo viewer. The original is stored unchanged. Because metadata is stripped from the derived versions, a shared thumbnail or display image does not reveal where it was taken.

A file can be attached to a tree, a citizen feedback report, or a water source. Attachment is separate from upload: a person can upload while still filling in a form, and the file is linked when the record is saved. A tree can hold several photos and one of them is its thumbnail. A feedback report can hold up to ten photos. A water source can hold photos that document its condition.

The platform returns stable links for the original, the display version, and the thumbnail. The links do not expire, so the app, the map, and any shared page can cache them. A link is built from the file's identifier, so it survives moderation and never needs refreshing.

## Requirements and availability

File storage is available in the hosted service and in self-hosted deployments.

- Object storage: an S3-compatible bucket for originals and derived images. The local filesystem driver does not support direct browser uploads, so a self-hosted deployment must configure object storage.
- Public file host: a base address that serves the bucket, normally a CDN in front of it. The platform builds every image link from this address.
- Accounts and roles: uploading and attaching require a signed-in account. The role must allow the relevant action: adding tree photos, submitting a feedback report, or managing water sources.
- Devices: any browser with a file picker and camera access can upload. The upload uses a direct connection to object storage, so a stable network helps for large photos.
- Formats: the reliable formats are JPEG, PNG, and WebP. HEIC and HEIF are rejected because the server cannot decode them; some phones produce these by default and must be set to a compatible format.

## Cost and effort

Storage is the main recurring cost. Each photo keeps the original plus a thumbnail and a display JPEG, so budget roughly two to three times the size of the original set. A deployment that receives a few hundred photos a month should expect a few gigabytes a year. Delivery cost depends on the CDN in front of the bucket; thumbnails are small and are cached for a long time, so repeat views are cheap. Processing is background work and adds no meaningful compute. Staff effort is limited to moderation and, for a deployment moving off the previous photo pipeline, one planned cleanup.

## Operations and limitations

- Hiding an image removes it from the platform's responses but does not delete the object. The link may still resolve until an operator removes the file from storage. Treat the visibility control as delisting, not as a guarantee of erasure.
- Processing is asynchronous. A newly uploaded photo can be attached before its derived images exist; the thumbnail and display versions appear within seconds, and a file that cannot be processed is flagged for attention rather than retried indefinitely.
- The original is retained. This allows re-processing and full-resolution display, but it also means storage cannot be reduced by discarding originals.
- There is no deduplication. Uploading the same photo twice stores it twice.
- Video is not processed in this release. The data model reserves a video kind and a poster image, but there is no transcoding, poster generation, or playback pipeline.
- The previous photo pipeline has been retired. The old numeric file links and the records behind them are gone, so a client cached from before the upgrade fails on those links until it reloads. This is a one-time break.
- Legacy Telegram alert photos are still served from their original store. They were never copied into this capability, so they are not managed here and remain a separate storage concern.
- Files copied from the previous pipeline carry placeholder size and dimension metadata. Storage reporting understates them until the inventory is refreshed.

## Moving from the previous photo pipeline

This release completes the move away from the previous photo pipeline. The copy itself was additive: the new storage was added alongside the old, photos were copied in the background while the service stayed live, and the old links kept working until clients moved to the new ones.

The cleanup is one-way. Once it has run, the previous release cannot be redeployed against the same database, and objects deleted from storage cannot be restored.

The sequence for an existing deployment is:

- Back up the database and record the current object count.
- Deploy the release that no longer reads the old tables and links.
- Export the old object keys before the schema changes, because the mapping that identifies them is dropped by the cleanup.
- Spot-check that a sample of the new links resolves, then delete the listed objects from storage by hand. Never delete by prefix.
- Apply the schema cleanup.
- Verify that the old storage records are gone and that legacy alert photos are unchanged.

Rollback is only possible from the database backup taken before the cleanup, and it does not restore the objects deleted from storage.

The design and its consequences are recorded in [ADR 0031](../../adr/0031-Universal-File-Storage.md).

## Questions organisations usually ask

- Are photos public? Yes. Stored images are served from the deployment's file host to anyone with the link. Hiding an image removes it from the platform's responses, but the link may still resolve until the object is removed from storage.
- Can we delete a photo? A user can remove their own file from the platform, and an operator can delist a file so it no longer appears in responses. Neither is a guaranteed erasure from the object store; a hard removal is a separate storage operation.
- Do we keep the original? Yes. The original is stored unchanged, and the platform derives a thumbnail and a display version from it. This costs more storage but allows re-processing and full-resolution viewing.
- What happens to location metadata? It is removed from the thumbnail and display versions. The original keeps its metadata, so treat the original as sensitive if the source photo carried location data.
- Can we use our own storage and CDN? Yes, in a self-hosted deployment. The platform needs an S3-compatible bucket and a public base address for the files. The hosted service uses the operator's storage.
- What does the move cost? The copy ran in the background while the service stayed live, so there was no downtime and no extra staff effort. The remaining work is one planned cleanup: take a backup, delete the old objects, and apply the schema change. The additional storage for the old objects ends when they are deleted.

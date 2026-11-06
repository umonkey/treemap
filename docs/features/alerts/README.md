# Citizen Feedback

## Overview

Citizen feedback lets people report what they notice on the ground — a new planting, a wildlife sighting, a damaged branch, or a maintenance need — and attaches each report to a point on the map. Signed-in users submit from the web map interface, and anyone who uses the deployment's Telegram bot can submit through a conversation. The organisation gets a public map layer of recent reports, a private notification stream for the staff who triage them, and a management view for reviewing the stored archive.

The main trade-off is visibility. Descriptions and photos are stored on the deployment's file host and are readable by anyone who has a link to them, so submissions should be treated as public. Reports are notification-only: the platform records and forwards them, but it has no assignment, case status, or resolution workflow.

## How it works

Web submission starts when a signed-in person positions the map, writes a description, attaches up to ten photos, and sends. Photos stay on the device until the person sends, then upload before the report is created. The report appears on the map immediately and is queued for curator notification.

Telegram submission uses a conversation instead: the person sends photos, a location, and a description over one or more messages. The report accumulates updates until it has all three, then locks; later messages start a new report. Incomplete conversations receive a reminder after ten minutes without activity.

After sending, a reporter can correct the text of their own web report, and curators whose role includes the alert management permission can correct any report.

Staff with the alert management permission can also review the archive. They browse a list of the most recent stored reports, open one to read its description, location, reporter identity, and photos, and correct the description when needed. Web reports are attributed to the submitting account. Telegram reports show the Telegram username instead, because the two identity systems are separate.

In both cases a completed report is delivered as a private notification to the curators configured for the deployment, with a link to the report preview. A report is marked as delivered only when every configured recipient receives it, and failed deliveries are retried automatically. Reports are retained permanently, but only those from the last seven days are rendered on the active map.

## Requirements and availability

Citizen feedback is available in the hosted service and in self-hosted deployments.

- Hosted service: a signed-in account to submit from the web. Browsing the map and its reports requires no account.
- Self-hosted deployment: a signed-in account whose role allows report submission. The deployment operator grants that permission through the role configuration.
- Management view: a role that includes the alert management permission. Administrator roles hold it by default, and a self-hosted operator can grant it to other roles. It is not required to submit a report.
- Telegram channel: a Telegram bot token and a list of curator recipients. A recipient must have started a conversation with the bot before notifications can be delivered.
- Devices: camera capture uses the browser's file picker, so a mobile browser with camera access is enough. Photos are held in the browser until the person sends, and uploading requires a network connection.

## Cost and effort

The capability adds negligible compute: the request only records metadata, and photo normalisation runs as background work. Storage grows with the number of photos; each is re-encoded as an oriented JPEG with a longest side of 2,000 pixels, typically a few hundred kilobytes. A deployment receiving a few hundred photo-bearing reports a month should budget a few gigabytes of storage a year. Telegram does not charge for bot messages, so the recurring staff cost is curator triage, a few minutes per incoming report. Archive review adds no meaningful compute: it reads stored metadata and photos, and its main cost is the staff time spent reviewing reports.

## Operations and limitations

- Curator notifications need a named team and a triage routine; the platform does not assign reports automatically.
- Only the description text can be changed after submission. Photos cannot be replaced or removed, and the platform keeps no edit history in this release.
- Photos are attached asynchronously, so a report preview can open before its photos are available. They usually appear within seconds, and refreshing the preview shows them.
- There is no per-user rate limit. Any account allowed to submit can create reports as fast as it can upload photos, so public deployments should monitor for abuse.
- Photos in HEIC or HEIF format from some phones may be rejected because the server cannot decode them. JPEG, PNG, and WebP are reliable.
- Reporting remains notification-only. Even with the management view, there is no assignment, status tracking, resolution workflow, or reply channel back to the reporter.
- Reports and photos are public on the deployment's file host. Do not use the channel for sensitive personal information.
- The management view shows the 100 most recent reports. Older reports remain stored and can still be reached from their original curator notification links, but they cannot be browsed in the interface.
- The map shows the last seven days. Older reports remain stored and are reachable from curator notifications, or from the management view while they are among the 100 most recent, but they are not rendered as markers.

## Questions organisations usually ask

- Who can submit a report? In the hosted service, any signed-in user. In a self-hosted deployment, any signed-in user whose role includes the submission permission. Telegram submission is open to anyone who can reach the bot.
- Are reports public or anonymous? Reports are public in the sense that their descriptions and photos are stored on the file host and can be read by anyone with the link. They are not anonymous: the platform records the submitting account, or the Telegram identity for bot submissions.
- What do curators receive? A private Telegram message with the report description and a link to its preview, including photos when they are ready. Delivery is retried until every configured recipient has received it.
- Can staff review reports after they leave the map? Yes. Staff whose role includes the alert management permission can browse the 100 most recent stored reports, open one for its description, location, reporter, and photos, and correct its description. Older reports remain stored but are not browsable in the interface.
- Does web submission replace Telegram? No. Both channels feed the same lifecycle, map layer, and curator notifications. A deployment can use either or both.
- What happens to photos? They are normalised to JPEG, stored with the report, and kept indefinitely with it. The original upload is kept as well, which uses additional storage.

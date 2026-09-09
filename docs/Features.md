# Application Features

## Keep screen awake

The application can prevent the device screen from automatically locking or dimming while the app is active. This is particularly useful for users mapping trees in the field who need constant access to the map without having to repeatedly unlock their device.

- Implementation: uses the Screen Wake Lock API.
- Requirements: requires a secure context (HTTPS) and a supported browser.
- Control: users can toggle this feature in the Settings page. It is enabled by default.
- Persistence: the preference is saved locally on the device.
- Visibility: the feature automatically re-acquires the lock when the application returns from the background.

## Location control

When activated by the user, the application tracks the device's location and displays a blue dot on the map. This visual indicator helps users more accurately position new trees during the mapping process.

- Implementation: uses the Geolocation API to monitor position updates.
- Privacy: the application does not store the user's location on any server and does not use the raw coordinates for any purpose other than map display.
- Controls: users can click the "Locate me" button to center the map on their current position.
- Visibility: a blue dot is rendered on the map to represent the user's real-time location.

## Background upload reminders

The application can notify the user when they have pending photo uploads and a stable WiFi connection is available. This ensures that photos collected offline are eventually uploaded without requiring the user to keep the application open.

- Implementation: uses the Background Sync API and Periodic Background Sync API.
- Requirements: requires a browser with support for these APIs (e.g., Chrome on Android) and user permission for notifications.
- Logic: the system checks the upload queue when connectivity changes or at regular intervals (approximately hourly). If the device is on WiFi and there are pending uploads, a system notification is shown.
- Interaction: tapping the notification opens the application's upload management page.

## App icon badging

The application displays the number of pending photo uploads as a badge (or "bubble") on the app icon when installed as a PWA. This provides a quick visual indicator of how much work is still pending without needing to open the app or check notifications.

- Implementation: uses the App Badging API (`navigator.setAppBadge`).
- Logic: the badge count is updated whenever the local upload queue changes. It is also updated in the background during sync events to remain accurate.
- Requirements: requires a supported browser (e.g., Chrome on Android/Desktop, Safari on iOS 16.4+).
- Reset: the badge is automatically cleared when the upload queue is empty.

## Citizen feedback via Telegram

Users can submit citizen feedback and remarkable sightings through a dedicated Telegram bot. These reports cover newly planted trees, wildlife sightings, health anomalies, and maintenance needs, providing real-time awareness of urban forest dynamics.

- implementation: a Telegram bot receives feedback reports and stores them in the central database.
- curator notifications: completed reports (containing photos, GPS location, and description) are automatically and promptly dispatched via private Telegram messages to a designated team of moderators/curators.
- visualization: feedback reports are rendered as markers on the map.
- retention: all reports are stored permanently in the database for historical analysis.
- display logic: only reports submitted within the last 7 days are displayed on the active map to ensure current relevance.

## Model Context Protocol (MCP) server

The application includes an integrated Model Context Protocol (MCP) server that exposes the tree database to AI agents through standardized tools.

- Implementation: uses the MCP specification over JSON-RPC.
- Tools: provides tools like `list_tallest`, `list_widest`, and `list_streets`.
- Data analysis: enables AI agents to perform complex queries and generate statistics about the tree population.
- Sorting: the `list_streets` tool supports custom sorting by name, count, or completeness.

## Street panoramas

The application integrates 360° street panoramas to support remote inventorying, automated computer-vision pipelines, and armchair mapping.

- Interactive 360 viewer: enables smooth exploration of equirectangular street imagery.
- Split-pane synchronization: displays the panorama and map side by side, keeping the viewer orientation and camera location synchronized in real time.
- Armchair mapping: supports remote auditing, tree cataloging, and asset tagging directly from desktop or mobile devices.
- Visual hints: displays overlays for mapped objects, such as trees and adjacent sequence nodes.
- Tree inventory integration: allows viewing existing tree profiles, placing new trees, or triangulating coordinates across multiple frames without standing under dense canopies.
- Automated trajectory alignment: aligns video frames with GPS tracks automatically using relative photogrammetry and robust similarity transformation without manual synchronization.
- Ray casting sightlines: projects viewer viewing direction onto the map to triangulate landmarks and verify alignments.
- Coordinate offsets: assists in fine-tuning spatial alignment between GPS tracks and imagery frames to match base map features.
- Detailed documentation: refers to `Panoramas.md` for full operational and technical guidelines.

## Role-Based Access Control (RBAC)

The application provides a secure environment for multiple types of users by managing access through a granular permission system. This ensures that sensitive operations are only accessible to authorized individuals while allowing regular users to contribute to the map.

- Granular permissions: controls specific actions such as editing street-level imagery or managing user accounts.
- Flexible roles: users can be assigned different roles (e.g., volunteer, editor, administrator) that define their capabilities within the system.
- Administrative oversight: provides tools for administrators to manage user access and maintain data quality across the platform.
- Security by design: access is restricted by default, ensuring that every sensitive operation requires explicit authorization.

## Tree search and data export

The application provides advanced search and filtering capabilities to help users query the tree population and export data for external research and analysis.

- Filtering: search trees by state, address, species, presence or absence of specific measurements, and data age such as recent photos.
- List view: view matching trees in a structured list with quick selection to jump directly to map locations.
- Data export: download the entire search result set as a CSV file for research, reporting, or external analysis.

## Trilateration

The application includes a trilateration tool that assists field arborists and volunteers in mapping trees located in obstructed environments where direct GPS positioning is impractical or inaccurate.

- Purpose: enables high-accuracy tree positioning via laser range finder measurements from visible reference points when direct GPS is inaccurate or unavailable.
- Reference landmarks: utilizes ground control points (GCPs) such as building corners or street infrastructure picked on an interactive map.
- Survey flexibility: allows changing or updating GCPs on the fly when surveying broad areas or long avenues.
- Batch creation: records multiple trees locally before batch-submitting default blank trees to the database.
- Detailed documentation: refers to `Trilateration-feature.md` for full operational and architectural guidelines.

## Water sources

The application maintains a registry of water access points (fountains, hydrants, irrigation points) to support planting and maintenance planning, identify dry areas with high risk of tree decline, and automate the selection of spots for civic planting. Water sources are rendered on the map with a 50 meter coverage disc and a clickable dot, track an `operational`, `dead`, or `gone` lifecycle status, and can be added or moved by users with the `water:manage` permission. See `Water-sources.md` for full operational and technical guidelines.

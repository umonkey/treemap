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

## Tree damage reports via Telegram

Users can report tree damage through a dedicated Telegram bot. These reports are integrated into the application's map, providing real-time awareness of tree health issues.

- Implementation: a Telegram bot receives reports and stores them in the central database.
- Visualization: damage reports are rendered as red circles on the map.
- Retention: all reports are stored permanently in the database for historical analysis.
- Display logic: only reports submitted within the last 7 days are displayed on the active map to ensure current relevance.

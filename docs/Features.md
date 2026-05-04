# Application Features

## Keep screen awake

The application can prevent the device screen from automatically locking or dimming while the app is active. This is particularly useful for users mapping trees in the field who need constant access to the map without having to repeatedly unlock their device.

- Implementation: uses the Screen Wake Lock API.
- Requirements: requires a secure context (HTTPS) and a supported browser.
- Control: users can toggle this feature in the Settings page. It is enabled by default.
- Persistence: the preference is saved locally on the device.
- Visibility: the feature automatically re-acquires the lock when the application returns from the background.

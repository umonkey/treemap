# TWA Publishing Guide

This guide describes how to build and publish the Trees of Yerevan application as a Trusted Web Activity (TWA) in the Google Play Store.

## Prerequisites

- Node.js and npm installed.
- The `release.keystore` file in `services/frontend/`.

## Configuration

The TWA configuration is stored in `services/frontend/twa-manifest.json`. This file contains:
- `packageId`: `app.treemaps.yerevan.twa`
- `host`: `yerevan.treemaps.app`
- `signingKey`: Points to `./release.keystore`.

## Building the App

To build the Android App Bundle (AAB):

1.  Navigate to the frontend directory:
    ```bash
    cd services/frontend
    ```

2.  Run the build command:
    ```bash
    make build-twa
    ```

This command runs `bubblewrap update` and `bubblewrap build`. It will generate an `.aab` file in the current directory (or a subdirectory created by Bubblewrap).

## Versioning

When publishing a new version to Google Play, you **must** increment both `appVersion` and `appVersionCode` in `twa-manifest.json`.
- `appVersion`: The user-visible version string.
- `appVersionCode`: An integer that must be higher than the previous version.

## Digital Asset Links

For the TWA to work without a browser URL bar, the `assetlinks.json` file must be served at `https://yerevan.treemaps.app/.well-known/assetlinks.json`.
The SHA-256 fingerprint in `assetlinks.json` must match the one in `twa-manifest.json` and the one used to sign the APK/AAB.

To view the fingerprint of the keystore:
```bash
keytool -list -v -keystore release.keystore
```

## Signing

The app is signed using `release.keystore`. **Keep this file safe and do not lose the password.** If lost, you will not be able to update the app in the Play Store.

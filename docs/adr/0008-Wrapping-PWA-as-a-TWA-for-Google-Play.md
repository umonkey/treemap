# ADR-0008: Wrapping PWA as a Trusted Web Activity for Google Play

- Date: 2026-04-13
- Status: Accepted

## Context

The application is currently a Progressive Web App (PWA) primarily used on Android devices. To improve discoverability and provide a more native feel, we want to publish it on the Google Play Store.

There are several ways to wrap a web application for Android, including Trusted Web Activity (TWA) and Capacitor. TWA is preferred for the initial release because it is lightweight, maintains a single codebase for updates, and leverages the existing PWA capabilities.

The production domain is `yerevan.treemaps.app`.

## Decision

We will use the Trusted Web Activity (TWA) approach to publish the application to Google Play.

- Package name: we will use `app.treemaps.yerevan` as the application ID, following the reverse domain name convention.
- Tools: we will use the `bubblewrap` CLI tool to generate and manage the Android project.
- Distribution: the app will initially be published to an Internal Testing track in the Google Play Console.

## Consequences

Positive:

- Discoverability: the app will be available in the Google Play Store.
- User experience: the app will appear in the app drawer and task switcher like a native app.
- Maintenance: updates to the frontend are automatically reflected in the TWA without requiring new store submissions.

Negative:

- Browser dependency: the TWA depends on the presence of a compatible browser (like Chrome) on the device.
- Storage isolation: switching to a native bridge like Capacitor in the future may result in the loss of local storage data due to origin changes.

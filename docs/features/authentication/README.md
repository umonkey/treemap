# Authentication

## Overview

Authentication lets people sign in with their Google account so they can change the map. Anonymous visitors can browse; adding a tree, commenting, uploading a photo, or opening an administrative tool requires an account. Accounts are created on first sign-in, and each person's permissions determine what they can change.

Every change is attributed to an account, so staff can review the history and correct or remove an edit, and can take roles away from an account that misbehaves. No passwords are stored; identity and multi-factor authentication are handled by Google.

## How it works

When someone tries to add a tree, comment, upload a photo, or open an administrative tool, the app asks them to sign in. Google authenticates them and returns them to the deployment. The backend verifies the Google response and confirms that the site's hostname belongs to an enabled tenant. On first sign-in it creates the person's account, assigns the standard contributor role, and issues a signed session valid for 30 days. The app stores that session, loads the person's roles and permissions, and returns them to the action they started.

Every later request carries the session. The backend resolves the person and checks the permission the action requires, and the interface hides actions the person cannot perform. Permissions are read on each request, so a role change takes effect immediately.

People can change their display name and profile picture in their profile; the Google copy is only the starting point. Signing out deletes the browser copy of the session. After 30 days, or if the session is rejected, the app clears it and asks the person to sign in again.

## Requirements

Authentication is available in the hosted service and in self-hosted deployments.

- A Google account. Any Google account can sign in, and a new account immediately receives the standard contributor role, which allows adding trees and comments.
- A standard browser. Google blocks its sign-in flow inside embedded in-app browsers.
- Hosted service: the site's hostname must be a registered, enabled tenant. Each community or municipality has its own. The platform operator adds it during onboarding.
- Self-hosted deployment: the operator registers a Google sign-in client for the deployment, allows the callback address, sets a signing secret, builds the frontend with the public client identifier, registers the tenant hostname, and uses HTTPS in production.
- No cookies are involved. The session is kept in browser storage.

## Limitations

- Sessions last 30 days and cannot be refreshed. Expiry means signing in again.
- There is no revocation and no list of signed-in devices. A copied session remains valid until it expires, and signing out affects only the browser where it is done.
- The session is stored in browser storage, so it is exposed to cross-site scripting. An HttpOnly cookie is not used because the frontend is served separately from the API.
- Google only. There is no email and password, Microsoft, SAML, or LDAP sign-in, and multi-factor authentication is whatever the person's Google account enforces.
- There is no allowlist by email domain. Any Google account can sign in and contribute as a standard user.
- Accounts and roles are platform-wide. A person who is an administrator on one tenant is an administrator on every tenant served by the same deployment, and the tenant in the session is checked at sign-in but not on every request.
- Role assignment is not exposed in the administration interface. The user screen lists roles and permissions and edits the display name only; operators change roles in the database.
- There is no account deletion or disable. Removing a person's roles stops privileged actions but does not stop them from signing in.
- Accounts are matched by email address. Changing the primary email on a Google account creates a new account rather than linking the old one.

## Cost and effort

Authentication adds no infrastructure; sessions are stateless. Google does not charge for sign-in. A hosted tenant has no setup beyond the registered hostname. A self-hosted operator spends roughly half a day registering the client, setting the signing secret, building the frontend, and testing sign-in. Ongoing effort is periodic access review.

## Questions organisations usually ask

- Can we restrict sign-in to our staff? Not currently. Any Google account can sign in and receives the standard contributor role. Elevated roles are granted by the operator.
- Can staff use Microsoft or email and password accounts? No. Google is the only provider.
- What happens when someone leaves? Remove their roles. Their session continues as a read-only visitor until it expires, and sign-in itself cannot be disabled.
- Can a person delete their account? Not through the application. The operator has to handle the request.

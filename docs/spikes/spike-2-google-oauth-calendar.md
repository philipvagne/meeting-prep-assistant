# Spike 2 — Google OAuth & Calendar

## Purpose

Validate whether Meeting Prep Assistant can securely authenticate with a user's Google account using OAuth 2.0 and retrieve calendar information while respecting the project's security principles.

This spike focuses only on authentication, secure token handling, and basic calendar access. It does not implement meeting preparation, Gmail, Google Drive, or AI.

---

## Engineering Question

Can a Windows desktop application securely authenticate with Google using OAuth 2.0, securely persist authentication, and retrieve upcoming calendar events without compromising user privacy?

---

## Hypothesis

A Tauri desktop application can securely authenticate using Google's Desktop OAuth flow, store authentication securely using operating-system-backed storage, and access Google Calendar through read-only permissions.

If this spike succeeds, the project can continue toward context collection.

---

## Scope

Validate:

* Google OAuth 2.0 Desktop Flow
* System browser authentication
* Local OAuth callback handling
* Authorization code exchange
* Secure refresh token storage
* Access token refresh
* Calendar read-only permissions
* Retrieve upcoming calendar events
* Logout / disconnect path
* Authentication persistence across application restart

---

## Out of Scope

Do NOT implement:

* Gmail integration
* Google Drive integration
* AI provider integration
* Meeting brief generation
* Background meeting monitoring
* Notification scheduling
* Final UI design
* User settings
* Local database
* Microsoft account support

---

## Security Requirements

The spike only passes if:

* Google authentication uses OAuth 2.0.
* The application never requests the user's password.
* Only the minimum required Google scopes are requested.
* Refresh tokens are never stored as plain text.
* Tokens are stored using secure operating-system-backed storage whenever possible.
* Calendar access is read-only.
* No Google data is stored on developer-controlled servers.

---

## Success Criteria

This spike passes if:

* The system browser opens for authentication.
* Google authentication completes successfully.
* OAuth callback is received correctly.
* Tokens are exchanged successfully.
* Refresh token is securely stored.
* Authentication survives application restart.
* The application can retrieve upcoming calendar events.
* Calendar permissions are read-only.
* Logout/disconnect clears local authentication correctly.

---

## Implementation Approach

* Use Google's desktop OAuth authorization-code flow in the system browser.
* Use a loopback callback on `127.0.0.1` so the desktop app receives the authorization code directly.
* Use PKCE for the desktop authorization flow.
* Request only `https://www.googleapis.com/auth/calendar.readonly`.
* Persist the Google desktop client ID and client secret as local-only app configuration.
* Persist tokens in a DPAPI-encrypted local file tied to the current Windows user profile.
* Refresh access tokens locally when needed.
* Fetch only minimal event metadata needed for validation.

---

## Result

PASS.

---

## Lessons Learned

* Google Calendar read-only access fits the local-first architecture cleanly when the app owns the OAuth callback and token exchange locally.
* The smallest secure validation path is browser-based desktop OAuth with PKCE and a loopback redirect.
* A Windows-only spike can use DPAPI-backed local encryption without introducing any developer-controlled backend.
* The app can keep the React layer thin by handling OAuth, token storage, refresh, and Google requests in the Tauri backend.
* Some Google testing projects still require the desktop client secret during token exchange, so the validation app must support local-only client ID and client secret configuration.

---

## Limitations

* This spike currently requires the user to supply their own Google desktop OAuth client ID and client secret for manual validation.
* Token storage uses a DPAPI-encrypted local file rather than Windows Credential Manager. This still avoids plain-text storage and keeps data tied to the local Windows user profile, but it should be revisited during implementation if a stronger OS-native secret store is preferred.
* Disconnect clears the app's local authentication state only. It does not revoke the Google refresh token remotely.
* The Google desktop client secret is stored locally in app configuration rather than inside DPAPI-protected token storage. This keeps it local-only and out of the repository, but it should still be treated as sensitive local configuration.

---

## Manual Validation Flow

1. Create a Google Cloud Desktop OAuth client.
2. Add the test user in Google Cloud while the app is in testing mode.
3. Configure the desktop client ID and client secret in the validation app.
4. Start the browser authentication flow from the app.
5. Complete Google authentication in the default browser.
6. Confirm the localhost callback returns successfully and the desktop app updates its authentication status.
7. Fetch upcoming calendar events successfully.
8. Fully close and restart the desktop application.
9. Confirm authentication persists after restart and the refresh token remains available.

---

## Notable Observations

* Google displays the expected unverified-app warning while the project remains in testing mode.
* Browser authentication completes successfully through the localhost callback path.
* Secure token storage works as intended for refresh token persistence across restarts.
* The validation UI displays only minimal calendar metadata.

---

## Validation Outcome

* Google OAuth flow opens in system browser: PASS
* User can authenticate successfully: PASS
* Only Calendar read-only scope is requested: PASS
* Tokens are stored securely, not as plain text: PASS
* Authentication persists across restart: PASS
* Upcoming calendar events can be retrieved: PASS
* Disconnect clears authentication: PASS
* No Gmail or Drive permissions are requested: PASS
* No Google write permissions are requested: PASS

---

## Final Status

COMPLETE - PASS.

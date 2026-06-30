# Meeting Prep Assistant

Spike 2 has been successfully validated. The current validation app demonstrates Google OAuth 2.0 desktop authentication and Google Calendar read-only access inside the Windows Tauri shell.

## Commands

```bash
npm install
npm run tauri dev
npm run build
```

## Spike 2 Setup

Before testing Google authentication, create a Google OAuth desktop client in Google Cloud, enable the Google Calendar API, and paste the desktop client ID and desktop client secret into the Spike 2 validation UI.

This spike requests only:

- `https://www.googleapis.com/auth/calendar.readonly`

It does not request Gmail, Drive, profile, or write permissions.

The client secret is treated as local-only configuration and must never be committed to the repository. Refresh tokens remain stored separately in DPAPI-protected local storage.

## Spike 2 Validation Status

Spike 2 is COMPLETE and assessed as PASS.

Manual validation successfully demonstrated:

- Google Desktop OAuth 2.0 authentication
- Default-browser OAuth launch
- Successful localhost callback handling
- Successful access-token and refresh-token exchange
- Secure Windows-backed refresh token persistence
- Authentication persistence across full app restart
- Google Calendar read-only event retrieval
- Disconnect availability after authentication
- No Gmail, Drive, profile, or write scopes

## Spike 2 Scope

This build intentionally covers only:

- Google OAuth 2.0 desktop flow
- System browser authentication
- Local loopback callback handling
- Secure local token persistence
- Google Calendar read-only validation
- Authentication persistence across restart
- Disconnect / local sign-out validation

It intentionally does not include Gmail, Drive, AI generation, meeting detection, settings, or production UI polish.

# Meeting Prep Assistant

Spike 2 has been successfully validated. The current validation app now focuses on Spike 3 and demonstrates Google OAuth 2.0 desktop authentication plus Gmail and Google Drive read-only context collection inside the Windows Tauri shell.

## Commands

```bash
npm install
npm run tauri dev
npm run build
```

## Spike 3 Setup

Before testing Google authentication, create a Google OAuth desktop client in Google Cloud, enable the Google Calendar API, Gmail API, and Google Drive API, and paste the desktop client ID and desktop client secret into the Spike 3 validation UI.

This spike requests only:

- `https://www.googleapis.com/auth/calendar.readonly`
- `https://www.googleapis.com/auth/gmail.readonly`
- `https://www.googleapis.com/auth/drive.readonly`

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

## Spike 3 Validation Status

Spike 3 has reached the milestone `Core functionality validated`.

Manual validation successfully demonstrated:

- Google OAuth 2.0 authentication in the desktop app
- Windows-backed secure refresh-token storage
- Restored authenticated state after full application restart
- Calendar, Gmail, and Drive read-only scopes only
- Upcoming Google Calendar event retrieval
- Using one selected Calendar event as the context seed
- Related Gmail message retrieval
- Related Google Drive document retrieval
- Aggregated Calendar, Gmail, and Drive context shown inside the desktop app

Validation used realistic but synthetic project data, including synthetic meetings, matching Gmail messages, and matching Google Drive documents. Related context was successfully collected for `Project Alpha Sync` and `Architecture Review`.

Known limitation:

Gmail matching currently relies primarily on meeting-title matching. For example, the meeting `Weekly Architecture Review` did not discover emails titled `Architecture Review - database`, but after renaming the meeting to `Architecture Review`, Gmail results were found immediately. This is being tracked as a search-strategy limitation, not an API limitation.

Future iterations should improve relevance through:

- title normalization
- keyword extraction
- multiple search variants
- semantic matching (future)

## Spike 3 Scope

This build intentionally covers only:

- Google OAuth 2.0 desktop flow
- System browser authentication
- Secure local token persistence
- Google Calendar read-only retrieval
- Gmail read-only search validation
- Google Drive read-only search validation
- Event-seeded context collection
- Disconnect / local sign-out validation

It intentionally does not include AI generation, meeting detection, settings, or production UI polish.

## Spike 3 Security

This spike:

- never requests write permissions
- never modifies Google data
- uses Calendar, Gmail, and Drive read-only scopes only
- performs all processing locally

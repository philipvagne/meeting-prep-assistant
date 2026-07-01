# Meeting Prep Assistant

The Validation Phase is complete. The project is now in `Implementation Phase - Phase 1: Desktop Application`.

Phase 1 focuses on desktop product structure and user experience:

- application shell
- navigation and search shell
- Home / generated briefs inbox
- Upcoming Meetings
- Settings foundation
- separate brief window
- notes interaction
- window management and tray flow

Phase 1 is guided by:

- `docs/roadmap.md`
- `docs/research/product-specification-v1.md`
- `docs/research/product-experience.md`

Early wireframes and Figma drafts are being used as structure references, not final visual design.

## Commands

```bash
npm install
npm run tauri dev
npm run build
```

## Current Status

- Validation Phase complete
- Spike 1: PASS WITH LIMITATIONS
- Spike 2: PASS
- Spike 3: Core functionality validated
- Spike 4: PASS WITH LIMITATIONS
- Active implementation phase: `Phase 1 - Desktop Application`

## Spike 4 Setup

Before testing Google authentication, create a Google OAuth desktop client in Google Cloud, enable the Google Calendar API, Gmail API, and Google Drive API, and paste the desktop client ID and desktop client secret into the validation UI.

To validate brief generation, provide a local AI provider configuration in the Spike 4 validation UI. The validation build currently supports:

- OpenAI
- Google Gemini

Each provider uses its own locally stored API key and model setting. Provider API keys are stored locally only and are not committed to the repository.
The default Gemini validation model is `gemini-2.5-flash`.

This spike requests only:

- `https://www.googleapis.com/auth/calendar.readonly`
- `https://www.googleapis.com/auth/gmail.readonly`
- `https://www.googleapis.com/auth/drive.readonly`

It does not request profile or write permissions.

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

## Spike 4 Validation Status

Spike 4 has now been validated and is assessed as PASS WITH LIMITATIONS.

Manual validation successfully demonstrated:

- AI provider abstraction
- Google Gemini brief generation
- OpenAI provider path remains available
- local-only provider API key storage
- manual brief generation from already-collected context
- Calendar, Gmail, and Drive context passed into the AI layer
- source references in generated briefs
- generated brief display in the app
- native Windows notification when a brief is ready

Current limitations:

- brief format and content quality are not final
- prompt design needs future iteration
- generated markdown still needs future UI rendering and presentation work
- tray/app restore remains the supported way to return to the brief

## Spike 4 Scope

This build intentionally covers only:

- Google OAuth 2.0 desktop flow
- Gmail and Drive read-only context collection
- local AI provider configuration
- provider abstraction for OpenAI and Gemini
- manual brief generation from selected-event context
- source-backed brief presentation
- brief-ready native notification
- disconnect / local sign-out validation

It intentionally does not include automatic scheduling, recurring reminders, semantic search, settings, or production UI polish.

## Spike 4 Security

This spike:

- never requests write permissions
- never modifies Google data
- uses Calendar, Gmail, and Drive read-only scopes only
- performs all processing locally
- stores provider API keys locally only
- sends only selected-event validation context to the AI provider

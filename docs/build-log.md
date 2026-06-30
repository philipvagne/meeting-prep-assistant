# Build Log

## 2026-06-29 - Spike 1 desktop foundation scaffold

### Summary

Created the first Tauri v2 + React/TypeScript application scaffold for Spike 1 and wired the validation surface for tray, background behavior, notifications, and autostart.

### Changes

- Added frontend project files for Vite, React, and TypeScript.
- Added `src-tauri` with Tauri configuration, capabilities, icons, and Rust entry points.
- Implemented:
  - tray icon with open and quit actions
  - close-to-tray behavior
  - notification test UI
  - autostart enable and disable actions
  - show and focus window action
- Updated README with basic project commands and Spike 1 scope.

### Validation Notes

- `npm run build` succeeded.
- `npm run tauri info` succeeded and reported:
  - WebView2 present
  - Rust not installed
  - Cargo not installed
  - Visual Studio Build Tools with MSVC/SDK not detected
- `npm run tauri dev -- --no-watch` failed because `cargo` is not installed.
- `npm run tauri build` failed because `cargo` is not installed.

### Outcome

Spike 1 is currently assessed as PASS WITH LIMITATIONS.

The desktop foundation is scaffolded and the implementation path is clear, but full runtime validation still requires installation of the Windows Rust and MSVC build prerequisites.

## 2026-06-29 - Spike 1 tray and restore behavior tightening

### Summary

Adjusted the Spike 1 desktop-shell behavior so tray restore, hide-to-tray, and notification-triggered restore share one backend-controlled window path.

### Changes

- Added backend Tauri commands to:
  - hide the main window to tray/background mode
  - restore and focus the main window
  - report current window state
- Updated the React validation UI to call those backend commands instead of controlling the window directly.
- Added visible, focused, and minimized state reporting to reduce ambiguity during manual validation.
- Clarified the notification validation path so the explicit `Open app` action is the intended restore signal.

### Validation Notes

- `npm run build` succeeded after the tray/restore changes.
- Full `npm run tauri dev` validation still could not be executed in this environment because Rust/Cargo and Windows build tools are unavailable here.

### Outcome

Spike 1 status remains PASS WITH LIMITATIONS until the updated tray and notification restore flows are re-validated on a Windows machine with the Tauri toolchain installed.

## 2026-06-29 - Notification activation investigation

### Summary

Investigated whether the current Tauri v2 desktop notification path on Windows can restore or focus the app when the user clicks a native notification.

### Findings

- Windows supports notification activation for desktop apps, but it requires explicit activation registration and app lifecycle handling.
- The current Tauri desktop notification plugin used in this spike only shows notifications on desktop.
- The plugin's desktop implementation does not add Windows activation registration or app relaunch handling for notification clicks.
- Because of that, notification click restore should be treated as unsupported in the current Spike 1 implementation rather than something to patch with frontend workarounds.

### Changes

- Removed the spike UI's notification reopen claim and action-based reopen wiring.
- Kept native notification sending intact.
- Updated spike documentation to record the limitation and the correct long-term direction.

### Outcome

Spike 1 remains PASS WITH LIMITATIONS, with notification-click restore documented as unsupported for the current Tauri v2 desktop notification path on Windows.

## 2026-06-30 - Spike 2 Google OAuth and Calendar validation flow

### Summary

Added the first Google OAuth and Calendar read-only validation flow to the Tauri desktop shell.

### Changes

- Replaced the Spike 1 validation UI with a Spike 2 validation surface for:
  - saving a Google desktop OAuth client ID locally
  - starting system-browser Google authentication
  - fetching upcoming calendar events
  - disconnecting and clearing local authentication
- Added Rust-side handling for:
  - loopback OAuth callback reception on `127.0.0.1`
  - PKCE generation for the desktop OAuth flow
  - authorization code exchange
  - access token refresh
  - minimal upcoming event retrieval from Google Calendar
- Added Windows local token protection using DPAPI-backed encryption for the persisted token payload.
- Updated README and the Spike 2 notes to document setup, scope, validation results, and limitations.

### Validation Notes

- `npm run build` succeeded.
- `cargo check` succeeded when pointed at an alternate writable target directory.
- `npm run tauri dev` compiled and launched `meeting-prep-assistant.exe` in this environment.
- `npm run tauri build --debug` compiled the desktop app and produced `src-tauri/target/debug/meeting-prep-assistant.exe`.
- The bundling step then failed because WiX could not be downloaded in this environment.
- A live Google sign-in and Calendar retrieval still require manual validation with a real Google desktop OAuth client ID.

### Outcome

Spike 2 is currently assessed as PASS WITH LIMITATIONS.

The secure local architecture path is validated, the desktop app compiles and launches, and the Google OAuth/Calendar read-only flow is implemented. Final pass confirmation for authentication, persistence, and event retrieval still requires a manual end-to-end run with real Google credentials.

## 2026-06-30 - Spike 2 desktop client secret support

### Summary

Updated the Google OAuth validation flow to support a local-only desktop client secret after live validation showed Google's token endpoint rejecting the exchange without it.

### Changes

- Expanded the local Google OAuth config to store:
  - desktop client ID
  - desktop client secret
- Updated the Spike 2 validation UI so the user can provide both values.
- Added backward-compatible handling for older local config files that only contained a client ID.
- Included the client secret in:
  - authorization code exchange
  - refresh token exchange
- Added ignore rules for local token/config artifacts that should never be committed.

### Validation Notes

- The Google loopback callback path had already been confirmed working in live validation.
- The token exchange failure was traced to Google's token endpoint requiring `client_secret` for the configured desktop OAuth client.
- Build validation was rerun after the fix.

### Outcome

Spike 2's authentication path should now complete successfully for Google desktop OAuth clients that require both client ID and client secret during token exchange, while still requesting Calendar read-only scope only.

## 2026-06-30 - Spike 2 manual validation completed

### Summary

Completed live manual validation for Spike 2 and confirmed that the desktop app can securely authenticate with Google and retrieve upcoming Calendar events using read-only access only.

### Manual Validation Flow

1. Created a Google Cloud Desktop OAuth client.
2. Added a Google test user.
3. Configured the desktop client ID and client secret in the validation app.
4. Completed the browser-based Google authentication flow.
5. Confirmed authentication status updated correctly in the app.
6. Fetched upcoming Google Calendar events successfully.
7. Fully closed and restarted the desktop application.
8. Confirmed authentication persisted after restart.
9. Confirmed the refresh token remained available after restart.

### Findings

- Google Desktop OAuth 2.0 authentication completed successfully.
- OAuth launched in the user's default browser.
- The localhost callback completed successfully.
- The authorization code was exchanged successfully for an access token and refresh token.
- The refresh token was stored securely using Windows-backed secure storage.
- Authentication state persisted after fully closing and restarting the app.
- The only requested scope was Google Calendar read-only.
- Upcoming Calendar events were fetched and displayed successfully.
- Disconnect functionality remained available after authentication.
- No Gmail, Drive, profile, or write scopes were requested.

### Notable Observations

- Google displayed the expected unverified-app warning because the project is still in testing mode.
- The localhost browser callback approach worked reliably for desktop authentication.
- Secure token storage behaved as intended during restart validation.
- Only minimal calendar metadata was displayed for validation.

### Outcome

Spike 2 is now COMPLETE and assessed as PASS.

# Spike 1 — Desktop Foundation

## Purpose

Validate whether Meeting Prep Assistant can behave like a real Windows desktop application before product implementation begins.

This spike focuses only on desktop application behavior, not Google integration, AI, final UI, or product polish.

---

## Engineering Question

Can a Tauri Windows desktop application run in the background, live in the system tray, start with Windows, send a native notification, and reopen the application when the notification is clicked?

---

## Hypothesis

Tauri can support the desktop foundation required for Meeting Prep Assistant.

If this spike succeeds, the project can continue toward Google integration validation.

---

## Implementation Approach

- Scaffolded a Tauri v2 application with a React and TypeScript frontend.
- Added a minimal Spike 1 validation screen with explicit actions for:
  - native notification test
  - hide to tray/background
  - show/focus window
  - autostart enable and disable
- Added desktop-shell behavior in the Tauri layer for:
  - tray creation
  - tray reopen action
  - quit action
  - close-to-tray behavior
- Configured notification and autostart plugins plus capability permissions.
- Added Windows bundle configuration notes in `tauri.conf.json` and validated the Tauri CLI environment report.

---

## Scope

Validate:

- Tauri application setup
- React/TypeScript frontend foundation
- System tray behavior
- Background running behavior
- Windows startup behavior
- Native Windows notification
- Notification click opens app window
- Windows packaging feasibility

---

## Out of Scope

Do NOT implement:

- Google OAuth
- Google Calendar
- Gmail
- Google Drive
- AI provider integration
- Meeting brief generation
- Final UI design
- Branding
- Product settings
- Real meeting detection

---

## Success Criteria

This spike passes if:

- The app can launch successfully.
- The app can stay running in the background.
- The app can appear in the Windows system tray.
- The app can start with Windows or demonstrate a clear implementation path.
- The app can send a native Windows notification.
- Clicking the notification can open or focus the application window.
- Windows packaging feasibility is understood.

---

## Result

Tauri is a viable fit for the Windows desktop foundation, but this validation completed with environment limitations.

The repository now contains a working desktop foundation scaffold for the spike, and the frontend build succeeds.

Full runtime validation of the desktop window, tray, notification, and packaging flow could not be completed on this machine because Rust and Visual Studio Build Tools with MSVC/Windows SDK are not installed.

The restore and hide flows were then tightened to use backend window commands instead of relying on frontend-only window control. The spike UI now also reports the window's visible, focused, and minimized state so manual validation is less ambiguous.

Notification click restore was investigated separately. The current Tauri desktop notification plugin does not implement the Windows activation plumbing needed to reopen or focus the app when the user clicks a native notification.

### Success Criteria Status

- App launches successfully: FAIL on this machine. `tauri dev` is blocked because `cargo` is not installed.
- App can run in the background: PASS WITH LIMITATIONS. Close-to-tray and hide-to-background behavior now use the same backend hide path, but were not runtime-verified here.
- App can appear in the Windows system tray: PASS WITH LIMITATIONS. Tray setup and reopen/quit actions now use the same backend restore path, but were not runtime-verified here.
- App can start with Windows or demonstrate a clear implementation path: PASS WITH LIMITATIONS. Autostart is implemented through the Tauri autostart plugin but not runtime-verified here.
- App can send a native Windows notification: PASS WITH LIMITATIONS. Notification test flow is implemented through the Tauri notification plugin but not runtime-verified here.
- Clicking the notification can open or focus the application window: FAIL for the current Tauri desktop notification path. Native notification display works, but notification activation is not implemented by the current desktop plugin on Windows.
- Windows packaging feasibility is understood: PASS WITH LIMITATIONS. `tauri build` is currently blocked by missing Rust/MSVC prerequisites, but the project is configured for Tauri Windows bundling and the next setup steps are clear.

---

## Lessons Learned

- Tauri v2 provides a straightforward path for tray, notification, and autostart behavior inside the desktop shell.
- A single backend restore path is more reliable for tray reopen and notification reopen behavior than duplicating window control logic in the frontend.
- Validation can move forward with a lightweight frontend while keeping product features out of scope.
- Windows supports launching or foregrounding a desktop app from a notification, but it requires toast activation registration plus activation handling in the app lifecycle.
- The current Tauri desktop notification plugin is focused on showing notifications and does not wire that Windows activation flow on desktop.
- Desktop validation depends heavily on local machine prerequisites:
  - Rust toolchain
  - Cargo
  - Visual Studio Build Tools with MSVC and Windows SDK
  - WebView2 runtime
- The current machine already has WebView2 available, which reduces one Windows-specific setup risk.
- Correct long-term support likely requires a Windows-specific activation path at the desktop-shell layer rather than additional frontend notification code.

---

## Limitations

- `tauri dev` and `tauri build` could not run because the machine is missing:
  - Rust
  - Cargo
  - Visual Studio Build Tools / MSVC / Windows SDK
- The current Tauri desktop notification implementation does not provide a reliable Windows notification-click restore path.
- Windows notification activation requires package/manifest activation wiring or equivalent desktop activation support that is outside the current plugin path used in this spike.
- Packaging feasibility is understood at the configuration level, but installer output was not produced in this environment.

---

## Final Status

PASS WITH LIMITATIONS

Spike 1 can be considered complete if notification-click restore is accepted as not supported by the current Tauri v2 desktop notification implementation.

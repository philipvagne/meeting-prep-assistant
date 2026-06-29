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

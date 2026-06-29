import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

const appWindow = getCurrentWindow();

type NotificationPermissionState = "unknown" | "granted" | "denied";
type WindowState = {
  visible: boolean;
  focused: boolean;
  minimized: boolean;
};

function App() {
  const [autostartEnabled, setAutostartEnabled] = useState<boolean | null>(null);
  const [notificationPermission, setNotificationPermission] =
    useState<NotificationPermissionState>("unknown");
  const [windowState, setWindowState] = useState<WindowState | null>(null);
  const [statusMessage, setStatusMessage] = useState(
    "Desktop foundation ready for validation.",
  );

  async function syncWindowState() {
    const nextState = await invoke<WindowState>("get_main_window_state");
    setWindowState(nextState);
    return nextState;
  }

  async function revealWindow() {
    const nextState = await invoke<WindowState>("restore_main_window");
    setWindowState(nextState);
    setStatusMessage(
      `Main window restore requested. Visible=${nextState.visible}, Focused=${nextState.focused}, Minimized=${nextState.minimized}.`,
    );
  }

  async function refreshStatus() {
    setAutostartEnabled(await isEnabled());
    setNotificationPermission(
      (await isPermissionGranted()) ? "granted" : "unknown",
    );
    await syncWindowState();
  }

  async function ensureNotificationPermission() {
    if (await isPermissionGranted()) {
      setNotificationPermission("granted");
      return true;
    }

    const permission = await requestPermission();
    const granted = permission === "granted";
    setNotificationPermission(granted ? "granted" : "denied");
    return granted;
  }

  async function sendValidationNotification() {
    const granted = await ensureNotificationPermission();
    if (!granted) {
      setStatusMessage("Notification permission was not granted.");
      return;
    }

    sendNotification({
      title: "Meeting Prep Assistant",
      body: "Spike 1 notification test.",
      autoCancel: true,
      extra: {
        source: "spike-1",
      },
    });
    setStatusMessage("Native notification sent.");
  }

  async function toggleAutostart() {
    if (autostartEnabled) {
      await disable();
      setStatusMessage("Autostart disabled.");
    } else {
      await enable();
      setStatusMessage("Autostart enabled.");
    }

    setAutostartEnabled(await isEnabled());
  }

  async function hideToTray() {
    const nextState = await invoke<WindowState>("hide_main_window");
    setWindowState(nextState);
    setStatusMessage(
      `Hide to tray requested. Visible=${nextState.visible}, Focused=${nextState.focused}, Minimized=${nextState.minimized}.`,
    );
  }

  useEffect(() => {
    let disposed = false;
    let removeFocusListener: (() => void) | undefined;

    async function initializeDesktopValidation() {
      removeFocusListener = await appWindow.onFocusChanged(() => {
        void syncWindowState();
      });

      await refreshStatus();
    }

    initializeDesktopValidation().catch((error: unknown) => {
      const message =
        error instanceof Error ? error.message : "Desktop validation failed to initialize.";
      if (!disposed) {
        setStatusMessage(message);
      }
    });

    return () => {
      disposed = true;
      removeFocusListener?.();
    };
  }, []);

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Meeting Prep Assistant</p>
        <h1>Spike 1 Desktop Foundation</h1>
        <p className="intro">
          This validation build focuses only on the Windows desktop shell:
          app window, tray behavior, background running, autostart, and
          native notifications.
        </p>
      </section>

      <section className="panel">
        <h2>Validation Actions</h2>
        <div className="button-grid">
          <button onClick={sendValidationNotification} type="button">
            Send native notification
          </button>
          <button onClick={hideToTray} type="button">
            Hide window to tray
          </button>
          <button onClick={revealWindow} type="button">
            Show and focus window
          </button>
          <button onClick={toggleAutostart} type="button">
            {autostartEnabled ? "Disable autostart" : "Enable autostart"}
          </button>
          <button onClick={refreshStatus} type="button" className="secondary">
            Refresh status
          </button>
        </div>
      </section>

      <section className="panel">
        <h2>Current Status</h2>
        <dl className="status-grid">
          <div>
            <dt>Autostart</dt>
            <dd>
              {autostartEnabled === null
                ? "Checking..."
                : autostartEnabled
                  ? "Enabled"
                  : "Disabled"}
            </dd>
          </div>
          <div>
            <dt>Notification permission</dt>
            <dd>{notificationPermission}</dd>
          </div>
          <div>
            <dt>Tray behavior</dt>
            <dd>Window close is redirected to tray/background mode.</dd>
          </div>
          <div>
            <dt>Notification reopen path</dt>
            <dd>
              Current Tauri desktop notification path does not restore the app on Windows.
            </dd>
          </div>
          <div>
            <dt>Window visible</dt>
            <dd>{windowState === null ? "Checking..." : String(windowState.visible)}</dd>
          </div>
          <div>
            <dt>Window focused</dt>
            <dd>{windowState === null ? "Checking..." : String(windowState.focused)}</dd>
          </div>
          <div>
            <dt>Window minimized</dt>
            <dd>
              {windowState === null ? "Checking..." : String(windowState.minimized)}
            </dd>
          </div>
        </dl>
      </section>

      <section className="panel">
        <h2>Spike Notes</h2>
        <ul>
          <li>Closing the window should keep the app alive in the system tray.</li>
          <li>The tray menu should allow reopening the app or quitting it fully.</li>
          <li>The notification test is intentionally synthetic and does not use meeting data.</li>
          <li>Notification click restore is being recorded as a desktop-plugin limitation for this spike.</li>
        </ul>
        <p className="status-message">{statusMessage}</p>
      </section>
    </main>
  );
}

export default App;

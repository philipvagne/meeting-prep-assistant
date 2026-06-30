import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type GoogleAuthStatus = {
  configured: boolean;
  authenticated: boolean;
  hasRefreshToken: boolean;
  clientId: string | null;
  hasClientSecret: boolean;
  expiresAtEpochSeconds: number | null;
};

type CalendarEventSummary = {
  id: string;
  summary: string;
  start: string;
  end: string | null;
  allDay: boolean;
  status: string | null;
};

function formatTimestamp(value: string | number | null) {
  if (value === null) {
    return "Unknown";
  }

  const date =
    typeof value === "number" ? new Date(value * 1000) : new Date(value);
  if (Number.isNaN(date.getTime())) {
    return String(value);
  }

  return date.toLocaleString();
}

function App() {
  const [clientIdInput, setClientIdInput] = useState("");
  const [clientSecretInput, setClientSecretInput] = useState("");
  const [authStatus, setAuthStatus] = useState<GoogleAuthStatus | null>(null);
  const [events, setEvents] = useState<CalendarEventSummary[]>([]);
  const [busyAction, setBusyAction] = useState<string | null>(null);
  const [statusMessage, setStatusMessage] = useState(
    "Google OAuth and Calendar validation is ready.",
  );

  async function refreshAuthStatus() {
    const status = await invoke<GoogleAuthStatus>("get_google_auth_status");
    setAuthStatus(status);
    setClientIdInput(status.clientId ?? "");
    return status;
  }

  async function runAction(actionName: string, action: () => Promise<void>) {
    setBusyAction(actionName);

    try {
      await action();
    } catch (error: unknown) {
      const message =
        error instanceof Error
          ? error.message
          : typeof error === "string"
            ? error
            : "The validation action failed.";
      setStatusMessage(message);
    } finally {
      setBusyAction(null);
    }
  }

  async function saveClientConfig() {
    await runAction("saveClientConfig", async () => {
      const status = await invoke<GoogleAuthStatus>("save_google_client_config", {
        clientId: clientIdInput,
        clientSecret: clientSecretInput,
      });
      setAuthStatus(status);
      setClientSecretInput("");
      setStatusMessage("Google OAuth client configuration saved locally for this device.");
    });
  }

  async function connectGoogle() {
    await runAction("connectGoogle", async () => {
      if (!authStatus?.configured && clientIdInput.trim()) {
        await invoke<GoogleAuthStatus>("save_google_client_config", {
          clientId: clientIdInput,
          clientSecret: clientSecretInput,
        });
      }

      const status = await invoke<GoogleAuthStatus>("connect_google");
      setAuthStatus(status);
      setClientSecretInput("");
      setStatusMessage(
        "Google authentication completed. Calendar read-only access is now available.",
      );
    });
  }

  async function fetchUpcomingEvents() {
    await runAction("fetchEvents", async () => {
      const nextEvents = await invoke<CalendarEventSummary[]>(
        "fetch_upcoming_calendar_events",
      );
      setEvents(nextEvents);
      setStatusMessage(
        nextEvents.length === 0
          ? "Authentication is valid. No upcoming events were returned."
          : `Retrieved ${nextEvents.length} upcoming calendar events.`,
      );
      await refreshAuthStatus();
    });
  }

  async function disconnectGoogle() {
    await runAction("disconnectGoogle", async () => {
      await invoke("disconnect_google");
      setEvents([]);
      const status = await refreshAuthStatus();
      setStatusMessage(
        status.configured
          ? "Stored Google authentication was cleared from secure local storage."
          : "Stored Google authentication was cleared.",
      );
    });
  }

  useEffect(() => {
    refreshAuthStatus().catch((error: unknown) => {
      const message =
        error instanceof Error
          ? error.message
          : typeof error === "string"
            ? error
          : "Could not load the Google validation status.";
      setStatusMessage(message);
    });
  }, []);

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Meeting Prep Assistant</p>
        <h1>Spike 2 Google OAuth &amp; Calendar</h1>
        <p className="intro">
          This validation build checks whether a Windows desktop app can authenticate
          with Google through the system browser, store tokens securely on-device,
          and read upcoming calendar events with the minimum read-only scope.
        </p>
      </section>

      <section className="panel">
        <h2>Validation Setup</h2>
        <label className="field-label" htmlFor="google-client-id">
          Google OAuth desktop client ID
        </label>
        <input
          id="google-client-id"
          className="text-input"
          type="text"
          placeholder="Paste your Google desktop client ID"
          value={clientIdInput}
          onChange={(event) => setClientIdInput(event.target.value)}
        />
        <label className="field-label" htmlFor="google-client-secret">
          Google OAuth desktop client secret
        </label>
        <input
          id="google-client-secret"
          className="text-input"
          type="password"
          placeholder={
            authStatus?.hasClientSecret
              ? "Client secret already saved locally"
              : "Paste your Google desktop client secret"
          }
          value={clientSecretInput}
          onChange={(event) => setClientSecretInput(event.target.value)}
        />
        <p className="field-help">
          This spike stores the Google desktop client ID and client secret in the local
          app config directory for this device only. Refresh tokens remain stored
          separately in Windows-backed secure storage.
        </p>
        <div className="button-grid">
          <button
            onClick={saveClientConfig}
            type="button"
            disabled={
              busyAction !== null || !clientIdInput.trim() || !clientSecretInput.trim()
            }
          >
            Save Google config
          </button>
          <button
            onClick={connectGoogle}
            type="button"
            disabled={
              busyAction !== null ||
              !clientIdInput.trim() ||
              (!clientSecretInput.trim() && !authStatus?.hasClientSecret)
            }
          >
            Connect Google
          </button>
          <button
            onClick={fetchUpcomingEvents}
            type="button"
            disabled={busyAction !== null || !authStatus?.authenticated}
          >
            Fetch upcoming events
          </button>
          <button
            onClick={disconnectGoogle}
            type="button"
            className="secondary"
            disabled={busyAction !== null || !authStatus?.authenticated}
          >
            Disconnect Google
          </button>
        </div>
      </section>

      <section className="panel">
        <h2>Authentication Status</h2>
        <dl className="status-grid">
          <div>
            <dt>Client ID configured</dt>
            <dd>{authStatus === null ? "Checking..." : String(authStatus.configured)}</dd>
          </div>
          <div>
            <dt>Client secret configured</dt>
            <dd>{authStatus === null ? "Checking..." : String(authStatus.hasClientSecret)}</dd>
          </div>
          <div>
            <dt>Authenticated</dt>
            <dd>{authStatus === null ? "Checking..." : String(authStatus.authenticated)}</dd>
          </div>
          <div>
            <dt>Refresh token present</dt>
            <dd>
              {authStatus === null ? "Checking..." : String(authStatus.hasRefreshToken)}
            </dd>
          </div>
          <div>
            <dt>Requested scope</dt>
            <dd>Google Calendar read-only only</dd>
          </div>
          <div>
            <dt>Next token expiry</dt>
            <dd>
              {authStatus === null
                ? "Checking..."
                : formatTimestamp(authStatus.expiresAtEpochSeconds)}
            </dd>
          </div>
          <div>
            <dt>Token storage</dt>
            <dd>OS-backed secure storage on Windows</dd>
          </div>
        </dl>
      </section>

      <section className="panel">
        <h2>Upcoming Events</h2>
        <p className="field-help">
          Only minimal metadata is shown here for validation: title, start, end, and event
          status.
        </p>
        {events.length === 0 ? (
          <p className="empty-state">No validation events loaded yet.</p>
        ) : (
          <div className="event-list">
            {events.map((event) => (
              <article key={event.id} className="event-card">
                <h3>{event.summary}</h3>
                <p>Start: {formatTimestamp(event.start)}</p>
                <p>End: {formatTimestamp(event.end)}</p>
                <p>{event.allDay ? "All-day event" : "Timed event"}</p>
                <p>Status: {event.status ?? "unknown"}</p>
              </article>
            ))}
          </div>
        )}
      </section>

      <section className="panel">
        <h2>Spike Guardrails</h2>
        <ul>
          <li>Only the Google Calendar read-only scope is requested.</li>
          <li>No Gmail, Drive, profile, or write scopes are used.</li>
          <li>No Google data is sent to developer-controlled servers.</li>
          <li>This is a validation UI only and not the final product design.</li>
        </ul>
        <p className="status-message">{statusMessage}</p>
      </section>
    </main>
  );
}

export default App;

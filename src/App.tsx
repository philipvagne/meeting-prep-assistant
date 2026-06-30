import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type GoogleAuthStatus = {
  configured: boolean;
  authenticated: boolean;
  hasRefreshToken: boolean;
  clientId: string | null;
  hasClientSecret: boolean;
  expiresAtEpochSeconds: number | null;
  grantedScopes: string[];
  hasRequiredScopes: boolean;
};

type CalendarEventSummary = {
  id: string;
  summary: string;
  start: string;
  end: string | null;
  allDay: boolean;
  status: string | null;
  htmlLink: string | null;
};

type CalendarSeedMetadata = {
  id: string;
  summary: string;
  start: string;
  end: string | null;
  status: string | null;
  descriptionPreview: string | null;
  attendeeEmails: string[];
  htmlLink: string | null;
};

type GmailContextResult = {
  id: string;
  threadId: string;
  subject: string;
  from: string | null;
  date: string | null;
  snippet: string | null;
  sourceLink: string;
};

type DriveContextResult = {
  id: string;
  name: string;
  mimeType: string | null;
  modifiedTime: string | null;
  webViewLink: string | null;
};

type MeetingContextCollection = {
  seed: CalendarSeedMetadata;
  gmailResults: GmailContextResult[];
  driveResults: DriveContextResult[];
  notes: string[];
};

const REQUESTED_SCOPES = [
  "Google Calendar read-only",
  "Gmail read-only",
  "Google Drive read-only",
];

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
  const [selectedEventId, setSelectedEventId] = useState("");
  const [contextResult, setContextResult] = useState<MeetingContextCollection | null>(null);
  const [busyAction, setBusyAction] = useState<string | null>(null);
  const [statusMessage, setStatusMessage] = useState(
    "Spike 3 Gmail and Drive context validation is ready.",
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
      setStatusMessage("Google OAuth configuration saved locally for this device.");
    });
  }

  async function connectGoogle() {
    await runAction("connectGoogle", async () => {
      if (!authStatus?.configured && clientIdInput.trim() && clientSecretInput.trim()) {
        await invoke<GoogleAuthStatus>("save_google_client_config", {
          clientId: clientIdInput,
          clientSecret: clientSecretInput,
        });
      }

      const status = await invoke<GoogleAuthStatus>("connect_google");
      setAuthStatus(status);
      setClientSecretInput("");
      setEvents([]);
      setSelectedEventId("");
      setContextResult(null);
      setStatusMessage(
        "Google authentication completed with Calendar, Gmail, and Drive read-only access.",
      );
    });
  }

  async function fetchUpcomingEvents() {
    await runAction("fetchEvents", async () => {
      const nextEvents = await invoke<CalendarEventSummary[]>(
        "fetch_upcoming_calendar_events",
      );
      setEvents(nextEvents);
      setSelectedEventId(nextEvents[0]?.id ?? "");
      setContextResult(null);
      setStatusMessage(
        nextEvents.length === 0
          ? "Authentication is valid, but no upcoming calendar events were returned."
          : `Retrieved ${nextEvents.length} upcoming calendar events for context selection.`,
      );
      await refreshAuthStatus();
    });
  }

  async function collectContext() {
    if (!selectedEventId) {
      setStatusMessage("Select an upcoming calendar event before collecting context.");
      return;
    }

    await runAction("collectContext", async () => {
      const result = await invoke<MeetingContextCollection>("collect_meeting_context", {
        eventId: selectedEventId,
      });
      setContextResult(result);
      setStatusMessage(
        `Context collection finished. Gmail results: ${result.gmailResults.length}. Drive results: ${result.driveResults.length}.`,
      );
    });
  }

  async function disconnectGoogle() {
    await runAction("disconnectGoogle", async () => {
      await invoke("disconnect_google");
      setEvents([]);
      setSelectedEventId("");
      setContextResult(null);
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

  const canConnect =
    busyAction === null &&
    !!clientIdInput.trim() &&
    (!!clientSecretInput.trim() || !!authStatus?.hasClientSecret);
  const canUseContextFlow =
    busyAction === null &&
    !!authStatus?.authenticated &&
    !!authStatus?.hasRequiredScopes;

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Meeting Prep Assistant</p>
        <h1>Spike 3 Gmail &amp; Drive Context Collection</h1>
        <p className="intro">
          This validation build checks whether one upcoming Google Calendar event can
          be used to find related Gmail messages and Google Drive files using read-only
          permissions only.
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
          The client ID and client secret are stored locally on this device only.
          Refresh tokens remain protected separately using Windows-backed secure storage.
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
          <button onClick={connectGoogle} type="button" disabled={!canConnect}>
            {authStatus?.authenticated && !authStatus?.hasRequiredScopes
              ? "Upgrade Google access"
              : "Connect Google"}
          </button>
          <button onClick={fetchUpcomingEvents} type="button" disabled={!canUseContextFlow}>
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
            <dt>Required scopes granted</dt>
            <dd>
              {authStatus === null ? "Checking..." : String(authStatus.hasRequiredScopes)}
            </dd>
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
            <dt>Requested scopes</dt>
            <dd>{REQUESTED_SCOPES.join(", ")}</dd>
          </div>
          <div>
            <dt>Granted scopes</dt>
            <dd>
              {authStatus === null
                ? "Checking..."
                : authStatus.grantedScopes.length === 0
                  ? "None yet"
                  : authStatus.grantedScopes.join(", ")}
            </dd>
          </div>
        </dl>
      </section>

      <section className="panel">
        <h2>Select Calendar Seed</h2>
        <p className="field-help">
          Fetch upcoming events, choose one event, then collect related Gmail and Drive
          context from that seed.
        </p>
        <label className="field-label" htmlFor="event-select">
          Upcoming calendar event
        </label>
        <select
          id="event-select"
          className="text-input"
          value={selectedEventId}
          onChange={(event) => setSelectedEventId(event.target.value)}
          disabled={events.length === 0}
        >
          <option value="">Select an upcoming event</option>
          {events.map((event) => (
            <option key={event.id} value={event.id}>
              {event.summary} — {formatTimestamp(event.start)}
            </option>
          ))}
        </select>
        <div className="button-grid">
          <button
            onClick={collectContext}
            type="button"
            disabled={!canUseContextFlow || !selectedEventId}
          >
            Collect context
          </button>
        </div>
        {events.length > 0 ? (
          <div className="event-list">
            {events.map((event) => (
              <article
                key={event.id}
                className={`event-card${selectedEventId === event.id ? " selected-card" : ""}`}
              >
                <h3>{event.summary}</h3>
                <p>Start: {formatTimestamp(event.start)}</p>
                <p>End: {formatTimestamp(event.end)}</p>
                <p>{event.allDay ? "All-day event" : "Timed event"}</p>
                <p>Status: {event.status ?? "unknown"}</p>
                {event.htmlLink ? (
                  <p>
                    <a href={event.htmlLink} target="_blank" rel="noreferrer">
                      Open calendar source
                    </a>
                  </p>
                ) : null}
              </article>
            ))}
          </div>
        ) : (
          <p className="empty-state">No upcoming events loaded yet.</p>
        )}
      </section>

      <section className="panel">
        <h2>Collected Context</h2>
        {contextResult === null ? (
          <p className="empty-state">No context has been collected yet.</p>
        ) : (
          <>
            <div className="subsection">
              <h3>Calendar Seed</h3>
              <div className="event-card">
                <p>Title: {contextResult.seed.summary}</p>
                <p>Start: {formatTimestamp(contextResult.seed.start)}</p>
                <p>End: {formatTimestamp(contextResult.seed.end)}</p>
                <p>Status: {contextResult.seed.status ?? "unknown"}</p>
                <p>
                  Attendees:{" "}
                  {contextResult.seed.attendeeEmails.length === 0
                    ? "None exposed"
                    : contextResult.seed.attendeeEmails.join(", ")}
                </p>
                <p>
                  Description preview:{" "}
                  {contextResult.seed.descriptionPreview ?? "No description available"}
                </p>
                {contextResult.seed.htmlLink ? (
                  <p>
                    <a href={contextResult.seed.htmlLink} target="_blank" rel="noreferrer">
                      Open calendar source
                    </a>
                  </p>
                ) : null}
              </div>
            </div>

            <div className="subsection">
              <h3>Gmail Results</h3>
              {contextResult.gmailResults.length === 0 ? (
                <p className="empty-state">No Gmail results found for this event.</p>
              ) : (
                <div className="source-grid">
                  {contextResult.gmailResults.map((message) => (
                    <article key={message.id} className="event-card">
                      <h4>{message.subject}</h4>
                      <p>From: {message.from ?? "Unknown sender"}</p>
                      <p>Date: {message.date ?? "Unknown date"}</p>
                      <p>Snippet: {message.snippet ?? "No snippet available"}</p>
                      <p>Thread ID: {message.threadId}</p>
                      <p>
                        <a href={message.sourceLink} target="_blank" rel="noreferrer">
                          Open Gmail source
                        </a>
                      </p>
                    </article>
                  ))}
                </div>
              )}
            </div>

            <div className="subsection">
              <h3>Drive Results</h3>
              {contextResult.driveResults.length === 0 ? (
                <p className="empty-state">No Drive results found for this event.</p>
              ) : (
                <div className="source-grid">
                  {contextResult.driveResults.map((file) => (
                    <article key={file.id} className="event-card">
                      <h4>{file.name}</h4>
                      <p>MIME type: {file.mimeType ?? "Unknown type"}</p>
                      <p>Modified: {formatTimestamp(file.modifiedTime)}</p>
                      <p>File ID: {file.id}</p>
                      {file.webViewLink ? (
                        <p>
                          <a href={file.webViewLink} target="_blank" rel="noreferrer">
                            Open Drive source
                          </a>
                        </p>
                      ) : null}
                    </article>
                  ))}
                </div>
              )}
            </div>

            <div className="subsection">
              <h3>Validation Notes</h3>
              {contextResult.notes.length === 0 ? (
                <p className="empty-state">No additional notes.</p>
              ) : (
                <ul>
                  {contextResult.notes.map((note) => (
                    <li key={note}>{note}</li>
                  ))}
                </ul>
              )}
            </div>
          </>
        )}
      </section>

      <section className="panel">
        <h2>Spike Guardrails</h2>
        <ul>
          <li>Only Calendar, Gmail, and Drive read-only scopes are requested.</li>
          <li>No Google write permissions are used.</li>
          <li>No AI provider receives user data during this spike.</li>
          <li>Only minimal validation metadata is displayed for Calendar, Gmail, and Drive.</li>
        </ul>
        <p className="status-message">{statusMessage}</p>
      </section>
    </main>
  );
}

export default App;

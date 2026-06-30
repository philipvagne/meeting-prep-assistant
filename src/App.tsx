import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

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

type AiProviderKind = "openAi" | "gemini";

type ProviderConfigStatus = {
  provider: AiProviderKind;
  label: string;
  model: string;
  hasApiKey: boolean;
  configured: boolean;
};

type AiProviderStatus = {
  selectedProvider: AiProviderKind;
  selectedProviderLabel: string;
  configured: boolean;
  providers: ProviderConfigStatus[];
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

type BriefSourceReference = {
  sourceId: string;
  sourceType: string;
  label: string;
  link: string | null;
};

type GeneratedMeetingBrief = {
  meetingTitle: string;
  provider: AiProviderKind;
  providerLabel: string;
  model: string;
  confidenceLabel: string;
  contextStrength: string;
  briefMarkdown: string;
  sources: BriefSourceReference[];
};

const REQUESTED_SCOPES = [
  "Google Calendar read-only",
  "Gmail read-only",
  "Google Drive read-only",
];

const PROVIDER_OPTIONS: { value: AiProviderKind; label: string; defaultModel: string }[] = [
  { value: "openAi", label: "OpenAI", defaultModel: "gpt-4.1" },
  { value: "gemini", label: "Google Gemini", defaultModel: "gemini-2.5-flash" },
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

function providerLabel(provider: AiProviderKind) {
  return PROVIDER_OPTIONS.find((option) => option.value === provider)?.label ?? provider;
}

function providerDefaultModel(provider: AiProviderKind) {
  return (
    PROVIDER_OPTIONS.find((option) => option.value === provider)?.defaultModel ?? "gpt-4.1"
  );
}

function App() {
  const [clientIdInput, setClientIdInput] = useState("");
  const [clientSecretInput, setClientSecretInput] = useState("");
  const [selectedAiProvider, setSelectedAiProvider] = useState<AiProviderKind>("openAi");
  const [providerApiKeyInput, setProviderApiKeyInput] = useState("");
  const [providerModelInput, setProviderModelInput] = useState("gpt-4.1");
  const [authStatus, setAuthStatus] = useState<GoogleAuthStatus | null>(null);
  const [aiStatus, setAiStatus] = useState<AiProviderStatus | null>(null);
  const [events, setEvents] = useState<CalendarEventSummary[]>([]);
  const [selectedEventId, setSelectedEventId] = useState("");
  const [contextResult, setContextResult] = useState<MeetingContextCollection | null>(null);
  const [generatedBrief, setGeneratedBrief] = useState<GeneratedMeetingBrief | null>(null);
  const [busyAction, setBusyAction] = useState<string | null>(null);
  const [statusMessage, setStatusMessage] = useState(
    "Spike 4 brief generation validation is ready.",
  );

  function providerStatusFor(provider: AiProviderKind) {
    return aiStatus?.providers.find((item) => item.provider === provider) ?? null;
  }

  function syncProviderInputs(status: AiProviderStatus, provider: AiProviderKind) {
    const nextProviderStatus =
      status.providers.find((item) => item.provider === provider) ?? null;
    setProviderModelInput(nextProviderStatus?.model ?? providerDefaultModel(provider));
  }

  async function refreshGoogleAuthStatus() {
    const status = await invoke<GoogleAuthStatus>("get_google_auth_status");
    setAuthStatus(status);
    setClientIdInput(status.clientId ?? "");
    return status;
  }

  async function refreshAiStatus() {
    const status = await invoke<AiProviderStatus>("get_ai_provider_status");
    setAiStatus(status);
    setSelectedAiProvider(status.selectedProvider);
    syncProviderInputs(status, status.selectedProvider);
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

  async function saveGoogleConfig() {
    await runAction("saveGoogleConfig", async () => {
      const status = await invoke<GoogleAuthStatus>("save_google_client_config", {
        clientId: clientIdInput,
        clientSecret: clientSecretInput,
      });
      setAuthStatus(status);
      setClientSecretInput("");
      setStatusMessage("Google OAuth configuration saved locally for this device.");
    });
  }

  async function saveAiConfig() {
    await runAction("saveAiConfig", async () => {
      const status = await invoke<AiProviderStatus>("save_ai_provider_config", {
        request: {
          provider: selectedAiProvider,
          apiKey: providerApiKeyInput.trim() ? providerApiKeyInput : null,
          model: providerModelInput.trim() ? providerModelInput : null,
        },
      });
      setAiStatus(status);
      setSelectedAiProvider(status.selectedProvider);
      syncProviderInputs(status, status.selectedProvider);
      setProviderApiKeyInput("");
      setStatusMessage(
        `${providerLabel(status.selectedProvider)} validation configuration saved locally for this device.`,
      );
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
      setGeneratedBrief(null);
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
      setGeneratedBrief(null);
      setStatusMessage(
        nextEvents.length === 0
          ? "Authentication is valid, but no upcoming calendar events were returned."
          : `Retrieved ${nextEvents.length} upcoming calendar events for context selection.`,
      );
      await refreshGoogleAuthStatus();
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
      setGeneratedBrief(null);
      setStatusMessage(
        `Context collection finished. Gmail results: ${result.gmailResults.length}. Drive results: ${result.driveResults.length}.`,
      );
    });
  }

  async function ensureNotificationPermission() {
    if (await isPermissionGranted()) {
      return true;
    }

    const permission = await requestPermission();
    return permission === "granted";
  }

  async function generateBrief() {
    if (!contextResult) {
      setStatusMessage("Collect context before generating a brief.");
      return;
    }

    await runAction("generateBrief", async () => {
      const brief = await invoke<GeneratedMeetingBrief>("generate_meeting_brief", {
        context: contextResult,
      });
      setGeneratedBrief(brief);
      setStatusMessage(
        `${brief.providerLabel} brief generated. Confidence: ${brief.confidenceLabel}. Context strength: ${brief.contextStrength}.`,
      );

      if (await ensureNotificationPermission()) {
        sendNotification({
          title: "Meeting brief ready",
          body: `${brief.meetingTitle} is ready to review in Meeting Prep Assistant.`,
          autoCancel: true,
        });
      }
    });
  }

  async function disconnectGoogle() {
    await runAction("disconnectGoogle", async () => {
      await invoke("disconnect_google");
      setEvents([]);
      setSelectedEventId("");
      setContextResult(null);
      setGeneratedBrief(null);
      const status = await refreshGoogleAuthStatus();
      setStatusMessage(
        status.configured
          ? "Stored Google authentication was cleared from secure local storage."
          : "Stored Google authentication was cleared.",
      );
    });
  }

  useEffect(() => {
    refreshGoogleAuthStatus().catch((error: unknown) => {
      const message =
        error instanceof Error
          ? error.message
          : typeof error === "string"
            ? error
            : "Could not load the Google validation status.";
      setStatusMessage(message);
    });

    refreshAiStatus().catch((error: unknown) => {
      const message =
        error instanceof Error
          ? error.message
          : typeof error === "string"
            ? error
            : "Could not load the AI validation status.";
      setStatusMessage(message);
    });
  }, []);

  const selectedProviderStatus = providerStatusFor(selectedAiProvider);
  const canConnectGoogle =
    busyAction === null &&
    !!clientIdInput.trim() &&
    (!!clientSecretInput.trim() || !!authStatus?.hasClientSecret);
  const canUseContextFlow =
    busyAction === null &&
    !!authStatus?.authenticated &&
    !!authStatus?.hasRequiredScopes;
  const canGenerateBrief =
    busyAction === null && !!contextResult && !!aiStatus?.configured;

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Meeting Prep Assistant</p>
        <h1>Spike 4 Brief Generation &amp; Notification Flow</h1>
        <p className="intro">
          This validation build checks whether already-collected Calendar, Gmail,
          and Drive context can be turned into a concise source-backed meeting brief
          using a user-selected AI provider and surfaced through the desktop
          notification flow.
        </p>
      </section>

      <section className="panel">
        <h2>Google Validation Setup</h2>
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
          The Google client ID and client secret are stored locally on this device only.
          Refresh tokens remain protected separately using Windows-backed secure storage.
        </p>
        <div className="button-grid">
          <button
            onClick={saveGoogleConfig}
            type="button"
            disabled={
              busyAction !== null || !clientIdInput.trim() || !clientSecretInput.trim()
            }
          >
            Save Google config
          </button>
          <button onClick={connectGoogle} type="button" disabled={!canConnectGoogle}>
            {authStatus?.authenticated && !authStatus?.hasRequiredScopes
              ? "Upgrade Google access"
              : "Connect Google"}
          </button>
          <button onClick={disconnectGoogle} type="button" className="secondary">
            Disconnect Google
          </button>
        </div>
      </section>

      <section className="panel">
        <h2>AI Validation Setup</h2>
        <label className="field-label" htmlFor="ai-provider-select">
          AI provider
        </label>
        <select
          id="ai-provider-select"
          className="text-input"
          value={selectedAiProvider}
          onChange={(event) => {
            const nextProvider = event.target.value as AiProviderKind;
            setSelectedAiProvider(nextProvider);
            setProviderModelInput(
              providerStatusFor(nextProvider)?.model ?? providerDefaultModel(nextProvider),
            );
            setProviderApiKeyInput("");
          }}
        >
          {PROVIDER_OPTIONS.map((provider) => (
            <option key={provider.value} value={provider.value}>
              {provider.label}
            </option>
          ))}
        </select>
        <label className="field-label" htmlFor="provider-api-key">
          {providerLabel(selectedAiProvider)} API key
        </label>
        <input
          id="provider-api-key"
          className="text-input"
          type="password"
          placeholder={
            selectedProviderStatus?.hasApiKey
              ? `${providerLabel(selectedAiProvider)} API key already saved locally`
              : `Paste your local ${providerLabel(selectedAiProvider)} API key`
          }
          value={providerApiKeyInput}
          onChange={(event) => setProviderApiKeyInput(event.target.value)}
        />
        <label className="field-label" htmlFor="provider-model">
          {providerLabel(selectedAiProvider)} model
        </label>
        <input
          id="provider-model"
          className="text-input"
          type="text"
          placeholder={providerDefaultModel(selectedAiProvider)}
          value={providerModelInput}
          onChange={(event) => setProviderModelInput(event.target.value)}
        />
        <p className="field-help">
          The selected provider becomes the active brief-generation path. API keys are
          stored locally only, one per provider, and only the selected event's minimal
          Calendar, Gmail, and Drive context is sent to the provider.
        </p>
        <div className="button-grid">
          <button onClick={saveAiConfig} type="button" disabled={busyAction !== null}>
            Save AI config
          </button>
        </div>
      </section>

      <section className="panel">
        <h2>Status</h2>
        <dl className="status-grid">
          <div>
            <dt>Google authenticated</dt>
            <dd>{authStatus === null ? "Checking..." : String(authStatus.authenticated)}</dd>
          </div>
          <div>
            <dt>Google scopes granted</dt>
            <dd>
              {authStatus === null
                ? "Checking..."
                : authStatus.grantedScopes.length === 0
                  ? "None yet"
                  : authStatus.grantedScopes.join(", ")}
            </dd>
          </div>
          <div>
            <dt>Selected AI provider</dt>
            <dd>{aiStatus === null ? "Checking..." : aiStatus.selectedProviderLabel}</dd>
          </div>
          <div>
            <dt>Selected provider ready</dt>
            <dd>{aiStatus === null ? "Checking..." : String(aiStatus.configured)}</dd>
          </div>
          <div>
            <dt>Selected model</dt>
            <dd>{selectedProviderStatus?.model ?? providerModelInput}</dd>
          </div>
          <div>
            <dt>Requested scopes</dt>
            <dd>{REQUESTED_SCOPES.join(", ")}</dd>
          </div>
        </dl>
        <div className="subsection">
          <h3>Provider Readiness</h3>
          <div className="source-grid">
            {aiStatus?.providers.map((provider) => (
              <article key={provider.provider} className="event-card">
                <h4>{provider.label}</h4>
                <p>Model: {provider.model}</p>
                <p>API key saved: {String(provider.hasApiKey)}</p>
                <p>Configured: {String(provider.configured)}</p>
              </article>
            )) ?? <p className="empty-state">Checking AI provider status...</p>}
          </div>
        </div>
      </section>

      <section className="panel">
        <h2>Manual Validation Flow</h2>
        <p className="field-help">
          Fetch one upcoming event, collect context, then generate a brief manually from
          that selected event only using the currently selected AI provider.
        </p>
        <div className="button-grid">
          <button onClick={fetchUpcomingEvents} type="button" disabled={!canUseContextFlow}>
            Fetch upcoming events
          </button>
          <button
            onClick={collectContext}
            type="button"
            disabled={!canUseContextFlow || !selectedEventId}
          >
            Collect context
          </button>
          <button onClick={generateBrief} type="button" disabled={!canGenerateBrief}>
            Generate brief
          </button>
        </div>
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
              {event.summary} - {formatTimestamp(event.start)}
            </option>
          ))}
        </select>
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
        <h2>Generated Brief</h2>
        {generatedBrief === null ? (
          <p className="empty-state">No meeting brief generated yet.</p>
        ) : (
          <>
            <div className="status-grid">
              <div>
                <dt>Meeting</dt>
                <dd>{generatedBrief.meetingTitle}</dd>
              </div>
              <div>
                <dt>Provider</dt>
                <dd>{generatedBrief.providerLabel}</dd>
              </div>
              <div>
                <dt>Model</dt>
                <dd>{generatedBrief.model}</dd>
              </div>
              <div>
                <dt>Confidence</dt>
                <dd>{generatedBrief.confidenceLabel}</dd>
              </div>
              <div>
                <dt>Context strength</dt>
                <dd>{generatedBrief.contextStrength}</dd>
              </div>
            </div>
            <pre className="brief-output">{generatedBrief.briefMarkdown}</pre>
            <div className="subsection">
              <h3>Sources</h3>
              <div className="source-grid">
                {generatedBrief.sources.map((source) => (
                  <article key={source.sourceId} className="event-card">
                    <h4>{source.sourceId}</h4>
                    <p>{source.sourceType}</p>
                    <p>{source.label}</p>
                    {source.link ? (
                      <p>
                        <a href={source.link} target="_blank" rel="noreferrer">
                          Open source
                        </a>
                      </p>
                    ) : null}
                  </article>
                ))}
              </div>
            </div>
          </>
        )}
      </section>

      <section className="panel">
        <h2>Spike Guardrails</h2>
        <ul>
          <li>Only selected-event context is sent to the active AI provider.</li>
          <li>No OAuth tokens, Google client credentials, or provider API keys are sent in the brief prompt.</li>
          <li>No Google write permissions are requested or used.</li>
          <li>The app uses the tray restore path as the supported way to return and view the generated brief.</li>
        </ul>
        <p className="status-message">{statusMessage}</p>
      </section>
    </main>
  );
}

export default App;

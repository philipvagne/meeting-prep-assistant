# Spike 4 — Brief Generation & Notification Flow

## Purpose

Validate whether Meeting Prep Assistant can turn collected meeting context into a short, useful, source-backed meeting brief and deliver it through the desktop notification flow.

This spike focuses on brief generation, source traceability, and notification handoff. It does not implement final product UI, long-term scheduling, or production polish.

---

## Engineering Question

Can the application generate a concise, useful meeting-prep brief from Calendar, Gmail, and Drive context, preserve source traceability, and notify the user that the brief is ready?

---

## Hypothesis

The collected context from Spike 3 can be transformed into a short meeting brief that helps the user prepare without exposing unnecessary data or inventing unsupported claims.

If this spike succeeds, the core product promise is technically validated.

---

## Scope

Validate:

- Using collected Calendar/Gmail/Drive context as brief input
- Sending minimal required context to an AI provider
- Generating a short meeting-prep brief
- Including source references for important claims
- Avoiding unsupported claims
- Handling low-context cases safely
- Displaying the generated brief in the desktop app
- Sending a native notification when the brief is ready
- Opening or surfacing the generated brief from the app/tray flow

---

## Out of Scope

Do NOT implement:

- Final UI design
- Background scheduling for all meetings
- Repeated reminder logic
- Tray unread badge
- Production notification center
- Gmail or Drive write operations
- Editing Google data
- Multi-user accounts
- Microsoft/Slack/Zoom support

---

## Security Requirements

The spike only passes if:

- Only relevant context is sent to the AI provider.
- OAuth tokens are never sent to the AI provider.
- Full raw Gmail/Drive content is not sent unless explicitly required for validation.
- Generated claims must be traceable to source material.
- If context is missing or weak, the brief must say so instead of guessing.
- No Google data is sent to developer-controlled servers.
- Logs must not contain OAuth tokens, raw sensitive content, or generated private briefs.

---

## Success Criteria

This spike passes if:

- The application can generate a meeting brief from collected context.
- The brief is concise and useful.
- The brief includes source references.
- The brief avoids unsupported claims.
- Low-context cases are handled honestly.
- The brief is displayed inside the desktop app.
- A native notification is sent when the brief is ready.
- The user can return to the app/tray to view the brief.
- No Google write permissions are requested.
- No sensitive tokens or raw private content are logged.

---

## Result

PASS WITH LIMITATIONS.

---

## Lessons Learned

* The validation-safe context gathered in Spike 3 is sufficient to drive a concise AI-generated meeting brief without sending full raw email or document content.
* A local-only AI configuration path can fit cleanly into the existing desktop validation flow when the API key is stored separately from repo files.
* Deterministic source IDs appended to the brief prompt give the model a clearer citation target than source names alone.
* Native brief-ready notification can be triggered after successful manual brief generation without adding background scheduling.
* A simple provider abstraction is enough for this validation phase and prevents the brief-generation path from being tightly coupled to one vendor.
* `gemini-2.5-flash` is the preferred default Gemini validation model because it improves output quality for this task while remaining cost-effective.

---

## Limitations

* Brief format and content quality are not final.
* Prompt design still needs future iteration.
* Generated markdown still needs future UI rendering and presentation work.
* The current brief format depends on model obedience to the requested markdown structure and citation instructions.
* Notification click restore remains limited by the current Tauri Windows notification path documented in Spike 1, so the supported return path is reopening the app from the tray.
* Because this is validation code, the UI prioritizes clarity over final product polish.

---

## Implementation Approach

* Add a validation-only AI provider configuration path stored locally on the desktop machine.
* Keep the rest of the app provider-neutral by routing brief generation through a shared backend provider interface.
* Keep Google OAuth configuration and token storage separate from AI-provider configuration.
* Reuse already-collected selected-event Calendar, Gmail, and Drive context as the only brief input.
* Send only minimal validation metadata and snippets to the AI provider.
* Support both OpenAI and Gemini behind the same internal brief-generation path.
* Instruct the model to:
  * keep the brief concise
  * avoid unsupported claims
  * acknowledge low-context cases
  * cite explicit source IDs
* Display the generated brief in the app and send a native Windows notification when the brief is ready.

---

## Manual Validation Flow

1. Authenticate with Google.
2. Fetch upcoming events.
3. Select one event.
4. Collect context.
5. Select either OpenAI or Gemini and save the local AI provider configuration.
6. Click `Generate brief`.
7. Review the generated brief in the app.
8. Confirm a native notification appears when the brief is ready.
9. Use the app or tray restore path to return to the brief if needed.

---

## Final Status

PASS WITH LIMITATIONS.

---

## Validation Outcome

* AI provider abstraction works: PASS
* Google Gemini brief generation works: PASS
* OpenAI provider path remains available: PASS WITH LIMITATIONS
* Local-only provider API key storage works: PASS
* Calendar, Gmail, and Drive context is passed into the AI layer: PASS
* Brief can be generated from collected context: PASS WITH LIMITATIONS
* Brief includes source references: PASS
* Brief avoids unsupported claims: PASS WITH LIMITATIONS
* Low-context cases are handled honestly: PASS WITH LIMITATIONS
* Brief is displayed in the app: PASS
* Native notification appears when brief is ready: PASS
* No Google write permissions are requested: PASS
* No sensitive secrets or tokens are logged: PASS

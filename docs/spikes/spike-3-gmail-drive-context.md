# Spike 3 — Gmail & Drive Context Collection

## Purpose

Validate whether Meeting Prep Assistant can gather useful meeting-related context from Gmail and Google Drive using the authenticated Google account while respecting the project's security principles.

This spike focuses only on context retrieval and relevance validation. It does not generate meeting briefs, use AI, or implement final product UX.

---

## Engineering Question

Can the application use a Google Calendar event to retrieve relevant Gmail threads and Google Drive documents that could help prepare the user for an upcoming meeting?

---

## Hypothesis

Meeting metadata such as title, description, attendees, and timing can be used to search Gmail and Google Drive for useful related context.

If this spike succeeds, the project can continue toward source-backed meeting brief generation.

---

## Scope

Validate:

* Gmail read-only access
* Google Drive read-only access
* Scope expansion from Calendar-only to Calendar + Gmail + Drive
* Searching Gmail using meeting title, description keywords, and attendee emails
* Searching Google Drive using meeting title and extracted keywords
* Retrieving minimal safe metadata from Gmail and Drive results
* Selecting a small set of relevant sources
* Returning source references suitable for later citation
* Handling no-result and low-confidence cases safely

---

## Out of Scope

Do NOT implement:

* AI-generated meeting briefs
* Prompt building
* Final summary generation
* Background meeting monitoring
* Notification scheduling
* Final UI design
* Microsoft support
* Slack support
* Zoom support
* Local file scanning
* Writing or modifying Google data

---

## Security Requirements

The spike only passes if:

* Gmail access is read-only.
* Google Drive access is read-only.
* No Gmail or Drive write permissions are requested.
* Only the minimum scopes required for this spike are requested.
* Retrieved content is displayed only for validation.
* Sensitive content is not logged.
* No Google data is sent to developer-controlled servers.
* No AI provider receives user data during this spike.

---

## Success Criteria

This spike passes if:

* The application can expand Google authorization to include Gmail read-only and Drive read-only access.
* The application can retrieve a selected upcoming Calendar event.
* The application can search Gmail for related messages.
* The application can search Google Drive for related documents.
* The application can return useful metadata and snippets for validation.
* The application can clearly distinguish Gmail sources from Drive sources.
* The application can provide source identifiers or links suitable for later citation.
* The application handles no-result cases without guessing or fabricating context.
* No write permissions are requested.

---

## Result

Core functionality validated.

---

## Lessons Learned

* The existing desktop OAuth foundation from Spike 2 can be extended to Gmail and Drive read-only access without introducing a backend service.
* Scope upgrades should be treated as a full reconnect path so the app can replace earlier Calendar-only tokens safely.
* Calendar event details provide enough seed material to build a simple first-pass Gmail and Drive search strategy for validation.
* Minimal metadata is sufficient to validate likely relevance without exposing full email bodies or document contents.
* Gmail no-result paths need defensive parsing because the validation helper can return empty or non-object JSON responses even when no real Gmail error occurred.
* Realistic but synthetic project data was sufficient to validate the end-to-end context-collection path for both Gmail and Drive.

---

## Limitations

* Gmail matching currently relies primarily on meeting-title matching plus a small set of attendee and keyword variants. It is intentionally validation-grade rather than production relevance ranking.
* This limitation was observed directly during validation: the meeting `Weekly Architecture Review` did not discover emails such as `Architecture Review - database`, but after renaming the meeting to `Architecture Review`, those Gmail results were found immediately.
* This should be treated as a search-strategy limitation rather than an API limitation.
* Future iterations should improve relevance through title normalization, keyword extraction, multiple search variants, and semantic matching.
* Drive matching currently uses keyword-based `name` and `fullText` queries and returns only minimal file metadata.
* The validation UI shows snippets and metadata only. It does not inspect or rank full message bodies or document contents.
* Disconnect clears local authentication state only. It does not revoke the refresh token remotely.
* Gmail empty-result handling is now tolerant of blank and unexpected JSON response shapes, but the underlying response variation should still be observed during more live validation.

---

## Implementation Approach

* Expand the Google OAuth request to include:
  * Calendar read-only
  * Gmail read-only
  * Drive read-only
* Treat previously stored Calendar-only tokens as needing reconnection so scope upgrades stay explicit.
* Fetch upcoming calendar events and allow one event to be selected as the context seed.
* Retrieve the selected event's title, description preview, attendees, and source link.
* Search Gmail using:
  * the exact meeting title
  * attendee email addresses
  * extracted title and description keywords
* Search Drive using:
  * extracted title keywords
  * extracted description keywords
* Return only minimal source metadata suitable for validation and later citation.
* Return explicit notes when Gmail or Drive searches find no likely results.

---

## Final Status

Core functionality validated.

---

## Validation Outcome

* OAuth scope upgrade works: PASS
* Only Calendar/Gmail/Drive read-only scopes are requested: PASS
* Calendar event can be used as context seed: PASS
* Gmail search returns related messages: PASS WITH LIMITATIONS
* Drive search returns related files: PASS
* Source metadata is available for later citation: PASS
* No-result cases are handled safely: PASS
* No write permissions are requested: PASS
* No AI provider receives user data: PASS
* No sensitive data is logged: PASS

---

## Manual Validation Performed

Validation intentionally used realistic but synthetic project data.

Performed:

* Created synthetic calendar meetings.
* Created matching Gmail messages.
* Created matching Google Drive documents.
* Successfully collected related context for `Project Alpha Sync`.
* Successfully collected related context for `Architecture Review`.

Successfully demonstrated:

* Google OAuth 2.0 authentication
* Windows-backed secure refresh-token storage
* Authenticated-state restoration after application restart
* Read-only Google scope requests only
* Upcoming Calendar event retrieval
* Selected-event context seeding
* Gmail context retrieval
* Drive context retrieval
* Aggregated context display inside the desktop app

---

## Security Notes

This spike:

* never requests write permissions
* never modifies Google data
* uses Calendar, Gmail, and Drive read-only scopes only
* performs all processing locally

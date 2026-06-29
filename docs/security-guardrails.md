# Security Guardrails

## Purpose

Meeting Prep Assistant is built around trust.

The application processes highly sensitive personal and professional information. Every engineering decision should prioritize user privacy, transparency, and security over convenience.

---

## Security Principles

### Local First

User information should remain on the user's computer whenever possible.

The application should not maintain a central backend containing user meetings, emails, documents, or generated briefings.

---

### Least Privilege

Only request the minimum Google OAuth scopes required for the current version of the product.

Never request write permissions unless a future feature absolutely requires them.

---

### Read-Only Access

Version 1 only reads information.

The application must never:

- Modify calendar events
- Send emails
- Delete files
- Edit Google Drive documents
- Create Google content

---

### Secure Authentication

Authentication must use Google OAuth 2.0.

The application must never ask the user for their Google password.

OAuth refresh tokens must never be stored as plain text.

They should be protected using operating-system-backed secure storage whenever possible.

---

### No Remote User Storage

Meeting data, emails, and document contents must never be stored on developer-controlled servers.

The application should operate directly between:

User Computer ⇄ Google ⇄ AI Provider

No persistent cloud backend.

---

## Data Flow

Meeting Prep Assistant follows a local-first data flow.

Google Calendar
        │
Google Gmail
        │
Google Drive
        │
        ▼
Local Desktop Application
        │
        ▼
Context Collector
        │
        ▼
AI Provider
        │
        ▼
Generated Meeting Brief

No Google data is permanently stored on remote servers controlled by this application.

Only the minimum context required to generate the meeting brief should be sent to the configured AI provider.

---

### Transparent AI

Users should know when AI is being used.

Generated summaries should never appear as unquestionable facts.

Important statements should always reference their original source.

The application should clearly communicate which AI provider is being used to generate the briefing.

---

### Source Traceability

Every generated briefing should allow the user to identify where information originated.

Examples:

- Calendar Event
- Gmail Thread
- Google Drive Document

Users should always be able to inspect the original source before trusting an AI-generated statement.

---

### Privacy Before Features

If a feature requires unnecessary data collection, additional permissions, or weakens user privacy, it should be rejected or redesigned.

---

## Version 1 Permission Scope

Allowed

- Google Calendar (read)
- Gmail (read/search)
- Google Drive (read/search)

Not Allowed

- Writing Google data
- Sending email
- Sharing user information
- Multi-user cloud storage
- Third-party analytics collecting user content

---

## Security Review Rule

Any pull request or significant architectural change that affects:

- OAuth
- Permissions
- Token storage
- AI processing
- Local storage
- External APIs

must review this document and update it if the security model changes.

---

## Fail Safe

If required meeting context cannot be retrieved or verified, the application should clearly communicate what information is unavailable rather than generating, inferring, or guessing missing details.

---

### Safe Logging

Application logs must never contain sensitive user content such as:

- Email bodies
- Document contents
- OAuth tokens
- Meeting summaries
- Personally identifiable information
- API responses containing user content

Logs should contain operational events only.

---

### Security Philosophy

Meeting Prep Assistant handles information that users already trust Google to store.

The application's responsibility is not only to protect that information technically, but also to earn the user's trust through transparency, minimal permissions, and predictable behavior.

When security, privacy, and convenience conflict, security and privacy take priority.
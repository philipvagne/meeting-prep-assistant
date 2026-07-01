# Decisions

## Purpose

This document records significant engineering and product decisions made throughout the project.

A decision should be recorded when it changes one or more of the following:

* Product direction
* Architecture
* Security model
* Engineering workflow
* Technology selection
* Project scope

Minor implementation details should be preserved through commits rather than this document.

---

# Decision 001

**Status:** Accepted

## Decision

Separate validation from implementation.

## Context

The initial roadmap mixed architectural validation with production implementation, creating ambiguity around the purpose of development phases.

## Alternatives Considered

* Combine validation and implementation into a single workflow.
* Separate validation and implementation into distinct phases.

## Reason

Validation should answer technical questions.

Implementation should build production-quality software.

Separating these phases reduces technical risk before committing to the full implementation.

## Consequences

**Positive**

* Clear engineering workflow.
* Reduced architectural uncertainty.
* Cleaner project roadmap.

**Negative**

* Longer planning phase before implementation begins.

---

# Decision 002

**Status:** Accepted

## Decision

Target Windows only for Version 1.

## Context

Supporting multiple operating systems would significantly increase implementation complexity during the MVP.

## Alternatives Considered

* Windows only.
* Windows + macOS.
* Cross-platform from Version 1.

## Reason

Focusing on a single operating system allows the project to validate the product while simplifying packaging, notifications, startup behavior, and installation.

## Consequences

**Positive**

* Smaller implementation scope.
* Easier testing.
* Simpler release process.

**Negative**

* Non-Windows users are excluded from Version 1.

---

# Decision 003

**Status:** Accepted

## Decision

Target Google Workspace for Version 1.

## Context

The product depends on calendar, email, and document integrations.

## Alternatives Considered

* Google Workspace.
* Microsoft 365.
* Support both ecosystems immediately.

## Reason

Google provides a well-integrated ecosystem that allows the product concept to be validated before expanding to additional providers.

## Consequences

**Positive**

* Faster development.
* Lower integration complexity.
* Strong MVP focus.

**Negative**

* Microsoft users must wait for future support.

---

# Decision 004

**Status:** Accepted

## Decision

Adopt a local-first architecture.

## Context

The application processes sensitive calendar events, emails, and documents.

## Alternatives Considered

* Local-first architecture.
* Cloud backend with centralized storage.

## Reason

Keeping user information on the user's computer improves privacy, reduces infrastructure complexity, and supports the project's security philosophy.

## Consequences

**Positive**

* Improved user trust.
* Better privacy.
* No backend infrastructure required.

**Negative**

* Some future collaboration features become more complex.

---

# Decision 005

**Status:** Accepted

## Decision

Version 1 will be read-only.

## Context

The product exists to assist users before meetings, not to modify their workspace.

## Alternatives Considered

* Read-only access.
* Read and write access.

## Reason

Read-only permissions reduce security risks and simplify user trust.

## Consequences

**Positive**

* Lower security risk.
* Simpler permission model.
* Easier user adoption.

**Negative**

* Workflow automation features are postponed.

---

# Decision 006

**Status:** Accepted

## Decision

Native notifications are part of the product, not an optional feature.

## Context

The product promise is reducing the user's cognitive load before meetings.

## Alternatives Considered

* Dashboard-only experience.
* User manually opens the application.
* Native proactive notifications.

## Reason

The notification delivers the product's value at the correct time without requiring the user to remember to prepare.

## Consequences

**Positive**

* Supports the product promise.
* Reduces cognitive load.
* Creates a proactive experience.

**Negative**

* Adds implementation complexity.

---

# Decision 007

**Status:** Accepted

## Decision

AI-generated briefings must include source references.

## Context

Users need confidence that important statements are supported by original information.

## Alternatives Considered

* AI summaries without citations.
* Source-backed summaries.

## Reason

Source references improve transparency, reduce hallucination risk, and allow users to verify important information.

## Consequences

**Positive**

* Increased trust.
* Better transparency.
* Easier verification.

**Negative**

* Additional implementation work.

---

# Decision 008

**Status:** Accepted

## Decision

Do not proceed to implementation until the architecture has been validated.

## Context

The project introduces multiple technical unknowns, including desktop architecture, Google OAuth, notifications, and AI integration.

## Alternatives Considered

* Begin implementation immediately.
* Validate architecture through engineering spikes first.

## Reason

The architecture should be proven practical before investing significant effort into production implementation.

## Consequences

**Positive**

* Lower implementation risk.
* Higher confidence in architectural decisions.
* Better long-term maintainability.

**Negative**

* Delays production development until validation is complete.

## Decision 009 — AI provider strategy

**Status**: Accepted

**Decision**: Use user-configured AI providers for the portfolio version, while keeping the architecture open for a future backend-managed AI service.

**Reason**: avoids developer API cost risk, keeps the app testable by technical users, preserves privacy-first/local-first architecture, and does not block future commercial release.

## Decision 010 – Portfolio AI Strategy

The portfolio version of Meeting Prep Assistant will support user-configured AI providers (initially OpenAI and Gemini). This keeps the application downloadable, privacy-first, and inexpensive to maintain while avoiding embedding developer API keys.

The architecture intentionally isolates AI provider integration behind an abstraction layer so a future backend-managed AI service can replace user-configured providers without major architectural changes.

## Decision 011 — AI Provider Abstraction

The application communicates with AI through a provider abstraction rather than directly with a specific vendor.

The portfolio version initially supports user-configured providers (Gemini and OpenAI). Additional providers can be added by implementing the common AI provider interface.

This design intentionally allows a future backend-managed AI service without requiring changes to the rest of the application.

---

# Decision 012

**Status:** Accepted

## Decision

Organize Phase 1 implementation around desktop product surfaces and UX-focused sprints.

## Context

The validation phase proved the major technical paths, but the original implementation roadmap described Phase 1 only at a very high level. Product specification and product-experience work now provide enough clarity to organize implementation around concrete desktop surfaces instead of generic UI work.

## Alternatives Considered

* Keep Phase 1 as a broad undifferentiated desktop implementation phase.
* Break Phase 1 into product-surface and UX-oriented sprints.

## Reason

The main implementation risk is no longer whether the desktop shell is possible. It is whether the product structure feels calm, predictable, and useful across Home, Upcoming Meetings, Settings, and the separate brief-reading flow. Organizing the work by user-facing surfaces keeps implementation aligned with the product specification and avoids prematurely pulling prompt quality, ranking, and release tasks into Phase 1.

## Consequences

**Positive**

* Clearer implementation sequencing.
* Better alignment with product-specification and product-experience work.
* Cleaner boundaries between UI structure, integration hardening, context quality, and release work.

**Negative**

* Requires more roadmap detail up front.

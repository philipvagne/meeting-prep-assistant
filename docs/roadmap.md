# Roadmap

## Purpose

This roadmap defines the planned development process for Meeting Prep Assistant.

The project is divided into two distinct stages:

* **Validation** — Prove that the proposed architecture is technically practical.
* **Implementation** — Build a production-ready application using the validated architecture.

Separating validation from implementation reduces technical risk while keeping the product focused.

---

# Phase 0 — Project Foundation

## Goal

Prepare the repository, documentation, architecture, and engineering rules before implementation begins.

### Includes

* Product Plan
* Security Guardrails
* Architecture
* Roadmap
* Evaluation Plan
* Spike Documentation
* AGENTS.md
* Prompt Template
* Build Log
* Decisions Log

### Exit Criteria

* Core documentation completed
* Architecture approved
* Security guardrails established
* Development workflow defined

---

# Validation Phase

The purpose of the validation phase is **not** to build the final product.

Each spike exists to answer a technical question and reduce architectural uncertainty before implementation begins.

The code produced during a spike may later be revisited, improved, or rewritten during implementation.

---

## Spike 1 — Desktop Foundation

### Goal

Prove that the application can behave like a professional Windows desktop application.

### Validate

* Tauri application
* System tray
* Background execution
* Windows startup
* Native notifications
* Notification click behavior
* MSIX packaging

### Success Criteria

* Application runs in the background
* Native notifications work
* Clicking a notification opens the application
* Application can be packaged for Windows

---

## Spike 2 — Google Authentication & Calendar

### Goal

Prove that the application can securely authenticate with Google and retrieve upcoming meetings.

### Validate

* Google OAuth 2.0
* Secure token storage
* Calendar access
* Meeting retrieval
* Authentication persistence

### Success Criteria

* User authenticates successfully
* Tokens persist securely
* Upcoming meetings are retrieved
* Meeting metadata is available

### Status

COMPLETE - PASS

---

## Spike 3 — Context Collection

### Goal

Prove that relevant meeting context can be collected from Google Workspace.

### Validate

* Gmail search
* Google Drive search
* Context selection
* Metadata extraction
* Basic relevance filtering

### Success Criteria

* Related emails are found
* Related Drive documents are found
* Context is relevant to the meeting
* Sources can be referenced later

### Status

CORE FUNCTIONALITY VALIDATED

---

## Spike 4 — Meeting Brief Generation

### Goal

Prove that useful meeting briefings can be generated from collected context.

### Validate

* Prompt Builder
* AI Provider integration
* Brief generation
* Source references
* Fail-safe behavior

### Success Criteria

* Brief is concise
* Brief contains citations
* Unsupported information is not generated
* Missing context is communicated clearly

### Status

VALIDATED - PASS WITH LIMITATIONS

---

# Decision Gate

## Question

Can the validated architecture support the Product Plan?

### Possible Outcomes

**PASS**

The architecture is validated.

Proceed to the implementation phase.

**PASS WITH CHANGES**

The architecture is viable but requires adjustments.

Update the architecture and documentation before implementation.

**FAIL**

The architecture cannot reliably support the product.

Resolve architectural issues before implementation continues.

---

# Implementation Phase

Once the architecture has been validated, implementation begins.

Unlike the validation phase, implementation focuses on building a polished, maintainable, production-ready application.

Implementation phases may revisit, improve, or replace code produced during validation.

Validation is now complete.

The active phase is `Phase 1 - Desktop Application`.

---

## Phase 1 — Desktop Application

### Goal

Build the production desktop application.

### Status

ACTIVE

### Phase Focus

Phase 1 focuses on desktop product structure, application surfaces, and calm predictable UX.

This phase is informed by:

* `docs/research/product-specification-v1.md`
* `docs/research/product-experience.md`
* early wireframes and Figma structure drafts used to validate layout and flow, not final visual design

Phase 1 intentionally does not own:

* final AI prompt quality
* advanced context ranking
* semantic search improvements
* release packaging
* installer polish

### Product UI Shell

Includes:

* Main application layout
* Permanent header
* Permanent navigation/sidebar
* Universal search placement
* Active content area
* Stable layout across Home, Upcoming Meetings, and Settings

### Home / Generated Briefs Inbox

Includes:

* Generated brief list
* Brief cards
* Open brief behavior
* Manual delete from brief card menu
* Auto-delete lifecycle
* Empty state
* Internal scrolling

### Upcoming Meetings

Includes:

* Upcoming meeting list
* Generate brief manually
* Generation status
* Brief-ready handoff back to Home
* Empty state

### Settings Foundation

Includes:

* Google account section
* AI provider/model section
* API key setup for portfolio/V1 mode
* Notification timing
* Brief auto-delete timing
* Advanced section for technical settings

### Brief Window

Includes:

* Separate native window
* Meeting title, time, and description
* AI-generated bullet-point brief
* Source reference links
* Optional attendees section
* User notes section
* Hidden technical details/disclosure
* Copy brief action

### Notes Window / Notes Interaction

Includes:

* Add note flow
* Save/cancel
* Notes belong to a specific generated brief
* Notes should not clutter the default reading experience

### Window Management

Includes:

* Main app window
* Separate brief window
* Optional notes popup/window
* Tray restore behavior
* Notification click behavior

### Shared UX / States

Includes:

* Error messages that remain visible until dismissed
* Loading states
* Empty states
* Minimal, calm, non-flashy UI behavior
* Stable layout and predictable navigation

### Sprint Breakdown

#### Sprint 1.1 — Application Shell

Status:

* ACTIVE

Focus:

* Establish the stable desktop shell
* Implement the main layout regions
* Define the permanent header, sidebar, and active content area

#### Sprint 1.2 — Navigation + Universal Search Shell

Focus:

* Implement navigation between Home, Upcoming Meetings, and Settings
* Place universal search in the shell
* Keep layout behavior stable while screens change

#### Sprint 1.3 — Home / Generated Briefs Inbox

Focus:

* Build the generated brief list and brief cards
* Implement open-brief behavior
* Add empty state, internal scrolling, delete action, and auto-delete lifecycle hooks

#### Sprint 1.4 — Upcoming Meetings

Focus:

* Build the upcoming meeting list
* Support manual brief generation initiation
* Show generation status and handoff completed briefs back to Home

#### Sprint 1.5 — Settings Foundation

Focus:

* Build Google account and AI provider sections
* Support provider/model selection and portfolio API key setup
* Add notification timing and brief auto-delete timing settings
* Keep advanced technical settings behind an advanced section

#### Sprint 1.6 — Brief Window

Focus:

* Build the separate brief window
* Present the meeting header, generated brief bullets, source links, and optional attendees
* Add copy brief and hidden technical details/disclosure

#### Sprint 1.7 — Notes Interaction

Focus:

* Add user notes tied to an individual generated brief
* Support add, save, and cancel flows
* Keep notes out of the default reading path

#### Sprint 1.8 — Window Management + Tray Flow

Focus:

* Refine coordination between the main window, brief window, and optional notes popup
* Complete tray restore behavior and notification-click behavior for the implementation design

#### Sprint 1.9 — Phase 1 UX Polish Pass

Focus:

* Review calmness, clarity, spacing, navigation consistency, and state handling
* Tighten visible error, loading, and empty states across all Phase 1 surfaces

---

## Phase 2 — Google Integration

### Goal

Build the complete Google Workspace integration.

Includes:

* OAuth hardening
* Calendar integration hardening
* Gmail integration hardening
* Google Drive integration hardening
* Token lifecycle hardening
* Secure error handling and recovery
* Production cleanup of validation-era integration edges

---

## Phase 3 — Context Engine

### Goal

Build the production context collection system.

Includes:

* Context Collector
* Search improvements
* Relevance filtering
* Source management
* Performance improvements
* Advanced context ranking
* Semantic search and retrieval improvements

---

## Phase 4 — Meeting Brief Experience

### Goal

Build the complete user experience around meeting preparation.

Includes:

* Prompt Builder
* Brief Generator
* Source references
* Brief presentation
* Notification experience
* Error states
* Prompt and brief quality iteration
* Confidence and uncertainty presentation refinement

---

## Phase 5 — Polish & Release

### Goal

Prepare Version 1 for public release.

Includes:

* Testing
* Bug fixes
* Performance improvements
* Security review
* Documentation review
* Installer
* MSIX packaging
* Release preparation

---

# Version 1 Non-Goals

The following are intentionally excluded from Version 1:

* Microsoft Outlook / Teams support
* Slack integration
* Zoom integration
* Local file scanning
* Multi-user accounts
* Cloud backend
* Web dashboard as the primary experience
* Automatic modification of Google Workspace data
* Long-form meeting summaries
* Multiple AI providers
* Mobile application

---

# Roadmap Rule

The roadmap should only change when the overall project direction changes.

Implementation details belong in the spike documents, architecture documentation, or build log—not in the roadmap.

Future ideas should be documented separately rather than expanding the active Version 1 scope.

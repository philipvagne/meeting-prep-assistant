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

---

## Phase 1 — Desktop Application

### Goal

Build the production desktop application.

Includes:

* Final desktop architecture
* Production UI
* System tray experience
* Startup behavior
* Window management
* Settings foundation

---

## Phase 2 — Google Integration

### Goal

Build the complete Google Workspace integration.

Includes:

* OAuth
* Calendar
* Gmail
* Google Drive
* Token management
* Error handling

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

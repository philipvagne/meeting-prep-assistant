# Architecture

## Overview

Meeting Prep Assistant is a local-first Windows desktop application that automatically prepares users for upcoming meetings by gathering relevant context from Google Workspace services and generating a concise AI-powered meeting brief.

The application is designed around background automation, minimal user interaction, and privacy-first principles. Users should be able to focus on their work while the application prepares relevant meeting context automatically.

---

## Architecture Rule

This document describes the intended system architecture.

If implementation changes the architecture, component responsibilities, or data flow, this document must be reviewed and updated before the change is merged.

---

# High-Level Architecture

```text
                    Windows Desktop
                           │
                           ▼
                    Tauri Application
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   React UI        Meeting Scheduler       SQLite
                           │
                           ▼
                Google Authentication
                           │
                           ▼
                   Google Services
          ┌───────────────┼───────────────┐
          ▼               ▼               ▼
     Calendar          Gmail           Drive
                           │
                           ▼
                   Context Collector
                           │
                           ▼
                     Prompt Builder
                           │
                           ▼
                      AI Provider
                           │
                           ▼
                     Meeting Brief
                           │
                           ▼
             Native Windows Notification
```

---

# Core Components

## Tauri Application

Responsible for:

* Desktop lifecycle
* System tray
* Startup with Windows
* Native notifications
* Application packaging

---

## React UI

Responsible for:

* User authentication flow
* Settings
* Meeting brief presentation
* Future meeting history

---

## Meeting Scheduler

Responsible for:

* Running in the background
* Detecting upcoming meetings
* Scheduling brief generation
* Triggering notifications
* Preventing duplicate briefing generation

---

## Google Authentication

Responsible for:

* OAuth 2.0 authentication
* Token lifecycle
* Secure authentication flow


---

## Google Services

Responsible for:

* Calendar access
* Gmail access
* Google Drive access

---

## Context Collector

Responsible for:

* Reading meeting context
* Selecting relevant emails
* Selecting relevant documents
* Preparing the minimum required context for AI summarization

---

## Prompt Builder

Responsible for:

* Constructing AI prompts
* Formatting context
* Enforcing prompt structure

---

## Brief Generator

Responsible for:

* Generating meeting summaries
* Attaching source references
* Formatting the final meeting brief

---

## SQLite

Responsible for:

* Application settings
* OAuth metadata
* Generated briefing metadata
* Application state

---

# Data Flow

```text
Meeting Scheduler
        │
        ▼
Upcoming Meeting Detected
        │
        ▼
Retrieve Calendar Details
        │
        ▼
Search Gmail
        │
        ▼
Search Google Drive
        │
        ▼
Context Collector
        │
        ▼
Prompt Builder
        │
        ▼
AI Provider
        │
        ▼
Meeting Brief
        │
        ▼
Native Notification
        │
        ▼
User Opens Brief
```

---

# Component Responsibilities

| Component             | Responsibility                  |
| --------------------- | ------------------------------- |
| Tauri                 | Desktop shell                   |
| React                 | User interface                  |
| Meeting Scheduler     | Background automation           |
| Google Authentication | OAuth & token lifecycle         |
| Google Services       | Calendar, Gmail & Drive access  |
| Context Collector     | Gather relevant meeting context |
| Prompt Builder        | Prepare AI prompt               |
| Brief Generator       | Generate source-backed briefing |
| AI Provider           | AI inference                    |
| SQLite                | Local persistence               |

---

# Component Interaction Rules

* The React UI must never communicate directly with Google APIs.
* Google Services must never communicate directly with the UI.
* The Meeting Scheduler is responsible for determining when briefing begins.
* The Context Collector is the only component responsible for assembling meeting context.
* The Prompt Builder is the only component responsible for constructing AI prompts.
* The Brief Generator is the only component responsible for generating the final briefing.
* Native notifications are only sent after a meeting brief has been successfully generated.

---

# Architecture Principles

* Local-first
* Read-only
* Background-first
* Modular components
* Single responsibility
* Source-backed AI
* Security before convenience

---

# Potential Extension Points

Future integrations may include:

* Microsoft Graph
* Slack
* Microsoft Teams
* Zoom
* Local file system
* Additional AI providers

These integrations should extend the existing architecture rather than replace core components.

---

# Out of Scope

The architecture intentionally excludes:

* Cloud backend
* Multi-user synchronization
* Shared databases
* Browser-based version
* Automatic modification of Google Workspace data

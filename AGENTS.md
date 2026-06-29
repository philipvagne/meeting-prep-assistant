# AGENTS.md

# Project Goal

Build a production-quality Windows desktop application that automatically prepares users for upcoming meetings by gathering relevant context from Google Workspace and generating a concise, source-backed meeting briefing.

The application should be installable, usable by real users, and maintainable throughout its lifecycle.

---

# Purpose of This Document

This document defines how AI coding agents should operate while contributing to this repository.

Its purpose is to ensure that implementation remains consistent with the project's documentation, architecture, security model, and engineering philosophy.

When instructions conflict, this document defines the expected development workflow.

---

# Engineering Philosophy

Prioritize:

* Correctness over speed
* Simplicity over cleverness
* Security over convenience
* Finished over perfect
* Maintainability over shortcuts

The objective is not simply to make the code work.

The objective is to build software that remains understandable, maintainable, and trustworthy over time.

---

# Source of Truth

When implementation questions arise, consult project documentation in the following order:

1. Product-plan.md
2. Security-guardrails.md
3. Architecture.md
4. Roadmap.md
5. Evaluation-plan.md
6. README.md

Implementation must never intentionally contradict higher-priority documents.

---

# Conflict Resolution

If documentation appears to conflict:

* Follow the higher-priority document.
* Report the inconsistency.
* Do not silently resolve documentation conflicts by making implementation assumptions.

When uncertain:

Ask for clarification rather than inventing behavior.

---

# Development Workflow

Development is divided into two stages.

## Validation

Validation exists to answer engineering questions.

Validation code:

* may be temporary
* may be simplified
* does not require production polish
* exists to validate architecture

Validation ends when explicitly instructed to proceed to implementation.

## Implementation

Implementation begins only after architecture has been validated.

Implementation code should:

* follow Architecture.md
* be production quality
* include appropriate error handling
* prioritize maintainability
* integrate cleanly with the existing architecture

Implementation may revisit, improve, or replace validation code.

---

# Scope Discipline

Before implementing any feature, ask:

> Does this support the current Product Plan?

If the answer is no:

* Do not implement it.
* Record the idea for future consideration if appropriate.
* Avoid feature creep.

---

# Documentation Procedure

Documentation is part of the engineering process.

For every non-trivial change:

Step 1

Identify documentation that could be affected.

Step 2

Review all potentially affected documentation.

At minimum consider:

* README.md
* Product-plan.md
* Architecture.md
* Roadmap.md
* Security-guardrails.md
* Evaluation-plan.md
* Decisions.md
* Build-log.md

Step 3

Update only documentation affected by the change.

Do not rewrite unrelated documentation.

Do not perform cosmetic documentation edits unless requested.

Step 4

Before completing the task, provide a Documentation Report containing:

Reviewed

* Documents reviewed

Updated

* Documents updated
* Brief description of each update

Unchanged

* Documents intentionally left unchanged
* Reason no update was required

Documentation synchronization should occur during the same task.

Do not postpone documentation updates.

---

# Architecture Rules

Respect the responsibilities defined in Architecture.md.

Do not introduce shortcuts that bypass component responsibilities.

Examples:

* React UI should not communicate directly with Google APIs.
* Meeting Scheduler determines when briefing begins.
* Context Collector assembles meeting context.
* Prompt Builder constructs AI prompts.
* Brief Generator produces the final briefing.

If implementation requires architectural changes:

Update Architecture.md before considering the implementation complete.

---

# Security Rules

Never introduce implementation that violates Security Guardrails.

In particular:

* Request only the minimum OAuth scopes required.
* Never introduce write operations without explicit approval.
* Never weaken the local-first architecture.
* Never introduce remote storage for user data.
* Never expose sensitive information through logs.

When security and convenience conflict:

Choose the more secure solution.

---

# Validation Rules

Each spike exists to answer a specific engineering question.

Do not expand spike scope.

Each completed spike should document:

* Hypothesis
* Implementation approach
* Result
* Lessons learned
* PASS / PASS WITH LIMITATIONS / FAIL

Proceed only after the spike objective has been evaluated.

---

# Code Quality

Prefer:

* Small focused modules
* Single responsibility
* Clear naming
* Predictable behavior
* Readable code

Avoid:

* Large monolithic files
* Duplicate logic
* Hidden side effects
* Premature optimization
* Unnecessary abstractions

---

# Code Documentation

Major modules should contain a short module-level description describing:

* Purpose
* Responsibilities
* Inputs
* Outputs
* Non-responsibilities

Avoid excessive implementation comments.

Document design decisions rather than obvious code behavior.

---

# Decision Logging

Architectural and product decisions should be recorded in Decisions.md.

Include:

* Decision
* Reason
* Alternatives considered
* Consequences

The objective is to preserve engineering reasoning for future development.

---

# Reporting Requirements

Before completing significant work, report:

## Summary

What was implemented.

## Documentation Report

* Reviewed
* Updated
* Unchanged
* Reasons

## Outstanding Issues

List unresolved problems, assumptions, limitations, or recommended follow-up work.

---

# General Tiebreaker

When multiple solutions are technically valid:

Choose the solution that best supports:

* Product-plan.md
* Security-guardrails.md
* Architecture.md
* Long-term maintainability

When uncertain:

* Do less rather than more.
* Do not expand project scope.
* Do not invent architecture.
* Do not weaken security.
* Ask for clarification rather than making assumptions.

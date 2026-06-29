# Prompt Template

## Purpose

This document defines the preferred prompt structure for AI-assisted development throughout the project.

The goal is to provide Codex with enough project context to perform focused engineering work while avoiding unnecessary scope expansion.

The template should be adapted to the task rather than followed mechanically.

---

# General Prompt Structure

A well-formed implementation prompt should include the following sections:

1. Context
2. Current Phase
3. Objective
4. Scope
5. Out of Scope
6. Constraints
7. Expected Deliverables
8. Documentation Requirements
9. Testing Requirements
10. Exit Criteria
11. Reporting Requirements

---

# Standard Implementation Prompt

## Context

Read before making changes:

* AGENTS.md
* README.md
* Relevant architecture documents
* Relevant source files
* Any documentation related to the requested feature

---

## Current Phase

Clearly identify the current project phase or validation spike.

Example:

Current Phase:

Spike 2 — Google Authentication

---

## Objective

Describe the engineering goal in one or two sentences.

Focus on the problem being solved rather than implementation details.

---

## Scope

List exactly what should be implemented.

Example:

Implement:

* Google OAuth login
* Secure token storage
* Calendar retrieval

---

## Out of Scope

Explicitly list work that must not be performed.

Example:

Do NOT implement:

* Gmail integration
* Google Drive
* UI redesign
* Additional providers

---

## Constraints

Include any architectural or security constraints.

Examples:

* Respect Architecture.md.
* Follow Security Guardrails.
* Maintain local-first architecture.
* Do not request additional OAuth scopes.
* Do not weaken existing behavior.

---

## Documentation Requirements

Apply the Documentation Procedure defined in AGENTS.md.

Update only documentation affected by the task.

Do not rewrite unrelated documentation.

---

## Testing Requirements

List expected verification steps.

Examples:

* Build succeeds.
* Existing functionality remains operational.
* New functionality behaves as expected.

---

## Exit Criteria

Define what success looks like.

Example:

* OAuth succeeds.
* Tokens persist securely.
* Calendar events are retrieved.
* Existing functionality remains unchanged.

---

## Reporting Requirements

When complete, report:

1. Files changed
2. What was implemented
3. What was intentionally not implemented
4. Documentation reviewed
5. Documentation updated
6. Documentation intentionally left unchanged
7. Testing performed
8. Exit criteria status
9. Limitations or follow-up recommendations

---

# Prompt Writing Guidelines

Good prompts:

* Clearly define the objective.
* Keep scope narrow.
* Explicitly state non-goals.
* Define success criteria.
* Reference project documentation.
* Avoid ambiguity.

Avoid prompts that:

* Combine unrelated features.
* Expand project scope.
* Leave architectural decisions unspecified.
* Omit testing or documentation expectations.

---

# Project Principle

Each prompt should represent a single engineering task.

If multiple independent tasks are identified, prefer multiple focused prompts rather than one large prompt.

## Prompt Rule

Each prompt should pursue one primary engineering objective.

Supporting context, constraints, testing, documentation, and reporting may be extensive, but the implementation goal should remain singular and well-defined.

If a prompt requires multiple unrelated objectives, split it into multiple prompts.
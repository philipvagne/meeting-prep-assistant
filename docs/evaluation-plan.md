# Evaluation Plan

## Purpose

This document defines how Meeting Prep Assistant will be evaluated throughout development.

The objective is to validate both the technical architecture and the finished product using measurable success criteria.

The project is evaluated in two stages:

* Validation Evaluation
* Product Evaluation

---

# Validation Evaluation

Each validation spike must end with a documented result.

Possible outcomes:

* PASS
* PASS WITH LIMITATIONS
* FAIL

A spike is considered complete only after its success criteria have been evaluated.

---

## Spike 1 — Desktop Foundation

Question:

Can the application behave like a professional Windows desktop application?

Evaluate:

* Starts successfully
* Runs in the background
* System tray functions correctly
* Native notifications work
* Clicking notifications opens the application
* MSIX packaging succeeds

Result:

PASS / PASS WITH LIMITATIONS / FAIL

---

## Spike 2 — Google Authentication

Question:

Can the application securely authenticate and retrieve upcoming meetings?

Evaluate:

* OAuth flow succeeds
* Tokens persist securely
* Calendar events are retrieved
* Meeting metadata is correct

Result:

PASS / PASS WITH LIMITATIONS / FAIL

---

## Spike 3 — Context Collection

Question:

Can useful meeting context be gathered automatically?

Evaluate:

* Gmail search returns relevant results
* Drive search returns relevant results
* Context is relevant to the meeting
* Sources can be identified

Result:

PASS / PASS WITH LIMITATIONS / FAIL

---

## Spike 4 — Meeting Brief

Question:

Can useful meeting briefings be generated?

Evaluate:

* Brief is generated successfully
* Brief is concise
* Sources are referenced
* Missing information is communicated
* No unsupported information is presented as fact

Result:

PASS / PASS WITH LIMITATIONS / FAIL

---

# Decision Gate Evaluation

Question:

Does the validated architecture support the Product Plan?

Possible outcomes:

PASS

Proceed to implementation.

PASS WITH CHANGES

Update architecture before implementation.

FAIL

Resolve architectural issues before implementation continues.

---

# Product Evaluation

After implementation, the complete application should be evaluated as an end-to-end workflow.

## Functional Evaluation

The application should:

* Start with Windows
* Run in the background
* Detect upcoming meetings
* Retrieve Calendar data
* Retrieve Gmail context
* Retrieve Drive context
* Generate a meeting brief
* Display a native notification
* Open the brief from the notification

---

## User Experience Evaluation

The application should:

* Require minimal interaction
* Stay out of the user's way
* Deliver the briefing at the correct time
* Present information clearly
* Reduce preparation effort

---

## Security Evaluation

Verify that:

* Only approved OAuth scopes are requested
* No write operations occur
* No sensitive information is stored remotely
* Tokens are stored securely
* Logging follows Security Guardrails

---

## Success Criteria

Version 1 is considered successful when:

* All validation spikes have passed.
* The implementation satisfies the Product Plan.
* The application follows the Security Guardrails.
* The complete meeting preparation workflow functions from calendar detection to briefing delivery.
* The application can be installed and used by a real user with a Google Workspace account.

---

## Evaluation Rule

Significant architectural or functional changes must update this document if they change how success is measured.

# Product Plan

## Product Name
Meeting Prep Assistant

## Product Promise
The user should not have to remember to prepare for meetings. The app should automatically deliver the right context 
at the right time, allowing the user to stay focused on their work until the meeting begins

## Problem
People waste time and mental energy before meetings reconstructing context from calendar events, emails, and documents.

## Target User
People who regularly attend meetings and rely on Google Calendar, Gmail, and Google Drive to manage their work.

## MVP
A Windows desktop app that:
- Starts with Windows
- Runs in the background
- Connects to Google via OAuth
- Reads upcoming Google Calendar events
- Searches related Gmail threads
- Searches related Google Drive documents
- Generates a short source-backed meeting brief
- Sends a native notification 15–20 minutes before the meeting
- Opens the brief when the notification is clicked

The application does not require the user to manually prepare or trigger the briefing process.

## Non-Goals for v1
- Microsoft ecosystem support
- Slack support
- Zoom transcript support
- Local file scanning
- Multi-user/team accounts
- Web dashboard as the main experience
- Long meeting summaries
- Writing or modifying Google data
- Multiple AI providers

## Core Principle

### Reduce Cognitive Load:
The application should remove mental overhead, not create more of it.

### Background First:
The application should stay out of the user's way and become visible only when it provides value.

### Trust Through Transparency:
Every generated briefing should be traceable back to its original source.
No "black box" AI.

### Local First:
User data should remain on the user's machine whenever possible.
Cloud services should only be used when required (Google APIs and AI provider).

## Product Pillars
• Background automation
• Trusted AI summaries
• Minimal user interaction
• Privacy-first architecture

## Success Definition
The app is successful if a user can work normally, receive a meeting-prep notification before an upcoming meeting, click it, and read a concise brief with relevant context and sources without manually gathering that information themselves.
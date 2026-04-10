# Poro Product Specification

**Version**: April 2026  
**Product**: Poro Desktop  
**Category**: Desktop AI coding workspace  
**Core Promise**: A beautiful, open-source desktop workspace for agentic coding that gives users Codex-style power with better visibility, calmer UX, and bring-your-own-provider flexibility.

## 1. Product Direction

Poro is no longer a chat subscription product.

Poro is now a **desktop UI platform for AI-assisted software work**. The product focus is the interface layer: session visibility, tool execution awareness, diff review, permission clarity, and local-first workflows.

The agent runtime is not the product moat. The moat is:

- Product taste
- UX clarity
- Trust and transparency
- Lower cost of entry
- Better support for long-running coding sessions

## 2. Target Users

### Primary users

- Indie developers
- Students learning to code with AI assistance
- Open-source maintainers
- Power users who already have API keys or local models

### Secondary users

- Design-minded engineers who care about workflow quality
- Users priced out of premium AI coding products
- Developers who want a local-first alternative to terminal-only tools

## 3. Product Positioning

Poro should not be described publicly as a "Codex clone."

Preferred framing:

- A desktop AI coding workspace
- A local-first UI for agentic development
- A bring-your-own-backend interface for `claw-code`
- A calmer, more transparent alternative to chat-heavy coding tools

## 4. Non-Goals

The MVP should **not** try to be:

- A hosted inference platform
- A payment or crypto product
- A general-purpose chat app
- A mobile app
- A team collaboration suite
- A multi-agent orchestration platform
- An App Store-first release

## 5. Product Principles

### 5.1 The transcript is not the whole product

The conversation view matters, but it is only one part of the experience. Users also need:

- Tool activity
- Permission state
- Diffs
- Session status
- Cost and model visibility
- Workspace context

### 5.2 Local-first by default

Users should be able to install Poro, point it at a repo, configure their provider, and work without creating an account.

### 5.3 Human in control

Poro should make risky actions visible and understandable. The UI should make it easy to:

- See what the agent is doing
- Understand current permissions
- Inspect changes before accepting them
- Resume or stop sessions cleanly

### 5.4 Beauty is part of the product

Poro should feel intentional, calm, and premium. The app should not look like a generic terminal wrapper. The design language should emphasize:

- Strong typography
- Clear visual hierarchy
- Spacious, readable layouts
- Elegant motion
- Confidence without noise

### 5.5 Backend swappable, UI opinionated

The runtime layer may evolve, but the UI should stay strongly opinionated. Poro is the interface for serious AI-assisted work, not a thin shell over command output.

## 6. Technical Direction

### 6.1 Frontend

- **Framework**: SvelteKit + TypeScript
- **Styling**: Tailwind CSS
- **Desktop shell**: Tauri 2
- **Reference implementation**: reuse the desktop setup patterns from `job_raptor`

### 6.2 Runtime layer

- **Primary backend**: `claw-code`
- **Default local path**: Ollama + Gemma 4
- Poro should **integrate with** `claw-code`, not reimplement it
- `claw-code` handles:
  - provider routing
  - tool execution
  - permission modes
  - session persistence
  - prompt execution

### 6.3 Desktop responsibilities

The Tauri layer should handle:

- launching and supervising the local `claw` process
- file and workspace selection
- local settings persistence
- access to OS dialogs and filesystem APIs
- packaging and native distribution

### 6.4 Local persistence

For MVP, persistence is local only:

- local app settings
- recent workspaces
- recent sessions
- local UI preferences

Cloud sync is explicitly out of scope for the initial build.

## 7. MVP Scope

The MVP is a **single-user desktop app** that lets a developer do real work through a local `claw-code` session without living in Terminal.

### 7.1 Core MVP features

- Workspace picker
- Session list for the current workspace
- Main session view
- Model and provider controls
- Permission mode controls
- Live transcript view
- Live tool activity timeline
- Diff / review panel
- Resume previous local sessions
- Settings for backend path and provider configuration

### 7.2 Out of scope for MVP

- Accounts
- Cloud sync
- Team workspaces
- Billing
- Plugin marketplace
- Mobile clients
- Advanced analytics
- Hosted execution
- Multiple backends beyond what is needed to support `claw-code`

## 8. Key Screens

### 8.1 Workspace picker

Purpose:

- choose a local repo
- reopen a recent project
- show first-run guidance

### 8.2 Main session view

Purpose:

- read and send prompts
- watch tool execution
- track status and cost
- understand what the agent is doing right now

Recommended structure:

- transcript area
- tool/event timeline
- composer / command input
- status bar with model, permissions, cwd, session state

### 8.3 Diff and review panel

Purpose:

- inspect changed files
- review before accepting risky edits
- keep code awareness visible, not buried

### 8.4 Settings

Purpose:

- configure backend binary path
- configure provider env vars or instructions
- set defaults for model and permissions
- manage local behavior and UX preferences

## 9. Core User Flows

### 9.1 First launch

1. User opens Poro
2. User picks a workspace
3. Poro checks backend availability
4. Poro checks whether Ollama is reachable and whether `gemma4:e2b` is installed
5. Poro guides the user through local runtime setup if needed
5. User starts a session

### 9.2 Run a coding task

1. User opens a workspace
2. User selects model and permission mode
3. User enters a prompt
4. Poro launches or resumes a `claw` session
5. User watches transcript, tools, and status updates
6. User inspects the produced diff
7. User decides whether to continue, refine, or stop

### 9.3 Resume previous work

1. User reopens the app
2. User selects a recent workspace
3. Poro shows recent sessions
4. User resumes the last session or starts a new one

## 10. Success Criteria For MVP

The MVP is successful when a user can:

- open a repo
- start a real `claw-code` session from the UI
- watch live tool activity
- inspect file changes
- understand current permission mode
- resume the session later

## 11. Quality Bar

Poro should feel:

- trustworthy
- visually polished
- faster to understand than a terminal transcript
- stable enough for daily use on personal projects

The MVP does not need to be feature-complete. It does need to feel coherent.

## 12. After MVP

Potential post-MVP expansions:

- direct signed and notarized distribution
- official binary releases
- local analytics and usage summaries
- optional cloud sync
- encrypted backup of session history
- team and shared workspace features
- premium support or managed convenience features

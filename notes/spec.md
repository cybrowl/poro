# Poro Product Specification

**Version**: April 2026  
**Product**: Poro Desktop  
**Category**: Desktop coding workspace  
**Core Promise**: A calm, trustworthy desktop workspace for running real AI coding sessions through Harness with visible state, clear permissions, and bring-your-own-provider flexibility.

## 1. Product Direction

Poro is no longer a crypto product, payment flow, or general chat app.

Poro is now a **desktop UI for serious AI-assisted software work**. The product is the experience of working with an agent:

- understanding what it is doing
- trusting when it is safe to let it continue
- seeing progress, blockers, and verification clearly
- keeping the workspace and code changes visible

The current stack is:

- Poro desktop app
- sibling `harness` repo as the coding runtime
- local Ollama by default
- optional hosted providers through Harness

## 2. Poro and Harness

Poro and Harness are separate on purpose.

- **Poro** is the desktop product: workspace selection, session surfaces, settings, runtime presentation, and UX
- **Harness** is the controller/runtime: mission state, verification, tool use, recovery, provider routing, and session persistence

The boundary matters for both product quality and privacy. Poro should stay opinionated about the interface, while the real runtime logic can remain private and evolve independently.

## 3. Target Users

### Primary users

- Indie developers
- Design-minded engineers
- Power users who already have local models or API keys
- Developers who want a calmer alternative to terminal-heavy agent tools

### Secondary users

- Students learning through AI-assisted coding
- Open-source maintainers
- Cost-conscious users who want control over providers

## 4. Positioning

Preferred framing:

- a desktop AI coding workspace
- a local-first interface for Harness-backed coding sessions
- a calmer, more transparent alternative to chat-heavy or terminal-only coding tools

Avoid framing Poro as:

- a model company
- a generic chat UI
- a crypto product
- a cheap clone of another desktop coding app

## 5. Product Principles

### 5.1 Visible state is a core feature

The transcript is not enough. The product should surface:

- current runtime activity
- progress and blockers
- permission mode
- verification status
- changed files and review context

### 5.2 Local-first by default

The best first-run experience is:

1. open a repo
2. use local Ollama + Gemma
3. start working without an account

Hosted providers are important, but the local path should remain the easiest default.

### 5.3 Human stays in control

The UI should make it easy to understand:

- what is happening now
- what has already happened
- what changed
- whether the agent is waiting, working, blocked, or verifying

### 5.4 Beauty is part of trust

Poro should feel calm, deliberate, and premium. The app should not look like a terminal transcript wrapped in chrome.

### 5.5 Strong boundary, flexible future

The product should keep a clean interface/runtime boundary so that:

- local development stays fast
- a sidecar boundary is possible later
- a hosted backend is possible later
- the UI does not become tightly coupled to runtime internals

## 6. Technical Direction

### 6.1 Frontend

- **Framework**: SvelteKit + TypeScript
- **Styling**: Tailwind CSS
- **Desktop shell**: Tauri 2

### 6.2 Runtime integration

- **Current runtime**: sibling `harness` repo
- **Desktop bridge**: `harness-server` launched through the Tauri layer
- **Default local path**: Ollama + `gemma4:e2b`
- **Current hosted options**: xAI / Grok and Anthropic via Harness

### 6.3 Desktop responsibilities

Poro should own:

- workspace selection
- runtime launch and stop flows
- health checks
- settings and provider/model defaults
- transcript and activity presentation
- review and diff presentation
- recent workspaces and session selection

### 6.4 Runtime responsibilities

Harness should own:

- mission state
- tool execution
- permission enforcement
- verification
- provider routing
- session storage and replay

## 7. Current Product State

The app already has:

- desktop workspace shell
- provider/model/permission controls
- workspace picker
- local settings
- Harness-backed runtime launch
- session resume/loading
- transcript view
- runtime activity view
- backend health checks

The next product gap is not raw integration anymore. It is **trustworthy session UX**:

- clearer progress
- cleaner action visibility
- stronger review surfaces
- better approval and verification presentation

## 8. Next Milestone

The next milestone is a UI quality milestone, not a runtime plumbing milestone.

Poro should become noticeably better at:

- showing what the agent is doing right now
- summarizing long runs without drowning the user in logs
- surfacing progress, blockers, and verification clearly
- making the desktop session feel like a workspace, not a console transcript

## 9. Non-Goals

Poro should not currently try to be:

- a hosted inference platform
- a team collaboration suite
- a mobile app
- a plugin marketplace
- a general-purpose chatbot
- a Mac App Store-first release

## 10. Success Criteria

The current phase is successful when a user can:

- open a repo
- start a real Harness-backed session
- understand current runtime state without guessing
- review changes with confidence
- resume the session later
- prefer the app experience over running the same workflow directly in terminal

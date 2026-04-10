# Poro Integration Notes

**Version**: April 2026

## 1. Core Integration Decision

Poro should use `claw-code` as the execution engine.

Poro should **not** rebuild:

- provider routing
- tool execution
- permission handling
- session storage
- prompt execution semantics

The desktop app should focus on orchestration, presentation, and UX.

## 2. Why `claw-code`

`claw-code` already provides:

- prompt mode
- interactive sessions
- session resume
- permission modes
- provider routing
- OpenAI-compatible endpoints
- local model support

This makes it a strong backend layer for a UI-first product.

## 3. Integration Strategy

### 3.1 Poro responsibilities

Poro should handle:

- workspace selection
- binary discovery and setup UX
- environment configuration guidance
- session launch and supervision
- live presentation of transcript and events
- diff and review experience
- recent workspaces and local app preferences

### 3.2 `claw-code` responsibilities

`claw-code` should handle:

- actual agent loop execution
- model/provider calls
- tool selection and execution
- local session persistence
- permission enforcement

## 4. Recommended Runtime Boundary

The Tauri backend should expose a thin set of commands to the frontend:

- select workspace
- check backend health
- launch session
- resume session
- stop session
- stream session output
- fetch recent local session metadata

This keeps the Svelte frontend clean and avoids pushing process management into browser code.

## 5. First Integration Targets

### 5.1 Health check

Poro should support a first-run diagnostic flow that mirrors the existing `claw` doctor mindset.

Goal:

- verify the binary exists
- verify the runtime can start
- verify provider credentials are likely configured

### 5.2 Session launch

Poro needs to launch a real session with:

- selected workspace
- selected model
- selected permission mode
- optional environment configuration

### 5.3 Session resume

Poro should list and reopen recent local sessions per workspace.

### 5.4 Output handling

Prefer structured output when possible. Avoid building the product around fragile parsing of human-formatted terminal text.

## 6. Desktop Stack Direction

Use `job_raptor` as the reference pattern for:

- SvelteKit layout
- Tailwind setup
- Tauri 2 packaging
- local plugin and desktop configuration

For MVP, only add desktop dependencies that directly support the core workflow.

## 7. Local Data Model

For MVP, Poro should store only local app metadata such as:

- recent workspaces
- preferred model
- preferred permission mode
- binary path
- UI preferences

The source of truth for agent session data should remain the backend session system unless there is a strong reason to duplicate it.

## 8. Integration Risks

### 8.1 Runtime coupling

If Poro assumes too much about exact backend internals, updates may become painful.

Mitigation:

- keep an adapter layer
- centralize backend communication in one module

### 8.2 Process UX

Launching a CLI from a desktop app can feel fragile if errors are surfaced poorly.

Mitigation:

- build a strong first-run setup flow
- show actionable error messages
- make state visible

### 8.3 Provider configuration confusion

Users may have multiple API keys, local servers, or model providers.

Mitigation:

- make provider selection explicit
- keep settings readable
- expose a clear health check

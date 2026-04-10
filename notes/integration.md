# Poro Integration Notes

**Version**: April 2026

## 1. Core Integration Decision

Poro should use the sibling `harness` repo as the execution engine.

Poro should **not** rebuild:

- mission control
- provider routing
- tool execution
- permission handling
- verification logic
- session storage

The desktop app should focus on orchestration, presentation, and UX.

## 2. Current Integration Shape

The current local development shape is:

```text
poro/
harness/
```

Poro currently:

- builds `harness-server` from `../harness`
- launches it from the Tauri layer
- uses Harness session storage and runtime events
- syncs session snapshots into the UI after each turn

This is the right shape for now because it keeps the real controller in Harness while letting the app move quickly.

## 3. Responsibilities

### 3.1 Poro responsibilities

Poro should handle:

- workspace selection
- local app settings
- first-run setup guidance
- backend health check UX
- launch/stop session commands
- session list and resume flow
- transcript presentation
- runtime activity presentation
- review and diff presentation

### 3.2 Harness responsibilities

Harness should handle:

- controller loop
- mission state
- provider calls
- tool selection and execution
- permission enforcement
- verification
- session persistence

## 4. Recommended Boundary

The desired long-term contract is:

- Poro talks to `harness-server`
- `harness-server` exposes structured session and runtime events
- the frontend never needs to understand private runtime internals

Current implementation still links some sibling Harness crates directly in the Tauri layer. That is acceptable for local iteration, but the architectural direction should move toward a cleaner server/sidecar boundary over time.

## 5. Runtime Modes

### 5.1 Local-first mode

Default stack:

- `harness-server`
- Ollama on `127.0.0.1:11434`
- `gemma4:e2b`

This should remain the easiest no-account path.

### 5.2 Hosted-provider mode

Current supported hosted-provider path:

- xAI / Grok through Harness
- Anthropic through Harness

Poro should stay provider-agnostic at the UI level. The runtime owns the provider behavior.

## 6. Health and Setup

The desktop app should keep a strong first-run diagnostic flow that checks:

- backend path resolution
- backend launchability
- local session-store access
- Ollama reachability in local mode
- selected model presence in local mode

The goal is to make the app feel dependable even when the environment is not yet configured correctly.

## 7. Current Integration Risks

### 7.1 Too much runtime knowledge in the app

If Poro learns too much about Harness internals, future runtime changes become painful.

Mitigation:

- centralize bridge logic
- keep event translation in one place
- avoid pushing private runtime semantics into the Svelte layer

### 7.2 Legacy naming drift

Some frontend modules still use `claw` naming for compatibility even though the runtime is now Harness-backed.

Mitigation:

- rename gradually
- keep runtime behavior stable while the naming is cleaned up

### 7.3 Local privacy vs shipped IP

Local development with a sibling private repo is good hygiene, but it is not the same as server-side secrecy.

Mitigation:

- keep the private runtime boundary clear
- avoid committing local runtime artifacts
- plan for a stronger sidecar or hosted boundary later

## 8. Recommended Next Integration Work

The next integration work should be UI-facing, not architecture-heavy:

- surface progress/blocker/verification state cleanly
- show richer runtime actions without log spam
- improve approval and review UX
- keep the runtime boundary intact while the app becomes more trustworthy

Only after that should we decide whether to:

- tighten the local sidecar boundary
- or move the real runtime server-side

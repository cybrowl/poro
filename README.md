# Poro

**A local-first desktop coding workspace for Harness-backed AI sessions.**

Poro is a SvelteKit + Tauri desktop app for running coding sessions against a local or bring-your-own-provider Harness runtime. The current product is focused on a calm desktop UI, visible runtime state, and a clean bridge between the app and the private sibling `harness` repo.

## Current State

Poro is no longer the old ICP/chat/payment product. The repo now reflects the desktop coding app direction:

- Tauri desktop shell with a Svelte 5 frontend
- local-first default flow with Ollama + Gemma 4
- optional hosted providers through the sibling Harness runtime
- session transcript, runtime activity, settings, workspace picker, and diff-oriented workspace surfaces
- privacy guardrails to avoid accidentally committing local Harness artifacts

Today, the real product loop is:

- Poro desktop UI
- sibling `harness-server` runtime
- local Ollama by default, or optional hosted providers such as xAI / Grok and Anthropic

## Architecture

Poro is the app shell. Harness is the coding runtime.

- `poro` owns the desktop experience, settings, workspace management, session surfaces, and runtime event presentation
- `harness` owns the controller, tool execution, verification, mission state, and provider integrations
- the Tauri layer talks to `harness-server` and syncs session snapshots into the UI
- `tauri/crates/agent-browser` is a vendored browser automation skeleton we can iterate on locally inside this repo

Current integration assumes this sibling layout:

```text
/Users/.../Repos/
  poro/
  harness/
```

## Quick Start

Install dependencies:

```bash
npm install
```

If you want the UI shell only:

```bash
npm run dev
```

That starts the Svelte frontend on `http://localhost:5173`.

If you want the real desktop app:

```bash
npm run tauri
```

That command also builds the sibling `harness-server` first from `../harness`.

## Local Desktop Setup

The default local stack is:

- Poro
- `harness-server`
- Ollama on `http://127.0.0.1:11434`
- `gemma4:e2b`

To use the default local path:

```bash
ollama pull gemma4:e2b
npm run tauri
```

Inside the app, keep:

- provider: `Ollama Local`
- model: `gemma4:e2b`
- permission: `workspace-write`

By default, Poro expects the backend path to resolve to `harness-server`.

## Hosted Providers

Poro can also launch sessions through the sibling Harness runtime using hosted providers.

Current UI/provider wiring supports:

- `Ollama Local`
- `xAI / Grok`
- `Anthropic`

Provider credentials are handled by Harness, not by the Svelte UI itself.

## Useful Commands

```bash
npm run dev
npm run check
npm run tauri
npm run tauri:build
npm run guard:private
```

Notes:

- `npm run check` runs the Svelte/type-check pass
- `npm run tauri` and `npm run tauri:build` build the sibling Harness server first
- `npm run guard:private` checks staged changes for private local artifacts and obvious secret material

## Repo Layout

```text
src/ui/                     Svelte desktop UI
tauri/                      Rust desktop bridge and native app shell
tauri/crates/agent-browser  Vendored browser automation skeleton
scripts/guard-private-assets.sh
notes/                      Product and planning docs
```

Key files:

- [`src/ui/routes/+page.svelte`](src/ui/routes/+page.svelte) — main desktop workspace
- [`src/ui/lib/desktop.ts`](src/ui/lib/desktop.ts) — desktop defaults and settings helpers
- [`src/ui/lib/clawRuntime.ts`](src/ui/lib/clawRuntime.ts) — frontend bridge for runtime calls and events
- [`tauri/src/lib.rs`](tauri/src/lib.rs) — native runtime integration

## Privacy and Security

This repo intentionally keeps the real Harness implementation in a separate sibling repo.

Poro currently includes:

- ignore rules for local Harness artifacts
- a staged-change guard to catch obvious private files and API-key-shaped strings
- a cleaner public Git history that removes accidentally included private project material

Important practical note:

- keeping the private runtime in a sibling repo is good hygiene
- shipping a local app is still not the same as server-side secrecy
- the long-term path for stronger IP protection is a stricter sidecar boundary or a hosted private backend

## Notes

- [`notes/spec.md`](notes/spec.md)
- [`notes/market.md`](notes/market.md)
- [`notes/integration.md`](notes/integration.md)
- [`notes/plan.md`](notes/plan.md)

## Community

- X: [@poro_app](https://x.com/poro_app)
- GitHub: [cybrowl/poro](https://github.com/cybrowl/poro)

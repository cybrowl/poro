# agent-browser

Vendored browser automation skeleton used inside the Poro desktop workspace.

## Why This Lives Here

This crate was imported from a stripped local fork of the open-source
`agent-browser` project so we can keep iterating inside the `poro` repo instead
of depending on a separate browser-tool workspace.

It is intentionally treated as a local internal crate:

- `publish = false`
- no external release workflow
- reduced command surface
- machine-native `stdio` mode for agent integration

## Current Shape

This is not the original full upstream product. The vendored version keeps the
browser control core and removes extra product layers we do not need right now.

Retained focus:

- browser session control
- snapshots and screenshots
- element interaction primitives
- newline-delimited JSON `stdio` protocol

Deliberately removed or reduced:

- chat-oriented flows
- dashboard-first UX
- install/upgrade product surface
- extra packaging and benchmark scaffolding

## Working Contract

Today this crate is a workspace member under:

```text
poro/tauri/crates/agent-browser
```

It currently builds as its own binary target and is intended to be spawned by
the desktop runtime before we decide whether any pieces should become a shared
library API.

## Notes

- Treat this as vendored internal code, not as a normal third-party dependency.
- If we re-sync from the stripped fork later, prefer small, explicit diffs.
- If the protocol evolves, keep the `stdio` machine contract stable first.

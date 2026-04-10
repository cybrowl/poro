# Poro Business and Monetization Notes

**Version**: April 2026  
**Product**: Poro Desktop

## 1. Executive Summary

Poro is an open-source desktop AI coding workspace built around a local-first UI and a bring-your-own-provider model.

The product focus is not model hosting. The product focus is the **experience of working with an AI coding agent**:

- clearer visibility
- better session flow
- stronger diff awareness
- calmer interface design
- lower cost of entry than premium seat-priced tools

Poro uses `claw-code` as the runtime layer for tool use and agent execution. Poro's value is the desktop experience built around that capability.

## 2. Positioning

Poro should be positioned as:

- a local-first AI coding workspace
- an open-source desktop alternative to terminal-heavy agent tools
- a more affordable path for users who already have API keys or local models

Poro should **not** be positioned as:

- a model company
- a crypto product
- a generic chat interface
- a direct "cheap clone" of a premium competitor

## 3. Why This Can Work

The opportunity is not "we built the smartest agent."

The opportunity is:

- many users want the workflow, not the bundled subscription
- many users already have provider access or want to use cheaper models
- many users do not want to live in a terminal
- trust matters more when a coding tool can read and change local files

An open-source, design-forward desktop app can win on:

- trust
- taste
- transparency
- flexibility
- affordability

## 4. Core Business Model

### Phase 1

Open-source core app plus official paid distribution.

Model:

- source code remains public
- self-build remains free
- official signed and notarized desktop builds are paid

Users are paying for:

- convenience
- trust
- polish
- release quality
- onboarding
- support

### Phase 2

Optional paid cloud conveniences.

Examples:

- sync across devices
- encrypted backup of sessions
- workspace settings sync
- searchable cloud history
- shared team workspaces

This is a stronger recurring revenue model than charging for local-only features.

## 5. Recommended Pricing

### Initial launch

Recommended path:

- free alpha and private beta builds for early users
- paid stable release once the product is clearly useful

Suggested pricing for the official app:

- **$29-$49 one-time**

Potential update policy:

- includes the current version and 12 months of updates
- later major versions can be paid upgrades

### Later paid plans

Only add subscriptions when there is real hosted value.

Examples:

- **Poro Sync**: cloud backup and sync
- **Poro Team**: shared workspaces and collaboration

## 6. Distribution Strategy

### Primary

Direct download from the Poro website.

Requirements:

- signed macOS app
- notarized distribution
- polished install flow
- clear release notes

### Secondary

- GitHub releases for community visibility
- Setapp after the app is polished enough

### Not first

Mac App Store should not be the default launch plan. Terminal access, local tool execution, and file-heavy workflows make App Sandbox constraints a poor fit for the first release.

## 7. Revenue Logic

Poro should monetize where users naturally expect to pay:

- official binaries
- convenience
- reliability
- sync and storage
- support

Poro should avoid building the business around:

- locking basic local UI features
- gating cosmetic-only features behind accounts
- forcing subscriptions before hosted value exists

## 8. Launch Strategy

### Phase 1: Build in public

- document the product direction clearly
- share design progress
- show the app working on real repos
- collect feedback from developers who already use AI tools heavily

### Phase 2: Closed beta

- onboard a small set of daily users
- validate the core workflow
- improve stability and review UX

### Phase 3: Public launch

- release public source
- sell official binaries
- publish product walkthroughs and comparisons

## 9. Risks

### 9.1 Over-dependence on the runtime

If the UI is too tightly coupled to `claw-code` internals, Poro may be harder to evolve.

Mitigation:

- keep an adapter boundary
- make the runtime integration replaceable over time

### 9.2 Weak monetization if everything is purely local

If there are no paid convenience layers, revenue may depend only on goodwill.

Mitigation:

- sell official builds first
- design optional hosted sync later

### 9.3 Trust concerns

Users may hesitate to install a local tool that can run commands and edit files.

Mitigation:

- keep source public
- make permission state obvious
- keep the UI transparent about actions and diffs

### 9.4 Support burden

Local developer tools can generate support complexity because user environments differ.

Mitigation:

- build a strong setup flow
- add health checks
- keep provider configuration clear and visible

## 10. Business Goal For The First Release

The first release does not need to maximize revenue.

It needs to prove that:

- developers want this UI
- they can do real work through it
- they trust it enough to use it repeatedly
- enough of them prefer the official app over self-building

If those are true, the monetization surface gets much stronger later.

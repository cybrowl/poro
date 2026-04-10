# Poro Market and Business Notes

**Version**: April 2026  
**Product**: Poro Desktop

## 1. Executive Summary

Poro is a desktop coding workspace for AI-assisted software work.

The product is not trying to win by owning the largest model or by bundling expensive inference. The opportunity is to win on:

- trust
- clarity
- workflow quality
- visual taste
- provider flexibility

Today the stack is best described as:

- public-facing desktop app
- private Harness-backed runtime boundary
- local-first usage by default
- optional hosted providers for users who want frontier models

## 2. Positioning

Poro should be positioned as:

- a desktop AI coding workspace
- a calmer alternative to terminal-heavy agent workflows
- a local-first interface with bring-your-own-provider flexibility
- a product that makes agent work visible and reviewable

Poro should not be positioned as:

- a model lab
- a generic chatbot
- a crypto project
- a cheap clone whose only story is price

## 3. Why This Can Work

The opportunity is not “we built the smartest raw model.”

The opportunity is:

- many users want the workflow more than the bundled subscription
- many users already have API keys or want local models
- many users do not trust black-box agent behavior
- desktop coding tools feel better when state, permissions, and diffs are visible

Poro can win on:

- product taste
- transparent UX
- lower cost of entry
- flexible runtime/provider choice
- a stronger feeling of control

## 4. Product and IP Shape

Right now the most sensible shape is a **hybrid product**:

- Poro desktop app can remain public-facing
- the real Harness implementation can stay private
- the boundary between them becomes part of the product architecture

This matters because the harness/controller logic is becoming real IP. We should not casually assume every important layer needs to ship publicly forever.

## 5. Monetization Direction

Do not overcommit too early. The likely path is:

### Phase 1

Sell the product experience:

- official signed desktop builds
- convenience
- polish
- onboarding
- release quality

### Phase 2

Add optional paid hosted value:

- account-linked sync
- encrypted session backup
- team features
- hosted private runtime for users who want stronger security or easier setup

The key rule is:

- do not force subscriptions before there is real hosted value

## 6. Distribution

Primary path:

- direct download from the Poro website

Requirements:

- signed macOS app
- polished first-run setup
- clear provider/runtime instructions

Secondary path:

- GitHub releases for visibility

Not the first path:

- Mac App Store

The current product shape is too dependent on local workspaces, agent permissions, and runtime flexibility to make App Sandbox a good first target.

## 7. Pricing Direction

Good early options:

- free alpha / private beta
- paid stable desktop release later

Reasonable initial range:

- one-time purchase for official builds

Only add subscriptions when the hosted side is genuinely useful.

## 8. Risks

### 8.1 Weak differentiation if the app feels like a wrapper

Mitigation:

- keep pushing visible state, review, progress, and approval UX

### 8.2 Over-coupling to a private local runtime

Mitigation:

- keep the boundary clean now so sidecar or hosted deployment remains possible later

### 8.3 Shipping too much private logic locally

Mitigation:

- keep sensitive runtime logic out of the public repo
- move toward stronger sidecar or server-side boundaries when the product stabilizes

### 8.4 Monetization confusion

Mitigation:

- do not promise a business model too early
- prove the product loop first

## 9. What Success Looks Like

The first real business proof is not maximum revenue.

It is:

- users prefer doing real coding work through Poro
- they trust it enough to use repeatedly
- they feel the UI meaningfully improves the agent workflow
- enough users are willing to pay for convenience, polish, or hosted value later

# Poro Product Plan

**Version**: April 2026

## 1. Objective

Ship a desktop coding workspace that feels materially better than running the same agent workflow directly in terminal.

The current project is no longer about proving we can launch a runtime. We have crossed that line. The next objective is to make the experience feel trustworthy, legible, and premium.

## 2. What Is Already Done

### Phase 0: Product reset

Completed:

- removed old ICP/chat/payment framing
- aligned the repo around the desktop coding app direction
- cleaned public Git history and privacy guardrails

### Phase 1: Desktop shell

Completed:

- main desktop layout
- workspace picker
- settings sheet
- session shell
- transcript surface
- runtime activity surface

### Phase 2: Harness integration

Completed:

- sibling `harness-server` integration
- backend health checks
- local settings persistence
- workspace launch flow
- session resume/load flow
- provider/model/permission wiring

## 3. Current Phase

### Phase 3: Trustworthy session UX

This is the active phase now.

Goal:

- make the desktop session feel alive, understandable, and safe

Deliverables:

- better live action visibility
- cleaner transcript/noise balance
- visible progress, blockers, and verification
- clearer approval states
- stronger diff/review presentation

Exit criteria:

- a user can understand what the agent is doing without reading raw logs
- long turns feel busy instead of broken
- review state feels first-class, not bolted on

## 4. Next Phase

### Phase 4: Beta readiness

Goal:

- make the app feel ready for repeated daily use by a small group

Deliverables:

- onboarding cleanup
- stronger empty/loading/error states
- packaging and release discipline
- cleaner naming and UI copy
- desktop polish pass

Exit criteria:

- a new user can install, configure, and run a real session without hand-holding

## 5. Immediate Next Tasks

1. Surface structured progress in the main session UI.
2. Improve approval UX so permission decisions feel intentional.
3. Compress transcript noise and elevate the most important runtime actions.
4. Strengthen the review surface for changed files and verification outcomes.
5. Clean up remaining `claw` naming in the UI bridge and product copy.

## 6. Things We Should Not Do Right Now

Avoid:

- moving the full runtime server-side immediately
- over-expanding features before the core session UX is strong
- turning the UI into a generic log viewer
- chasing breadth instead of trust and clarity

## 7. Decision Rules

When there is a tradeoff, prefer:

- visibility over hidden automation
- calm over noise
- one excellent core flow over broad feature count
- a clean boundary over short-term coupling
- local-first iteration over premature infra complexity

## 8. Success Metric For The Next Milestone

The next milestone is successful when Poro can convincingly demo this flow:

1. open a repo
2. choose provider, model, and permission mode
3. send a coding task
4. watch the agent work without confusion
5. review the result with confidence
6. resume the same work later

If that flow feels good, the product has a real foundation.

# Poro MVP Plan of Action

**Version**: April 2026

## 1. Objective

Ship a local-first desktop MVP that lets one developer use Poro as a polished UI for real `claw-code` coding sessions on a local repository.

## 2. MVP Definition

The MVP is done when a user can:

- open a local workspace
- configure or verify backend access
- start a real session
- watch transcript and tool activity live
- inspect file diffs
- resume the session later

## 3. Build Order

### Phase 0: Product reset

Goal:

- remove old ICP product framing
- align the repo and notes around the new desktop direction

Deliverables:

- updated product notes
- new MVP scope
- clear technical direction

### Phase 1: UI shell with mock data

Goal:

- build the interface before backend coupling

Deliverables:

- desktop-first layout
- workspace picker screen
- session screen shell
- tool activity timeline
- diff / review panel
- settings sheet

Exit criteria:

- the app already feels like the product even before real integration

### Phase 2: Tauri desktop foundation

Goal:

- turn the current frontend into a real desktop app

Deliverables:

- Tauri 2 setup using `job_raptor` as reference
- desktop config and packaging basics
- file picker integration
- local settings persistence

Exit criteria:

- Poro launches as a desktop app and can select a local workspace

### Phase 3: `claw-code` integration

Goal:

- replace mock data with a real local agent session

Deliverables:

- binary path configuration
- health check flow
- launch session command
- stop session command
- session output streaming
- local session resume support

Exit criteria:

- Poro can run a real session against a real repo

### Phase 4: Review and control loop

Goal:

- make the UX trustworthy for actual code changes

Deliverables:

- visible permission mode state
- diff awareness in the UI
- session status visibility
- clearer error and recovery states

Exit criteria:

- users can understand what the agent did and what changed

### Phase 5: Beta polish

Goal:

- prepare for a credible early release

Deliverables:

- onboarding cleanup
- empty states and loading states
- visual polish pass
- first-run guidance
- signed beta build strategy

Exit criteria:

- small-group beta users can install and use Poro without hand-holding

## 4. Immediate Next Tasks

The next implementation steps should be:

1. Add the Tauri desktop shell to this repo using the `job_raptor` structure as reference.
2. Replace the current route structure with a desktop-oriented layout.
3. Build the main session screen with mock data.
4. Add workspace selection and recent-workspace storage.
5. Implement the Tauri command layer for backend process control.

## 5. First Version UI Requirements

The first version should include:

- recent workspaces
- one strong primary session screen
- a transcript panel
- a tool activity panel
- a diff / review panel
- model and permission controls
- backend setup guidance

The first version should not include:

- cloud sync
- billing
- team features
- mobile support
- advanced analytics
- plugin systems

## 6. Decision Rules

When there is a tradeoff during MVP, prefer:

- clarity over configurability
- local-first over hosted complexity
- trust over cleverness
- polish on one flow over breadth across many flows

## 7. Success Metric For The Next Milestone

The next milestone is successful when Poro can convincingly demo this flow:

1. open a repo
2. choose permissions and model
3. send a prompt
4. watch the session work
5. inspect the diff
6. resume the session later

If that flow feels great, the rest of the product has a strong foundation.

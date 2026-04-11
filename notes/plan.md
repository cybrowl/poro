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
6. Tighten the desktop visual language so it feels closer to a serious editor than a stack of dashboards.

## 6. Plan Of Action

This is the working roadmap from here. The goal is not to add lots of features quickly. The goal is to make the core experience feel excellent, then make it durable, then make it ready for real users.

### Phase 3A: Session clarity

Goal:

- make the main session understandable at a glance

Workstreams:

- turn the activity feed into the primary “what is happening now” surface
- distinguish planning, editing, verification, browser actions, and blocked states more clearly
- reduce transcript duplication and push low-signal runtime noise out of the way
- make empty, waiting, and in-progress states feel intentional instead of accidental

Exit criteria:

- a user can tell what the agent is doing in under 2 seconds
- long turns feel active, not frozen
- the session reads like one coherent workflow instead of transcript + side widgets

### Phase 3B: Review and verification

Goal:

- make the result trustworthy, not just visible

Workstreams:

- build a first-class review surface for changed files
- show verification outcomes as a product surface, not buried metadata
- expose what was changed, what passed, what failed, and what still needs attention
- connect browser verification and runtime verification into the same story

Exit criteria:

- a user can review the outcome without dropping to raw git or raw logs
- verification feels like part of the product, not an implementation detail
- browser-assisted checks can be understood in the same session flow

### Phase 3C: Browser-assisted workflows

Goal:

- make browser capability feel native to the agent workflow

Workstreams:

- surface browser actions inline in the session feed
- support prompt-driven “fix, then verify in browser” loops
- keep the browser inspector secondary while making browser activity first-class
- expand browser eval coverage for realistic app flows

Exit criteria:

- users can ask for browser work in plain language and understand what happened
- browser activity feels like part of the same agent session, not a separate tool
- browser evals cover enough flows to trust the feature direction

### Phase 3D: UI polish system

Goal:

- make the shell feel premium and repeatably improve it

Workstreams:

- finish the visual cleanup toward a calmer editor-like feel
- lock typography, spacing, surface, and accent rules into a clearer design system
- use `agent-browser` visual baselines for repeatable before/after screenshot checks
- treat UI work as verification-required: a UI fix is not complete until a fresh rendered screenshot is captured after the last edit
- keep refining the shell, transcript, composer, sidebar, and drawers until the product feels intentional

Exit criteria:

- the shell feels like a desktop app, not a centered web page
- the visual language is consistent enough that new surfaces do not drift
- screenshot baselines exist for the key desktop views

### Phase 4A: First-run and settings quality

Goal:

- make setup and repeated use calm for a new user

Workstreams:

- simplify provider/model/permission setup language
- improve onboarding copy and state transitions
- clean up backend path selection and health check behavior
- remove remaining confusing terminology and rough settings edges

Exit criteria:

- a new user can get from install to first useful session without hand-holding
- settings feel like product UI, not a debug panel

### Phase 4B: Reliability and release discipline

Goal:

- make the app safe to ship to a small group repeatedly

Workstreams:

- stabilize the main eval batteries and browser evals
- add a small release checklist for desktop builds
- use screenshot baselines and regression checks to catch obvious UI drift
- tighten private-asset guardrails and packaging discipline

Exit criteria:

- internal builds are repeatable and predictable
- regressions are caught earlier
- the app feels safe to hand to a few real users

### Phase 4C: Small beta

Goal:

- validate that the experience is strong outside our own heads

Workstreams:

- put the app in front of a very small set of design-minded developers
- watch where they get confused in session flow, review flow, and setup flow
- fix the highest-friction points before adding broader scope

Exit criteria:

- at least a few outside users prefer Poro to a terminal-only workflow for real tasks
- the main workflow survives real usage without collapsing into support burden

### Phase 5: Expansion, only after the core is excellent

Goal:

- expand carefully from a strong center

Possible directions:

- hosted or sidecar runtime boundary when the product loop is stable
- richer review workflows
- broader browser-assisted validation
- future adjacent products like `Cutline`

Rule:

- do not start Phase 5 work until the core Poro session loop feels excellent and beta-worthy

### Current UI Audit

The latest visual pass surfaced a few concrete problems we should keep using as a design checklist:

- left rail hero copy is too large and wraps badly
- main canvas duplicates too much session context and feels cramped
- live action feed is too narrow, so labels and titles break awkwardly
- pills and buttons are still too loud and bulky
- browser inspector competes with the main work surface instead of supporting it
- too many boxes and borders make the interface feel busy instead of sleek

### Screenshot Analysis Note

## 7. Document Composer Roadmap

We have chosen a hybrid editor architecture for the main prompt surface:

- `pretext` owns text layout and visible rendering
- a hidden native input layer still handles typing, composition, selection semantics, and clipboard behavior
- we progressively close the gap until the surface feels like a real native document editor

### Phase 0: Interaction baseline

Goal:

- make the hybrid composer feel intentional instead of looking like a transparent textarea trick

Workstreams:

- render visible text exclusively from the `pretext` layer
- track focus and selection state from the hidden native input
- add custom caret and selection visuals that align with the rendered text layer
- hide native selection/background artifacts so the document surface looks coherent

Exit criteria:

- the composer visibly behaves like a document surface
- caret and selection feel present and legible
- typing still uses stable native input semantics underneath

### Phase 1: Editing fidelity

Goal:

- improve the feeling of editing so the surface behaves more like a document than a form field

Workstreams:

- make caret motion and selection visuals match rendered lines more closely
- support click placement and drag selection with fewer visible mismatches
- refine focus, current-line, and empty-state behaviors
- improve scroll anchoring so long documents feel calm while typing

Exit criteria:

- common editing actions feel trustworthy
- the visible document and the native input state rarely drift in obvious ways

### Phase 2: Rich document behaviors

Goal:

- make the surface capable of more than plain prompt entry

Workstreams:

- introduce richer inline blocks such as references, chips, or lightweight structured segments
- support document sections, prompt templates, and reusable snippets
- explore richer visual hierarchy inside the composer without losing text-editing stability

Exit criteria:

- the composer can support more complex prompt documents than a plain textbox
- richer structures still feel editor-like, not widget-heavy

### Phase 3: Decide the editor ceiling

Goal:

- decide how far Poro should go toward a real document editor

Decision paths:

- keep the current hybrid model and polish it as a premium prompt/document surface
- extend the hybrid model into a deeper editor with stronger custom behaviors
- or adopt a fuller editor architecture only if the product clearly needs it

Decision criteria:

- whether Poro benefits materially from a true editor surface
- whether the added complexity improves the core workflow enough to justify it
- whether the hybrid model already feels good enough for the product

If DOM snapshots are not enough for judging the rendered UI, we can use screenshot analysis as a helper lane. A candidate to evaluate later is [Falcon-Perception](https://github.com/tiiuae/Falcon-Perception), mainly for OCR / screenshot understanding, not as a replacement for the browser sidecar or prompt-driven browser actions.

### UI Verification Rule

For UI and visual tasks, source-level changes are not enough. The working rule is:

- do not mark a UI task complete from code inspection alone
- capture a fresh rendered screenshot after the last patch
- compare it against the current goal, baseline, or reference before calling the task done

This should live in the harness over time so UI verification becomes a real runtime phase instead of a guess.

### Future Milestone: Poro Can Build Poro

This is a later milestone, not the current focus, but it is worth naming clearly:

- the long-term bar is that Poro becomes trustworthy enough to be the natural environment we would choose to use while building Poro itself

That requires:

- session trust that holds up during real work
- first-class review and verification surfaces
- native-feeling git workflow
- dependable agent-driven repo changes
- screenshot-backed UI verification instead of source-only guesses

This is not a Phase 3 requirement, but it is an important north-star milestone for later.

### Future Product Note: Cutline

Keep a note for a possible future product branch called `Cutline`.

Idea:

- a video editor / storytelling workspace built with the same taste for calm, guided, AI-assisted creative flow
- focused on helping a user shape narrative, sequence clips, refine structure, and turn rough material into a coherent story
- should be treated as a future adjacent product direction, not part of the current Poro desktop coding scope

## 7. Things We Should Not Do Right Now

Avoid:

- moving the full runtime server-side immediately
- over-expanding features before the core session UX is strong
- turning the UI into a generic log viewer
- chasing breadth instead of trust and clarity

## 8. Decision Rules

When there is a tradeoff, prefer:

- visibility over hidden automation
- calm over noise
- one excellent core flow over broad feature count
- a clean boundary over short-term coupling
- local-first iteration over premature infra complexity

## 9. Success Metric For The Next Milestone

The next milestone is successful when Poro can convincingly demo this flow:

1. open a repo
2. choose provider, model, and permission mode
3. send a coding task
4. watch the agent work without confusion
5. review the result with confidence
6. resume the same work later

If that flow feels good, the product has a real foundation.

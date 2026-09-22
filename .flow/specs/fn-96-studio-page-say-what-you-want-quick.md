# Studio page: quick dials up front, parameters explained

## Conversation Evidence

> user: "I want to rework the UI to basically take many of these building blocks and expose them natively => connect your claude code / codex and express what your tree should look like."
> user: "We should have the main entry point be an easy to use just say what you want and it'll happen plus the most important dials that someone might wanna tune quickly. Then there should be the option to look at the parameters. Group those nicely and intuitively. With explainers where needed. "
> user (selected): "Fixed authored set"
> user (selected): "Kept, behind the advanced view"
> user (selected): "Owner first"
> user: "ok go ahead do it" (accepted splitting the page rework and the Jev switch-on out of fn-94)

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 60% [user], 30% [paraphrase], 10% [inferred] -->

The UI today is a developer harness: every dial in one panel, labelled for the person who wrote the generator. The owner wants the main entry point to be easy: a place to say what you want, the tree, and the few most important dials someone might want to tune quickly. Behind it sits the option to look at the parameters, grouped intuitively, with explainers where needed.

This spec is the page alone. It ships and is useful with no agent: a person tunes a tree from the quick dials and can open the full parameters. The "say what you want" box is not built here: the agent studio (fn-94) depends on this spec and owns the box whole, from input to the agent's reply, in the place this page reserves for it. The owner is the first user; the page is built so an outside developer can use it unchanged.

## Architecture & Data Models
<!-- scope: technical -->

- **One dial table drives both views.** The harness already renders its dials from one table carrying key, label, group, range, step and unit. This spec adds a plain explainer per dial and marks the fixed quick-dial set in that same table; the entry view and the full parameter view both render from it. [paraphrase]
- **Entry view:** the tree, the quick dials, a way into the full parameters, and a reserved place for the conversation that fn-94 fills. Until then the place is empty and the layout reads as complete without it. [paraphrase]
- **Full parameter view:** one step from the entry view; dials grouped by what they shape, each group named in plain words, each dial with its explainer where its name does not explain it. [user]
- **Developer tools stay where species work expects them:** the full dials, the growth switch behind its flag and the views species QA relies on are reachable behind the advanced view and behave as before. [user]
- **The library stays free of the page.** The published package gains no UI code. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Dial table row:** key, label, group, range, step, unit, explainer, quick (yes or no). The agent studio reads this same table. [inferred]
- **The conversation's place:** the entry view names one region another spec may fill. This spec puts nothing in it. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Species QA must read the same tree.** The rework changes no parameter, default, camera or build path that the headless stills and species QA use. [paraphrase]
- **Growth stays hidden** behind its flag, never a default. [paraphrase]
- **Explainers are plain words,** a sentence or two, saying what the dial does to the tree a person sees, never the mechanism's internal name alone. [inferred]
- **Small screens:** the entry view works at a laptop's width; the full parameter view may scroll. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The entry view shows the tree, a fixed authored set of quick dials and a way into the full parameters, with a reserved place for fn-94's conversation. Errors: with that place empty the page shows no dead control and the layout reads as complete. [paraphrase]
- **R2:** The full parameters are one step away, grouped by what they shape, each dial with a plain explainer where its name does not explain it. Errors: a dial with no group or no explainer fails a check, never ships silently. [user]
- **R3:** The quick-dial set is authored in the dial table, the same every session. Errors: a quick dial naming a key the table lacks fails the check. [user]
- **R4:** Today's developer tools, the full dials, the growth switch behind its flag and the views species QA relies on, stay reachable behind the advanced view and judge the same tree as before. Errors: a species QA run that reads differently after the rework fails the spec. [paraphrase]
- **R5:** The owner tunes a tree from the entry view's quick dials alone and records that the page reads as easy. Errors: a verdict taken with the full parameter view open does not count. [inferred]

## Boundaries
<!-- scope: business -->

- No agent, conversation, say-what-you-want box, sessions or local process; those are the agent studio's (fn-94). [paraphrase]
- No Jev. [paraphrase]
- No new dials and no generator or renderer behaviour. [inferred]
- The quick dials are a fixed authored set; nothing chooses or pins them at run time. [user]

## Decision Context

Split out of fn-94 at the owner's word on 2026-09-20. The page ships without an agent, is front-end work with its own check (species QA reads the same tree), and leaves the agent studio one loop with one proof. Which dials make the quick set is authored by the owner in the table; the implementer's first draft is a proposal the owner edits. [paraphrase]

## Strategy Alignment

- Serves the Build metric's interactive editing use case and the primary user, a developer meeting Telperion for the first time. [strategy:The core and integration]

## Resolved via Codebase

- The harness renders its dials from one table with key, label, group, range, step and unit, and no explainers (`harness/params.ts`, `SLIDERS`); `harness/family.ts` translates them to the generator's family.
- The harness is a development-only Vite and React page; the published library carries no UI (`vite.config.ts`).

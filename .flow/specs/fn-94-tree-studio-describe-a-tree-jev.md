# Tree studio: describe a tree, your agent tunes it

## Conversation Evidence

> user (turn 1, part 1): "check the fn-68 and corresponding add-species runner spec. I want to rework the UI to basically take many of these building blocks and expose them natively => connect your claude code / codex and express what your tree should look like."
> user (turn 1, part 2): "Jev will propose tuning sets and the llm will set the parameters. You can then iterate this way and if you're unable to get something done create flow-next spec that would close the gap."
> user (turn 1, part 3): "People could then make pull requests with those specs or done implementations."
> user (turn 2, selected): "i think we can combine a + b?"

## Goal & Context

<!-- scope: business -->

The species pipeline's blocks (fn-68's dial table, partial-overlay candidates, Jev choosing a direction and size per dial with code computing every value; fn-89's conductor and the add-species runner) serve one person at a terminal; the UI offers hand dials only.

The owner wants the UI to expose these blocks natively. The entry point: say what you want and it happens, beside the few most important quick dials; behind it, the parameters grouped intuitively with explainers. A person connects their own Claude Code or Codex and talks to it in the studio page, in words and pictures. It is a collaboration: the person says what they see; the agent states the gap more clearly (from botanical facts it may look up) or names gaps it sees itself, and the person can correct it. The agent sets parameters at once, says what changed and why, and one click takes a change back.

On 2026-09-20 the owner split out the page itself (entry view, quick dials, grouped explained parameters) as fn-96, which this spec depends on, and Jev's tuning proposals as fn-97, which depends on this spec and fn-68. This spec is the loop between a person and their agent.

The owner is the first user; an outside developer can use the studio unchanged with their own agent and TypeSafe key. The first proof is a tree the owner would keep, reached from words. When a look cannot be reached, fn-95 turns it into a gap spec; this spec ends where the loop reports it is stuck. The target is a description, not fn-68's photograph: the blocks carry over, the photograph-distance score does not.

## Architecture & Data Models
<!-- scope: technical -->

- **The studio runs locally.** One command starts a local studio process and opens the page. The process serves the page, starts the agent and writes sessions; the page draws the tree and carries the conversation. No hosted studio in this spec. [user]
- **The studio starts the agent.** Per session the local process launches the person's Claude Code in its headless mode, under their own login, relays the page's chat to it and its replies back, and hands it the studio's tools. Claude Code is the first adapter behind a small seam; Codex is a second adapter later. [user]
- **The agent's powers are fixed by the studio:** the studio's tools, web lookups, and reading the repository (catalogue and docs, to ground its botany). No file writes, no shell. [user]
- **A change is a partial overlay on the current parameters,** the same candidate shape fn-68 uses, so a row the generator does not know or a value off its range is refused by name. [inferred]
- **The dial table is the page's** (fn-96): key, label, group, range, explainer and the quick-dial mark. The agent reads that table and keeps no copy. Its merge with fn-68's table is fn-97's. [paraphrase]
- **A session is files on disk** in a git-ignored folder: the description and pictures, and a list of rounds, each with the observations, the gap as agent and person stated it, the overlay applied, the seed and the generator revision. Any round can be replayed or undone. [user]
- **The still the agent sees is the page's own picture of the tree,** so agent and person judge the same image. [inferred]
- **The result is a template:** a value table over the shared generator, a point in the one tree space, with no species or template branch in generator or renderer. [strategy:The catalogue]

## API Contracts
<!-- scope: technical -->

The studio's tools, as the agent sees them:

- **Read state:** the current parameters, the dial table with groups, explainers and ranges, the generator's declared capabilities, and the session's rounds. [inferred]
- **Look:** a still of the current tree as the page shows it, at a named view. [inferred]
- **Apply:** a partial overlay and a one-line reason in; the effective parameters, the time the rebuild took and the round's id out, or a refusal naming the row and the reason. [inferred]
- **Undo:** a round id in; the parameters as they stood before it out. [inferred]
- **Stuck:** the described trait not reached and the rounds that tried. fn-95 consumes this report. [paraphrase]

Between page and local process: chat messages both ways, agent status (starting, ready, working, failed with reason), and tree changes pushed to the page as they are applied. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **No Jev here.** The studio works on the agent alone; fn-97 adds proposals and everything the TypeSafe rule asks of them. [paraphrase]
- **The local process listens on the local machine only,** and the page and the agent's tools reach it with a per-run token. [inferred]
- **The agent is missing, logged out or crashes:** the page shows the reason, the dials keep working, the session's rounds are already on disk, and a restart resumes the session. [inferred]
- **Person and agent change the tree at the same moment:** changes apply in arrival order, each as its own round, so undo stays exact. [inferred]
- **A refused value is refused, never clamped,** and the round records the refusal. [inferred]
- **Mature trees are the product.** The studio tunes the direct build; growth stays behind its flag. [paraphrase]
- **Build latency bounds the loop.** A round is only as quick as parameters-to-tree; the Build metric's numbers apply and every apply reports its time. [strategy:The core and integration]
- **The library stays free of the studio.** The published package gains no UI, agent or server code; React and the studio remain development-side. [paraphrase]
- **Pictures stay local.** Reference pictures live in the ignored session folder and are never committed. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A person connects their own Claude Code or Codex to the running UI, and the UI shows that an agent is connected. Errors: with no agent connected the UI works by hand as before; a dropped connection is shown and the session's rounds survive it. [user]
- **R2:** The person states in words what their tree should look like; the description is recorded in the session and is what every later round works toward. Errors: an empty description starts no round. [user]
- **R3:** The connected agent can read the current parameters, the page's dial table (fn-96), the generator's declared capabilities and a still of the current tree. Errors: a dial outside the table is never offered. [paraphrase]
- **R5:** The connected model sets the parameters as a partial overlay, and the tree rebuilds in the UI. Errors: an unknown row or an out-of-range value is refused by name, never clamped, and the tree is unchanged. [user]
- **R6:** The person iterates: each round is recorded with its description, observations, applied overlay and seed, and any round can be undone or returned to. Errors: undo past the first round returns the starting preset. [paraphrase]
- **R7:** The tuned tree exports as a template value table that loads back into the UI and reproduces the same tree at the same seed and generator revision. Errors: an export the parser refuses is reported, never written. [inferred]
- **R8:** When rounds stop reaching a described trait, the loop says so and names the trait and the rounds that tried, instead of proposing further. Errors: the report never claims a generator gap on its own; deciding that is the follow-on spec's. [paraphrase]
- **R10:** The conversation with the connected agent happens in the studio page: this spec builds the say-what-you-want box, its history and the agent's status line in the place fn-96's entry view reserves. The person says what they see; the agent restates the gap between tree and description, names gaps it sees itself, and the person can correct either. Errors: with no agent connected the box says so and the dials still work. [user]
- **R11:** The agent applies a change at once, says which parameters moved and why, and one click undoes it. Errors: a refused change says why and leaves the tree as it was. [user]
- **R12:** A description takes words and pictures. Errors: a picture the agent cannot read is reported, and the words still stand. [user]
- **R13:** The agent may look up botanical facts during a session and names its source when it does; a claim it is unsure of is said to be unsure. Errors: no source found means the claim is offered as the agent's own reading. [paraphrase]
- **R15:** The owner reaches a tree they would keep from a description without opening the full parameter view, and records that verdict in the spec. Errors: a keep reached only by hand-tuning the full parameters does not count. [user]
- **R16:** One command starts the local studio and opens the page; the studio launches the person's Claude Code headless under their own login and the page shows it ready. Errors: agent not installed, not logged in or crashed shows the reason in the page and leaves the dials working. [user]
- **R17:** The agent can use the studio's tools, web lookups and repository reads, and nothing else. Errors: an attempted file write or shell command is denied and the denial is shown in the chat. [user]
- **R18:** Sessions are files in a git-ignored folder, written as rounds happen; stopping and restarting the studio resumes the session with its rounds and undo intact. Errors: a session from a different generator revision loads flagged as stale, never silently replayed. [user]
- **R19:** The local process accepts connections from the local machine only and refuses a request without the run's token. Errors: refused requests change nothing. [inferred]
- **R20:** The agent adapter is one seam: the studio's tests drive the whole loop with a scripted stand-in agent, with no network and no key. Errors: the isolation guard fails if core, render, Wasm or the published library import studio or caller code. [paraphrase]

## Boundaries
<!-- scope: business -->

- Writing the gap spec and the pull-request path for specs and implementations belong to the follow-on spec that depends on this one. [paraphrase]
- No photograph-matched scoring, automated visual readiness or owner acceptance; those stay in fn-68 and fn-89 for real species. [inferred]
- No hosted model and no key held by the project: the person brings their agent and their key. [paraphrase]
- No new dials, measurements or generator behaviour; the studio exposes what exists. [inferred]
- The add-species pipeline's research, literature and catalogue stages are not exposed here. [inferred]
- Full species research from the UI is a separate spec; here the agent's lookups serve one conversation and write nothing to the catalogue. [user]
- A kept tree is a saved, exportable template; it does not become a catalogue entry here. [user]
- The page's entry view, quick dials, grouped parameters and explainers are fn-96; the agent does not choose or pin quick dials. [user]
- Jev's tuning proposals and the merge of the two dial tables are fn-97. [paraphrase]

## Decision Context

### Motivation
<!-- scope: business -->

Tuning a tree should feel like talking to a collaborator, not a terminal, a dial table and an owner-only pipeline. Owner first, because the owner judges it daily; their keep verdict on one described tree closes the spec. Apply-then-undo over propose-then-approve, so saying it makes it happen. Bring-your-own-key keeps the project from holding or paying for a key. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

The owner first combined the agent connection and Jev's proposals, then on 2026-09-20 split out the page (fn-96) and Jev's proposals (fn-97): R4 and R21 could not be met before fn-68 lands, and the page ships without an agent. R4, R9, R14 and R21 moved with them; the remaining R-IDs keep their numbers. The studio reuses fn-68's candidate shape (one overlay vocabulary) and drops its score (no photograph to measure against). [paraphrase]

Local only, with the studio starting the agent, beat a hosted page with a local helper and attaching a person-started session: one command, nothing to wire, the person's login pays. Claude Code first behind one seam, because the proof needs one agent and the two start differently. Read-only repository access keeps a stranger's first session safe and leaves spec writing to fn-95. No duplication identified (the two dial tables are fn-97's to merge); no back-edge (the studio depends on the library and page, not the reverse). [paraphrase]

## Strategy Alignment

- Serves "The catalogue": a template is a point in one tree space, and a studio where anyone reaches new points is evidence of what the space covers. [strategy:The catalogue]
- Serves "The core and integration" and the Build metric: interactive editing is a named use case for fast generation. [strategy:The core and integration]

## Resolved via Project Docs

- Audience: game and real-time 3D developers first, the owner as first user; interactive editing is a named Build use case (STRATEGY.md).
- Growth stays behind its flag; Jev never runs in the browser source (AGENTS.md).

## Resolved via Codebase

- The harness is a development-only Vite and React page; the published library has no UI (`vite.config.ts`, `package.json`).
- The harness renders dials from one table (`harness/params.ts`, `SLIDERS`: key, label, group, range, step, unit; no explainers), translated by `harness/family.ts`.
- Jev is a local Rust crate with shared caller, ledger and isolation guard (`crates/telperion-jev`), so proposals need a local process.
- fn-68's dial table was not in the repository when written; its overlay is `params::overlay` in the core. Declared capabilities live in the core's `capability` module (fn-84).

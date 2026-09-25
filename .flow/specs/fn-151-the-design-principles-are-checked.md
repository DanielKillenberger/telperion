# The design principles are checked before a push

## Conversation Evidence

> owner (2026-09-25): "let's do it properly and make sure that we have something in place that makes sure these design philosophies really get adhered to. What kind of check would make sense to enforce this? It shouldn't be overbearing but agents should live and breathe this and have their decisions guided."
> owner (2026-09-25): "can we actually have a test suite that we run through jev to check if the diff adheres to our guiding principles."
> owner (2026-09-25): "but why just ci? if we have it part of the local suite we don't have to open a PR first to figure out the mistake"
> owner (2026-09-25): "we could make it a pre_commit hook?" / host: pre-push instead, once per push on the whole branch / owner: "yes"
> owner (2026-09-25): "fn-151 should also make sure that STRATEGY.md is upheld"
> owner (2026-09-25): "for fn-151 we should check through previous PR's identify ones that should definitely have raised flags and make sure that the hook catches it"
> owner (2026-09-25): "so 151 really needs to be designed well. It needs to be token efficient, fast but be highly reliable in catching offending pushes. We should have in agents.md and claude.md (or can we just remove claude.md at this point, claude reads agents.md?) that the design of specs and implementation will be evaluated with jev to align with design principles. Actually as I write this can we run jev on specs before we start work?"

## Goal & Context
<!-- scope: business -->

STRATEGY.md states the principles ("Our approach" and "Key metrics"): one pipeline every tree passes through, never a fallback path, one continuous tree space with no switch, no renderer code per template, generate only the detail the consumer needs, and every change measured for cost and look ("Minimalist af, efficient af and beautiful"). Nothing checks them, so they slip. Three slips this week:
- the slim field crate kept its own copy of the build chain after fn-102;
- the date palm shipped in 0.1.3 unable to grow through `telperion/field`;
- the first fix for that grew the slim Wasm 48% and was stopped only by a manual instruction.

The species runner also grew about 30 stop sites of approval and re-checking. The concrete slips get code guards in the gate. The judgement-shaped ones get a Jev reviewer that reads the branch's diff before every push. Four questions in CLAUDE.md guide the calls no check can make. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25 on master (39348def).** [checked]
- There are no git hooks in the repository and `core.hooksPath` is unset.
- The `jev` binary (`crates/telperion-jev/src/bin/jev.rs`) has the subcommands screen, select, cite, triage, ask and cases.
- Question sets and labelled cases live in `crates/telperion-jev/data/{questions,cases}`, with thresholds in `data/thresholds.json`.
- CLAUDE.md keeps Jev out of every workspace test command, and its key is visible only to an interactive shell (`bash -ic`).

**The principles are STRATEGY.md's, never a second list.** [inferred]
- Code reads STRATEGY.md's "Our approach" and "Key metrics" and takes each commitment as one principle with a stable id. Among them:
  - one continuous tree space, where no family field is a switch;
  - no renderer code for a new parameter or template;
  - one pipeline, with each feature a term inside a stage and never a route around it;
  - an input the pipeline cannot represent is an explicit error, never a fallback path;
  - generate only the detail the consuming engine needs;
  - measured runtime cost and visual evidence;
  - Build, Frame, Fidelity and Attributability as stated.
- **STRATEGY.md's continuity sentence changes with this spec** (owner approved, 2026-09-25: "that's good"). "The generator is one continuous tree space: every generator parameter is a numeric trait that acts on every tree ..." becomes: "The generator is one continuous tree space: every parameter is a numeric trait defined for every tree, a template is a point in that space, and a small change in any parameter makes a small change in the tree. A parameter may be dormant where the structure it shapes is absent, and it wakes smoothly as that structure appears; its description states where it is dormant. No parameter is a switch between ways of building, and a count steps only by one unit of the structure it counts." The rest of the paragraph is unchanged. Under it, a dormant dimension (branching rows on a palm) is clean and a jump is a breach (#59: the first frond removes every other foliage source). [checked]
- The question set's version is STRATEGY.md's hash. An edit to the strategy re-derives the principles and requires the labelled set to be re-run before the reviewer is trusted again.
- The four AGENTS.md questions are a short rendering of these principles, and the spec template's "Strategy Alignment" line must name sections STRATEGY.md has; code checks that.
- Checked on master (39348def), the core pipeline's `placed_field` fallback (`pipeline/stage.rs`) contradicts "never a fallback path". It is the first finding this spec's reviewer must report, and fn-150 removes it from the slim build. [checked]

**Code guards, in `cargo test`** (deterministic, offline, free). [inferred]
1. **One path.** A test lists the only modules allowed to call the build stages (`branching::generate`, `foliage::plan::plan`, leaf placement, the `Field` constructors, `surface::build`); today that is the core pipeline. Any other caller fails with a message naming the principle.
2. **Every preset through every entry.** Each shipped preset builds through the native pipeline, the main Wasm binding and the slim field entry. A new preset or entry joins without editing the test.
3. **Budgets.** A checked-in `budgets.json` records each Wasm artifact's raw and brotli size and each preset's build time and peak memory. Growth past a stated margin fails unless the same change updates the file, so a trade-off is a visible line in the diff.

**The principles reviewer: `jev principles --base <ref>`.** [inferred]
- **Candidates.** Code extracts them from the diff against the base: new callers of build stages, new cargo features and cfg gates, new pause, decision or approval sites, new modules or scripts whose names or public items resemble an existing one, new CLI subcommands, and new or removed presets and package entries.
- **Judgement.** Jev judges each candidate against the four principles, with "touches none" always an answer. Code writes every finding with file and line.
- **Thresholds.** They come from a labelled set drawn from this repository's merged and closed PRs: a survey (`.flow/evidence/fn-151-the-design-principles-are-checked/PR-SURVEY.md`) proposes which PRs should have raised a flag and which are clean, the host labels them and the owner confirms the positives; the set includes at least:
  - fn-102's removal of the copied chains;
  - the slim crate's surviving copy;
  - #115's missing slim support;
  - the conductor's approval layers;
  - the twin reviewer paths;
  - the leaf-plan fallback's +48%;
  - and clean diffs as negatives.
- **Cost.** The result is cached by the diff's hash, so an unchanged diff costs nothing to recheck.

**Specs, before work.** `jev principles --spec <id>` takes each bullet of the spec's Architecture section as a candidate and judges it the same way; the host runs it before `flowctl spec ready`, and the pre-push run covers any `.flow/specs/*.md` in the diff. A design that adds a second path or an approval layer is caught before anyone builds it. [inferred]

**Cost, speed and reliability.** [inferred]
- Code extracts candidates; Jev never reads the whole diff or spec. Each question carries one candidate's few lines of state, and questions are batched up to the caller's limit.
- A run on a typical branch finishes in seconds and costs a handful of calls; the PR reports both from the labelled set's runs.
- Reliability is measured, not assumed: on the labelled set, recall on offending cases is at least 0.9 and the false-flag rate on clean cases is at most 0.1, each reported. A candidate class the extractor misses is a test failure, never a silent pass.

**The labelled PRs (owner confirmed, 2026-09-25: "seems reasonable what you proposed").** [user]
- Positives, which must be flagged:
  - #6: copied build chain in the Wasm binding;
  - #58: the slim field crate's third copy;
  - #55: the placed-leaf fallback;
  - #115: the palm unbuildable through the slim entry (the code guard, not Jev);
  - #3: an enum switching builders;
  - #59: the first frond removes all other foliage, a jump;
  - #52: twin reviewers and assessors;
  - #53: the copied route table and approval pauses;
  - #61: the Claude twin scripts;
  - #13: an unread per-vertex buffer;
  - the runner's approval layers: #38, #75, #76, #77, #83, #87 and #94.
- Clean, which must pass: #17, growth as a sanctioned hidden feature; #50, the GPU executor STRATEGY.md allows; #82, fn-102's cleanup; and the survey's other clean PRs (`.flow/evidence/fn-151-the-design-principles-are-checked/PR-SURVEY.md`).
- STRATEGY.md gains one line on process: a step that stops work must catch defects that the steps around it cannot. The runner positives then cite a stated principle.

**Where it runs.** [inferred]
- A checked-in `.githooks/pre-push` runs the reviewer on the pushed branch's diff against `origin/master`, through `bash -ic` for the key. `scripts/setup` (or the existing setup path) sets `core.hooksPath` once, so every checkout and worktree gets it.
- A finding stops the push with its file, line, principle and a one-line reason. The author fixes it, or pushes with `--no-verify` and names the trade-off in the PR's Decisions section.
- A missing key or network skips with a warning, never a failure.
- CI runs the same command on each PR as advice, for pushes that skipped the hook.

**Guidance.** [inferred]
- AGENTS.md becomes the one instruction file, and CLAUDE.md shrinks to `@AGENTS.md` so Claude Code imports it (today AGENTS.md is a partial, older copy: 66 lines against CLAUDE.md's 109). The file states that every spec and every push is checked by Jev against the design principles.
- AGENTS.md gains four questions: Does this add a second path? Does it compute detail the consumer doesn't read? Does it add a switch to tree space? Is its cost and look measured?
- A PR whose change trades one principle for another names the tension under Decisions (`docs/pr-format.md`).
- The spec template gains a "Principles" line, filled at spec time by the host.

**Unknown.** [unknown]
- Whether `jev principles` belongs in the `jev` binary or the split fn-107 plans (a crate for Jev primitives).
- The labelled set's size needed for a threshold, which the implementer measures.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** On a checkout of master before fn-150, the every-preset guard fails for `date-palm` through the slim entry. Each guard also fails on a scratch branch that breaks it (a direct stage call outside the pipeline, a Wasm over budget) and passes on master after fn-150. [inferred]
- **R1b:** Replayed on the diff of every PR the owner confirmed as a positive, the pre-push check flags it with the principle named; replayed on the confirmed clean PRs, it passes. The replay is a test over stored diffs, runs offline against recorded Jev answers, and a new positive joins by adding its PR number. [inferred]
- **R2:** On the labelled set, recall on offending cases is at least 0.9 and false flags on clean cases at most 0.1; a branch run finishes in seconds with its call count reported; `jev principles` flags the slim crate's copied chain and the +48% fallback diff, and passes a clean diff. It meets the labelled set's accuracy at its threshold and never answers without a candidate. [inferred]
- **R3:** A push from any worktree runs the hook once per push. A cached diff costs no Jev call, a missing key warns and passes, and `--no-verify` skips it. [inferred]
- **R4:** `jev principles --spec` flags a labelled spec that adds a second path or an approval layer and passes fn-150's final design. AGENTS.md is the one instruction file with CLAUDE.md importing it, and it, `docs/pr-format.md` and the spec template carry the guidance above. The workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not the continuity measure (fn-148), which joins the guards when it lands. The reviewer advises and never blocks CI; only the code guards fail the gate. It follows fn-150.

## Strategy Alignment

- Serves "Our approach" and the mantra "Minimalist af, efficient af and beautiful". [strategy:Our approach]

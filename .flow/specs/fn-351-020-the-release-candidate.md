# 0.2.0: the release candidate

## Conversation Evidence

> user (11 Nov 2026): "I want you to actually figure out how to get to a satisfying result for a 0.2.0 release. That reaches my stated goals while being engineered cleanly. I want you to run there autonomously. You may use all the tools and skill at your disposal. You may use flow-next goals or not. I only want you to return to me if you're genuinely stuck."
> user (11 Nov 2026): "you need to consolidate all goal specs and reqs we've specified all over the place and come up with a clean plan on how to resolve this."
> user (11 Nov 2026): "speed i really think we should aim for better but in a way that doesn't get us stuck if it can't be reached with the approach we're doing now"
> user (11 Nov 2026): "i think what you could also do is judge where we need fable intelligence."
> user (11 Nov 2026): "ok prompt pls", then the run prompt: "Run telperion 0.2.0 to a release candidate, autonomously, by the plan I approved on 11 Nov 2026."

Every earlier ruling this goal rests on is in `.flow/evidence/fn-351-020-the-release-candidate/RULINGS.md`, by date, in the owner's words.

## Goal & Context

Telperion 0.2.0 is brought to a release candidate: one grower on the CPU, old trees that stand and keep a crown, species held to real trees, growth cheap enough for the website, and the release's own checks done, ending at a draft pull request for the owner.

This goal is the one source of truth for 0.2.0. It replaces fn-237 (the release map), fn-321 (one grown tree at any scale) and fn-347 (a tree pays only for living wood), which close as superseded by it. The consolidation, the state of each requirement on 11 Nov and the reasoning are `.flow/evidence/fn-321-020-one-grown-tree-at-any-scale-in/PLAN-2026-11-11.md`, approved by the owner on 11 Nov 2026.

## Acceptance Criteria

Who set each line is named. "Host" numbers are the host's and the owner may change them.

### The product

- **P1 (strictly better than 0.1.4, efficient in the website's coarse style):** in a browser on the CPU, every species 0.1.4 has, at its preset age, is measured against 0.1.4's time and reported. The aim is half of 0.1.4's time (host). Owner, 31 Oct and 9 Oct 2026. Speed is an aim and never a stop: see Rigor.
- **P2 (old, tall trees that read as real):** the oak grows to 300 on twelve seeds with no tree refused; at 200 and 300 it stands, carries a living crown to its lower limbs, and reads as an old oak beside reference photographs by the host's eye. Owner, 3 Nov 2026.
- **P3 (the best fidelity for the cost):** native on one thread the oak at 40 and to 300, and a thousand oaks at 40 across the cores, are measured at every landing. The aim is 100 ms, 15 s and 10 s (host). No landing makes a tree slower or heavier than the last unless the tree is better and the cost is stated. Owner, 9 Nov 2026.
- **P4 (growth is the product; a year is a state):** every tree is grown year by year from its seed; the tree at year y is the same whatever age was asked for and whether grown in one call or paused. Owner, 13 Oct 2026.
- **P5 (one grower, on the CPU):** the GPU grower and the tests holding the CPU to it are deleted; every entry grows on the CPU, native and Wasm. Owner, 1, 10 and 11 Nov 2026.
- **P6 (one general law):** no setting, rule or value fitted per species; no preset carries a `/space/coarse` row; every architectural row of a real species names its source or is listed as an estimate, and the count of estimates per preset is reported and lower than on 11 Nov. Owner, 3 and 9 Nov 2026.
- **P7 (no artificial limit):** no cap of the generator's own on how much a tree may grow. Owner, 3 Nov 2026.
- **P8 (dead wood costs nothing):** a test holds that dead wood adds nothing to a tree's upkeep. Owner, 10 Nov 2026.
- **P9 (the space stays continuous):** a walk between any two species at a fixed seed changes the tree by degree, a seed gives the same tree again, and the tests that hold these today hold them at the candidate. STRATEGY.md.
- **P10 (a modular path to engines):** a native entry with no browser, harness or engine dependency takes parameters and a seed, never a species name, and returns the tree's data at any age; a headless test grows the oak through it. No plugin is built. Owner, 1 Nov and 25 Oct 2026.
- **P11 (the website's download):** the growth path downloads at most 250,000 Brotli bytes, and growth runs off the main thread with or without WebGPU. Owner, 1 Nov 2026.
- **P12 (nothing prebaked, no network):** no stored tree, twig or model ships; no network is trained. Owner, 13 Oct 2026.
- **P13 (drawn fast):** the hero oak's GPU time and a forest of 1,000 oaks' are measured in the harness against STRATEGY.md's 2 ms and 4 ms on the RTX 3080 and reported.
- **P14 (the owner's eye):** the owner's written verdict that each real species reads as itself beside its reference photographs, and the oak at 120, 200 and 300. Only the owner gives it; the run prepares the page and cannot meet this itself.
- **P15 (held to real trees):** for each real species at its preset age, and the oak at 120 and 300, on twelve seeds: trunk diameter against age and height, height against age, crown width against height, leaf area index and new shoots a year sit inside the published range where a checked source exists; a measure with no source is reported and counts neither way. Host.

### The release

- **L1:** nothing merges to master, is tagged or is published. The run ends at a draft pull request.
- **L2:** the full gate and `npm test` green; shipped artifacts inside `scripts/artifact-budgets.json`; `cargo fmt --all --check` clean.
- **L3:** fn-237 R2's measured release checks, cold starts with the shader cache off.
- **L4:** `docs/pipeline.md`, `docs/tree-space.md` and the migration notes current; a changelog.
- **L5:** the README's film shows the engine that ships, re-rendered from its script and screened by the host.
- **L6:** the oak on seed 7 no longer needs fixing by the host's eye (owner, 15 Oct 2026: "Oak seed 7 does look like it needs fixing").
- **L7:** a draft pull request in `docs/pr-format.md`'s format, every open item listed, and one page for the owner showing every preset beside 0.1.4 and the 11 Nov build, what is unmet first.

## Boundaries

- Nothing merges to master, nothing is tagged, nothing is published to npm. Merging publishes by itself. Pull request #159 is not written to.
- The host may push the integration branch `goal/fn-321-020-one-grown-tree-at-any-scale-in` and open one draft pull request from it. Nothing else is pushed.
- No value fitted per species by hand, by search or by calibration. No network. No artificial limit. No prebaked tree, twig or model.
- Heavy and GPU work runs through `telperion-heavy`, one GPU job at a time; reviews run inside the queue. Raw output stays in ignored `raw/` directories.
- killenberger.com's repository is not edited.
- No message is sent on the owner's behalf. No plugin is installed. The TypeSafe key is never read; Jev is called only through the shared caller.

## Rigor and stop conditions

- **The scoreboard.** A fixed panel: every preset at its preset age; the oak at 40, 80, 120, 200 and 300; twelve seeds for measures and three for sheets; whole, mid and close views; with 0.1.4 beside it where 0.1.4 can draw the tree. It is redrawn at every landing. A phase lands only if no tree on it is worse than at the last landing, by the host's eye on every sheet, Jev on narrow questions, and the measures.
- **Before a rule is built** the host checks `RULINGS.md`, what the code and the sources already have, and derives the expected result on paper with its arithmetic.
- **Each phase begins with a survey** and ends by redrawing the scoreboard.
- **Speed (owner, 11 Nov).** Missing the aim never stops the run. After counted foliage lands, speed gets its own loop: profile, name the largest cost, remove it, measure, repeat while a round gains a tenth or more. When the approach has no more to give, the run goes on and the candidate reports the gap, what was measured, and the approaches that could close it.
- **Which model does what (owner, 11 Nov).** The host judges per dispatch and logs the choice with its reason. Fable: rule design, arithmetic, sources against their text, phase surveys, verdicts, specs. Opus: building to a written design, measurements, drafts; it stops at any question about how the system should work. Sonnet or Haiku: mechanical work with a checkable result. Codex: review. Work that comes back wrong twice on a task moves up a tier.
- **Path.** Children commit straight onto the integration branch. A child that leaves code in the crates runs the gate once at its close and a codex review of its own diff inside the queue; a child whose commits hold only `.flow/` skips both with the reason recorded.
- **When the run returns to the owner.** Only when stuck: a phase's exit is missed after two different attempts and the host has no third resting on a source or a measurement; or going on would break a ruling in `RULINGS.md`; or free space on `/home` falls under 15 GB. A result does not end the run.
- **It does not check in.** Progress goes to this goal's log and one owner's page updated in place. Times come from the clock; unknowns are written as unknown.
- **Limits.** No limit on children.

## Build order

- **Phase 0, one source of truth:** this spec; the closures; `RULINGS.md`; the routing lines; the format fix; the scoreboard on the 11 Nov build beside 0.1.4.
- **Phase 1, one grower (P5):** delete the GPU grower and its tests; the harness and the website draw CPU-grown trees. The 11 Nov port in fn-349 is dropped.
- **Phase 2, wood that lasts (P2, P8, L6):** land the budget fix; girth as living sapwood plus kept heartwood, the pipe sized from measured sapwood per leaf area, the `retained` setting deleted; shed branches buried smoothly; sag reads that wood. Exit: the oak stands at 200 and 300 on twelve seeds, its trunk inside White's range, no young tree worse.
- **Phase 3, a crown that renews (P2):** sleeping buds that persist on old wood and wake by light; two tries, no more, at the jagged limb line. Exit: the oak at 200 and 300 holds a living crown to its lower limbs.
- **Phase 4, species on sources (P6, P15):** each real species' shoot and leaf tables from measurements; every row sourced or listed as an estimate. Exit: P15's measures in range where a source exists.
- **Phase 5, counted foliage (P1, P3, P6):** leafy shoots that cannot leave a light cell carried as counts; a twig built from its key when a view resolves it; a proof on the oak, spruce and beech before the product is touched; the per-species coarse rows deleted. Exit: the scoreboard no worse and every tree cheaper than at the last landing; then the speed loop.
- **Phase 6, drawn fast, the website and the entry (P1, P10, P11, P13):** levels of detail from the structure's own order; the voxel grove from the tree's cells; growth off the main thread; the 250 KB budget; the native entry; the forest rig and the frame measures.
- **Phase 7, the candidate (L1 to L7):** dead code and settings removed; docs and changelog; the film; the full gate; the release checks; a codex review of the whole diff; the push, the draft pull request and the owner's page.

## Decision Context

Approved by the owner with the plan on 11 Nov 2026: AGENTS.md and STRATEGY.md say that leafy shoots confined to a light cell are counted by the same law and built from their key when seen, which is the growth and not a shortcut; fn-279, fn-280 and fn-267 wait until after the release unless the scoreboard shows their fault on a shipped preset, and fn-307's repro is run once and decides it; the film is re-rendered once the trees are final; a new draft pull request is opened and #159 left to the owner; the Two Trees take a potential height from their base species and are re-screened.

What went wrong before this goal, so it is not repeated: the oak's values were fitted around artefacts (dead wood charged upkeep, a pipe sized so the living crown alone makes the trunk, limbs made by timers), and fixing them in the order they were noticed exposed one after another; rules were invented where the code or the literature already had one; approaches were judged at the end with no standing scoreboard; and the host lost a ruling (the GPU grower's deletion).

## Open Questions

- The aim numbers in P1 and P3 are the host's.
- P15's twelve seeds are the host's.

## Goal log

- 2026-11-11 12:58 Zurich: goal written from the approved plan. Phase 0 begins.
- 2026-11-11 13:10 Zurich: Phase 0, the paperwork. `RULINGS.md` written. Closed as delivered: fn-191, 192, 198, 200, 220, 230, 272, 283, 294, 297, 298, 299, 301, 302, 303, 304, 306, 308, 309, 310, 311, 312, 313, 314, 316, 317, 319, 350. Retired as superseded: fn-190 (by fn-191), fn-295 (by fn-309), fn-237, 321, 347, 242, 273, 285, 265 (by this goal). Retired as moot: fn-268, 289, 300, 318. fn-349 is this goal's child. Working evidence of the closed specs is not pruned in this step. AGENTS.md carries the routing lines and the sentence on counting; STRATEGY.md's sentence on the persistent structure is reworded, both as the approved plan's section 4 says. Not surveyed, left open and unknown: the open specs numbered below fn-190.
- 2026-11-11 13:01 Zurich: Phase 0 dispatches, with the model and why. fn-352 (the scoreboard) to Opus: a runner built to a panel the host wrote. The survey of what depends on the GPU grower to a Fable agent: it opens Phase 1 and an error in it sends the removal the wrong way. The open specs below fn-190 to a Sonnet scout: an inventory with a checkable result. Host decision: the format fix moves to Phase 1's close, since formatting code that Phase 1 deletes is work thrown away, and Phase 1 ends in a full gate anyway.
- 2026-11-11 13:05 Zurich: the 49 open specs below fn-190 were read by a Sonnet scout (goal, criteria and endings; not every section). None states a 0.2.0 need this goal lacks. Two owner rules from them are added to `RULINGS.md` because they bind Phase 6: the far draw is the near draw minus what the eye cannot resolve (fn-176, 18 Sep), and growth decides where leaves are so nothing is culled after (fn-125, 3 Oct). Left open and untouched, since closing them is outside this goal: about 20 the scout reads as written for the old generator or the GPU path, 5 as delivered, the rest as later features. Unknown: fn-98 (every parameter combination grows a tree) has no repro on the new engine. Not covered by this goal and the owner's wish: a tree that grows smoothly as the page scrolls (fn-28); P4 gives yearly states, no blend between them.
- 2026-11-11 13:10 Zurich: Phase 1 opened. The survey (`PHASE1-SURVEY.md`, a Fable agent) found that the device expansion reads one buffer only the device grower writes, and that a full CPU path to the screen is already production (`mesh::build`, `Renderer::submit`). Host decisions, in fn-353: all of the render crate's `generation/` goes, about 51,500 of its 70,000 lines, not only the grower, since keeping the device expansion would need a second path from a CPU tree to the screen; the browser grows on the main thread through `setTree` until Phase 6's Worker; `telperion/field` grows on the CPU now and works without WebGPU; grow-in and play-the-year stay on a new binding over `pipeline::growth`; the page budget and "makeRoom" go. The cost, stated: the harness's build per dial move goes from the device's 0.2 to 1.3 s to full CPU growth (oak 3.6 s, spruce 11.9 s in Wasm) until Phases 4 and 5; no tree's own growth cost changes. Built by Opus in its own worktree `.worktrees/fn-353`, because the scoreboard is measuring the integration branch's build.
- 2026-11-11 13:11 Zurich: dispatches, with the model and why. fn-353 task 1 (bindings, harness, field entry) to Opus in `.worktrees/fn-353`: building to the host's written decisions. The Phase 2 survey (how a radius is computed and read today, why kept pipes draw stepped rings, and the measured botany of sapwood per leaf area, heartwood, trunk girth by age, branch burial and green-wood stiffness, each checked against its text) to a Fable agent: it is the base of the girth rule's design, where the last four attempts went wrong. It runs beside Phase 1 because it reads and builds nothing.
- 2026-11-11 13:16 Zurich: a finding from fn-353's first task, measured: P4 does not hold today on either grower. The grower takes the age asked for as its horizon (`grow.rs:201` builds the presence windows from it; `grow/cycle/paused.rs:203` lays out light for the years before it), so the beech at seed 7 grown toward 12 differs at every year before 12 from the beech asked for at that age (year 10: 2,870 nodes and 853 leaves against 2,849 and 851; year 1: root radius 0.016549 m against 0.015964 m) and is equal only at the horizon. Grow-in has always shown the tree on its way to its final age. Host decision: fn-353 binds growth as it is and its test holds what is true (repeat, pause and resume, equality at the horizon); P4 gets its own child of this goal after Phase 1, surveyed first, since making the grower ignore the horizon changes how trees grow. Also decided: the browser's growth handle owns its state by shared ownership, with no `unsafe`.
- 2026-11-11 13:28 Zurich: the Phase 2 survey is read (`PHASE2-SURVEY.md`) and the design is on paper (`PHASE2-DESIGN.md`); nothing is built. What the survey changed: the host's proposed measurement of 11 Nov (keep every pipe, scale the pipe down) is withdrawn before it ran, since on paper it overshoots real oaks at 80 and 120 by up to 1.9 times, a pipe being laid per held phytomer and not per leaf; no girth or sag row in any preset has a source; and girth and sag were fitted against each other (limbs about twice too thick, sag rows 10 to 60 times softer than green oak). The rule designed: a section is heartwood plus sapwood; sapwood is the leaf above over the species' measured leaf area per sapwood area; one Nth of it retires to heartwood a year, N the species' measured sapwood ring count; the tree lays what it is short of and never loses wood. Two measured traits per species replace six rows per growth stage. On paper the oak's trunk is 30, 72, 103, 144, 175 cm at 40 to 300, inside White's open-grown range at every age. Sag reads that wood at green wood's own stiffness (`4ρg/E`, measured E and density). The estimates are named in the design. A dependency found: wood sized from leaf cannot land for the beech while its leaf is a fifth of a real beech's, so Phase 2 takes the leaf half of Phase 4 with it. A probe comes before any product code, with what would refute it written down.
- 2026-11-11 13:45 Zurich: two more surveys read, both by Fable agents, both saved beside this goal. `P4-SURVEY.md`: the requested age reaches growth through the presence windows (fn-192's decision 3 normalises each draw's share by the expected tree at the requested age) and through sleeping buds that would wake after it; the settle reads nothing after the year grown; the light layout skipped in the last year is deferred work. The fix is its own child after Phase 1. `LEAF-SURVEY.md`: three leaf accounts exist with nothing tying them together (the grower's deposit, the drawn stations, the measure fn-339 used), disagreeing by 1.2 to 2.4 times; the beech's deficit is real and lies in shoot count, its branching shares having been read from understorey trees; the spruce's "27 times too many shoots", which the host repeated to the owner on 11 Nov, is mostly a mismatch of tree size (an open 7 m crown against stand trees of a fifth the crown) with a real factor nearer 3 that rests on one disputed source; the oak's leaf is close to real; the birch was never measured. Consequences for the plan, host: before wood is sized from leaf there must be one leaf account, the drawn leaf being the grower's leaf; the comparison with real trees is made per crown projection; the beech's branching is Phase 2's dependency as already logged.
- 2026-11-11 13:48 Zurich: fn-353 task 1 done on its branch (`8a3a20b74..0daf5e02d`): the grower owns its species (digests unchanged, 774 space and core tests green), the browser's growth calls are CPU calls over `pipeline::growth`, the harness builds and grows through them, and `telperion/field` returns its grid from the CPU with or without WebGPU (checked in Node without WebGPU). The oak at 40, seed 1001, is unchanged at 272,095 nodes and 47,279 leaves. Not verified, because agents do not drive WebGPU in a browser, and carried to the candidate's list for the owner: a grown year reaching the canvas with its colouring, "grow in", "play the year", a build after it, and the feel of a CPU build on the main thread. The host's stop rule on timing (2% either way) was tighter than the runs' own 8 to 13% spread and stopped the builder once for nothing; withdrawn. Task 2 (the deletion itself) dispatched to Opus.

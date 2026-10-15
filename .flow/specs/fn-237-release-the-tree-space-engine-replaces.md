# Release: the tree-space engine replaces 0.1.4, everywhere

## Goal & Context

The owner's goal (2026-10-08/09): "replace the old generator with the new one that's higher fidelity", "make it all the way. Ship it all together": the engine, every species (oak, spruce, beech, birch, palm, ordinary, Laurelin, Telperion), the harness and killenberger.com's move off telperion 0.1.4, in one release PR from the F4 integration branch (fn-198) to master, with nothing merged to master without the owner. This spec holds no code: it is the release map. Its dependencies are every spec the release needs; `flowctl` and `/flow-next:deps` on it show what blocks what. fn-198 task 9 (release checks, the PR) runs when every dependency here is done.

## Requirements

- **R0 (the release bar; owner, 2026-10-09):** "I would really like to ship something that is at least as good as the previous version." Every shipped species is at least as good as master on every axis the scorecard measures (harness build time, frame rate whole and close, GPU memory, the site's per-tree time and download, and the owner's verdict on old-versus-new stills), or the part that is worse does not ship (for example the site stays on 0.1.4) until it is. Refined by the owner the same day: "if we get way better fidelity for slightly less performance that's a trade we can live with. But as of now performance is way worse. Hard to justify." So a species may trade a slight performance loss for a clearly better look the owner confirms on the stills; a large performance loss is not traded.

- **R0a (the bar as set, owner 2026-10-13):** per-tree ratios to 0.1.4 are replaced by absolute budgets ("Your budgets seem reasonable"): every shipped tree ready in under 0.5 s warm in the harness on the owner's RTX 3080; Telperion and Laurelin named exceptions at about 1.25 s warm; the site's per-tree field after the once-per-page device start and compile (fn-257 R3-0) at or under 0.1.4's per-tree time; the owner's verdict on each species' stills.
- **R1:** every dependency of this spec done and merged into F4.
- **R2:** fn-198 task 9's release checks pass (harness timing against master, the site's field against 0.1.4, sizes, docs, npm version) and the release PR is opened per docs/pr-format.md.
- **R3:** the owner's verdict on each species and on the site's grove recorded.

- **R4 (found on the way):** fn-234's measurement found the resident device path's tree reaching 0.227 m below ground against the CPU's 0.040 m (the cherry beech at 40); checked in the release checks, and a spec minted if it is a defect.
- **R5 (found on the way):** `src/field/field.test.ts` "grows every shipped preset, byte for byte the main entry's field" runs about 60 s against its 60 s timeout; it timed out under load twice (fn-217's merge at load 58, F4 after fn-234 at load ~30) and passes alone. Before the release PR, made to finish well inside its timeout (or its timeout set from a measured CI run), so CI does not flake on it.
- **R6 (found on the way):** fn-235's cold-build test holds the first tree to 5 s besides `grow` with about 0.3 s of margin after the fn-231 merge (MERGE-235.md), so it may fail under load; before the release PR, the margin restored (MERGE-235 names one route: the lateral place threshold's logarithm from the CPU table instead of in `asks`) or the bound restated from a measured CI run.
- **R7 (friction, obvious fix):** `cargo fmt --all --check` reports 79 unformatted spots that predate the work, so every agent's `cargo fmt -p` rewrites files it never touched (fn-235, fn-241, fn-213 FRICTION.md). Once the last open branch merges, F4 is formatted once in a commit of its own, with no other change, so no open branch inherits the conflicts. The same pass clears `cargo clippy -p telperion-core`'s 26 errors that predate the work (fn-248 FRICTION.md).
- **R8 (found on the way):** the render module is 3,295,462 bytes against its 2,800,000-byte ceiling (`scripts/artifact-budgets.json`, fn-243's report); measured on F4 before fn-243 to see what moved it, then brought under or the ceiling restated with the owner.
- **R9 (found on the way):** fn-235's cold-build test times `grow`'s compile against its 20 s exception, and failed at 20.6-26.0 s under load from other sessions (fn-246 FRICTION.md) while passing alone; a compile time measures the machine's load as much as the shader. Before the release PR, the test runs as an on-demand probe (like the device-limit probes, `docs/gpu-tests.md`) in the release checks on a quiet machine, and the default suite keeps a load-independent guard (the shader's size or call-site count) instead.

- **R10 (friction, obvious fix; fn-260 FRICTION.md):** `telperion-jev`'s replay test needs a `telperion-core` example already built, so it fails when the gate runs package by package in the documented order; before the release PR it builds what it needs itself (or the dependency is declared so cargo builds it), and the package-by-package gate passes from a clean target.

- **R11 (found on the way, fn-254):** the catalogue's pinned values for the beech, spruce, oak and birch no longer match the engine (up to 4.8 m apart) and no test reads them; before the release PR they are either read by a test and re-pinned with evidence, or deleted (docs/principles.md, "Question, delete, then optimise").

- **R12 (friction, obvious fix; fn-261 FRICTION.md):** a render test that takes `generation::whole_tree()` and then calls into `roots::kernels` deadlocks on the same non-reentrant test mutex (`_one`) and holds the GPU lock for its timeout (10 minutes lost in fn-261); before the release PR, the guard is taken in one place, so a test cannot take it twice, or a second take fails at once with a message naming the guard.

- **R13 (friction, obvious fix; fn-253 FRICTION.md):** a still sheet an owner verdict rests on is kept beside the spec's evidence outside the ignored `raw/` (a compressed JPEG), so a later spec can compare against the approved look without rebuilding it in another worktree; `docs/evidence-retention.md` says so before the release PR.

- **R16 (owner, 2026-10-13: "we can do it with release and embed it into the repos readme while we're at it"):** the release film, fn-272: through species space with trees growing and shrinking, embedded in the README. fn-271 (the local gate in minutes) also rides the release.
- **R14 (found on the way, fn-257's probe):** fn-260's device-free shader check validates with naga only; Chrome's Tint rejected three things naga accepted (a NaN constant from `bitcast`, a barrier under a non-uniform loop bound, 17 storage buffers in one stage against Chrome's 16). Before the release PR, every shipped WGSL module is also checked against Tint's rules (Tint itself, or checks for these classes with a red fixture each), and against 16 storage buffers per stage.

- **R15 (friction, obvious fix; fn-256 task 13 FRICTION.md):** `scripts/build-wasm.mjs` ignores `CARGO_TARGET_DIR` and reads a stale Wasm from the default target, so `npm test` in a worktree with its own target dir can test old code; before the release PR it honours `CARGO_TARGET_DIR` (or fails by name when the Wasm it reads is older than the sources).

## The owner's species verdicts (R3, 2026-10-15, review page `Kfsy2fgr5B2tJ4uiE1qxae`)

| Species | Verdict | The owner's note |
|---|---|---|
| Ordinary | approve | |
| Date palm | approve | |
| Oregon white oak | needs work | "seed 7 specifically doesn't look like a real tree. Lower branches that grow to long and zigzag weirdly." |
| European beech | needs work, after the release | "they really aren't full enough.. We need to be able to have big crowns with leader branches and full foliage. But i think we can move that work to after release." |
| Silver birch | needs work | "We still need to fix the ground branches and here also we need to be able to make larger species with big leading branches. But it's the best species." |
| Norway spruce | needs work | "Looks really good but still needs fuller curtains and larger species." |
| Telperion | needs work, accepted for now | "Doesn't look magical enough and flowers look too much like copy pasted. Not natural. But can accept for now" |
| Laurelin | needs work | "Looks better than telperion but still flowers are too samey all over. and doesn't look magical enough." |

The owner on the four that ask for size: "i think all have a similar problem is that we're not efficient enough with the generator to make large species at full fidelity. We need to have a plan on how to figure this out post release." That plan is fn-285. The birch's ground branches and the oak's long low branches are fn-282's ground. Open with the owner: whether the oak at seed 7, the birch's ground branches and the flowers of Telperion and Laurelin hold the release.

## Boundaries

Map only. Post-release: fn-232, fn-233, fn-285.

## Housekeeping decisions (host, 2026-10-13, branch fn-237-housekeeping)

- R10, R12, R13, R15 done; R14's storage half done (8 storage buffers per stage, red and green fixtures).
- R5 superseded by fn-241 R2 (one field test per preset), closed once a full `npm test` confirms every preset under its timeout. R6 superseded by `691620d04` (2.5 s per pipeline; the first tree's total reported only).
- R9 and R14's Tint half wait for fn-270's merge (both live in `generation/`): R9's load-dependent compile timing is replaced by a load-free guard then; the Tint checker extends to every shipped module, and its two findings (barriers in `select`, whose bound is a constant, and `positions`) are checked against Tint itself before either is fixed or exempted.
- Species audit: SPECIES-AUDIT.md; what is left is mostly the owner's verdicts and re-sheets on the current engine, gathered into the release's stills session.

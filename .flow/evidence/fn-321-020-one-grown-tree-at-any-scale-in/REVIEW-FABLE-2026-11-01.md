# Fable's review of fn-321, 1 Nov 2026

The host's condensed record of Fable's report, from the brief in `REVIEW-BRIEF-2026-11-01.md`. Verdict: **NOT READY**. Astra's full report, also NOT READY, is `REVIEW-ASTRA-2026-11-01.md`.

## Findings

- **P0, integration branch:** goal mode creates the branch from `origin/master`, 2,500 commits behind this checkout (measured), and fn-319 is unpushed. Create and push it by hand from the fn-319 head with the goal spec committed, never merge master, open any final pull request as a draft, and do not pass `--until=merge`.
- **P0, dirty tree:** `auto.md` stops at the start on `docs/typesafe.md`'s uncommitted change. Commit everything first.
- **P1, gate:** the gate fails without `node_modules`, and this worktree has none (measured). Run `npm ci` once per checkout.
- **P1, R2:** "WebGPU disabled" requires a CPU executor in the browser, against the owner's 11 Oct 2026 ruling ("a browser without WebGPU gets a named error, never a CPU fallback"), and no quoted owner line asks for it. Strike it or ask the owner.
- **P1, R4:** at n = 50 the IQR's sampling noise fails a 20 % bound even against an exact copy (inferred). Use the larger of 20 % and the reference's own split-half difference. `species_metrics` has no limb count or shed share (read from code).
- **P1, R9 and R2:** `telperion-render.wasm` is 835,720 Brotli bytes (measured), so 250 KB is fn-242's whole scope plus the route. Keep the bar, measure it per child, and never let it gate a child.
- **P1, resources:** 44 GB free, 20 GB of raw data, a 15 GB stop and 2-3 GB of `target/` per worktree leave no margin. Lower raw data to 10 GB.
- **P2, R1:** the grid is a field-only input, and mesh entries have none. Name the parameter in what the tree resolves and test phytomers across three values.
- **P2, R3:** no forest rig exists (grep). Build one or drop the frame half.
- **P2, child 1:** name the costs: scaffold-only CPU cost per year at 40, 80 and 300, and the CPU light cost separately.
- **P2, expectation:** with a 27-minute gate and a review per child, 18 hours fits child 1, child 2 and perhaps one more. Expect `NEEDS_HUMAN` at 19:00 with R4 numbers, not a tree.
- **P2, naming:** say Astra runs through `codex exec`.

## The route

Sound as a research step, and the order is right. The strongest reason it fails (inferred, COUNCIL.md lines 43 and 123-124): low-limb shedding and the light field depend on the twig density the closure replaces, and a table conditioned on local scaffold state is the same aggregate with learned coefficients; nothing yet shows it carries that feedback. Form (d), the only exact result, kept all the growth work. The route stop should also count CPU scaffold cost.

## Checked and sound

The baseline numbers; grid not changing cost (`make.rs`); the 2 GiB split (`cycle/state.rs`, one measured refusal); eight presets; the oak's preset age of 40; R8 buildable, since `telperion-core` depends only on `telperion-space`; the fn-268 and fn-285 citations; R2's benchmark rebuildable with two path edits and a no-WebGPU variant; `telperion-heavy`; 8 GB of 10 GB GPU memory free; children committed straight within goal mode's "How strict".

## Not checked

The 2026-10-05 decisions; 0.1.4's browser times for Telperion and Laurelin; `flowctl spec set-plan` in this install (the host has since run it); the fieldspike tooling's build state; GPU timer precision for R6's 2 ms outside Chrome developer flags (`GrowerDev.tsx` warns of 100 µs quantisation).

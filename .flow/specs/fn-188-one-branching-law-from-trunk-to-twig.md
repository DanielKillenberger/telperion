# One branching law from trunk to twig

## Goal

Mature trees whose structure reads like the real species, the beech first: a few thick limbs dividing progressively into finer branches, ending in a dense mesh of fine twigs at the periphery and along the limbs, on a clear trunk, fast enough for runtime. Spec after spec (fn-61, fn-170, fn-177, fn-179, fn-180, fn-183, fn-182, after fn-34's 23 rounds) fixed one symptom and exposed the next, and most defects sat on one seam: the scaffold and the twig layer joined at a crossover (horns from the tip hand-off, bare thick limbs from the `limbRadius` gate, thickness that does not fall because the pipe solve stops at the crossover, the crown wall, clean-ups after the fact). The owner asked to step back and question the assumption, probe first, no production code.

## Decisions

- user (2026-10-02): "let's take a step back and look at the problem from the start. What are we trying to do? Why haven't we been able to get there? Is there something that we fundamentally have to change? something to delete? an assumption to question?"
- user (2026-10-02, on questioning the scaffold/twig split, fixing the judging lens first, probing one branching law before any production code): "yes that sounds good"
- Astra's challenge (2026-10-02): the beech uses no attractor competition (`attractorWeight` 0), so its structure is rule-generated; the seam explains part, not all; the "86% under a pixel" was projected width, not proven lost coverage. Steps revised: lens, small ablations, then a probe only if a topology defect remains.
- owner (2026-10-02): speed is a product requirement, "in the long term it needs to be super fast"; a candidate with no credible path to being faster than today is rejected.
- owner (2026-10-02), after the probe and the tip trace: "i agree that the question about the fraction of the trunk is another arbitrary rule like the crown border checks. We should have a branching rule that makes them grow organically according to the params set."
- Overnight rounds (host, 2026-10-03) stopped at the stopping rule: the remaining gap is design for the owner.
- Closed into fn-190 (owner, 2026-10-03): competition growth is the base; the missing piece is axis differentiation by physiological age; one production spec for a wide array of trees.

## Acceptance Criteria

- **R1:** A judging lens with the capture's sample count stated and coverage loss shown or ruled out — met: headless captures run at 4x MSAA; stills keep twig tone; a pinned beech seed-1 pose for every candidate.
- **R2:** Ablations measured through the lens — met: (b) deleting the tip continuation is a clean win (trunk path 31.87 to 27.27 m against a 32 m envelope, pole 4.14 to 0.49 m, nodes −12%, skeleton 71 to 56 ms; built as fn-189); (a) whole-tree girth barely moves daughter ratios; (c) axis-relative forks give the mesh but run away (4 to 71 leaders). A topology defect remained.
- **R3:** The competitive-growth probe — met over nine throwaway rounds (code committed and reverted on branch `fn-188-one-branching-law-from-trunk-to-twig`).
- **R4:** The owner decides in order — met: closed into fn-190.

## Result

Probe rounds (Pałubicki et al. 2009 family: buds, space markers, Borchert-Honda vigour, whole-tree pipe radii):
- Round 1: graded fine wood along thick limbs for the first time, but limb count jumped (2-3 to 33+ between apical preference 0.48 and 0.5), envelope-wall outline, 4 to 6 times slower.
- Rounds 2-3: grown from the scaffold with no gate, tip shoot or wall; with buds along every metamer and space markers as the resource the mesh is even, the edge ragged, growth stops by itself, λ continuous, 122-128 ms.
- Speed round: grid, flat markers, threads, early stop: beech 55 ms (16 threads), Telperion 15x faster.
- Rounds 4-7: straightness, pipeline foliage, leaf-pipe radii, four leaders, plagiotropy; best bare result in round 5; angles move by degree.
- Adapter round: unique node identities removed the repeated leaf pattern (20% to 0.5%), coverage 0.889 in the reference's range.
- Rounds 8-9 and the bare scorecard are fn-190's starting point.

Where it stood: competition removes the thickness gate, the tip shoot and the girth patches, grows a graded mesh and stops at the crown on its own; the beech still read as a uniformly filled egg with sleeved branches. Beech fine structure needs short/long shoot differentiation, shoot-local two-ranked sprays and positional lateral development (mechanisms, not values). Foliage expansion was 91% of probe-plus-mesh time on the beech (Laurelin 7 s), the larger speed opportunity.

## Open

All carried by fn-190.

## Friction

- Round measuring tools were never committed (~10 min, ~25k tokens to recover) -> commit and revert each round's tool beside its numbers.
- dcg blocked redirects to variable paths; `/usr/bin/time` missing -> local setup, reported.
- Restoring a reverted probe on the spec branch was awkward -> keep a probe on its own scratch branch or worktree.
- Round 8's run settings were not recorded -> each probe row now carries its environment.
- The bare beech photograph was too small and cropped for image measures -> a higher-resolution open-sky reference (fn-190's references).
- A research agent waited an hour on a dead background job; agents overwrote a shared scratch file; Commons rate-limited parallel downloads -> foreground downloads with timeouts, one scratch folder per agent, sequential downloads with backoff.

## Evidence

- `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/LITERATURE.md` (architecture literature brief, cited by fn-190 to fn-198).
- `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/ASTRA-BARE-TARGETS.md` (the bare scorecard).
- `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/R3-PROBE.md`, `R3-SPEED.md`, `R3-ADAPTER.md`, `R3-PROBE9.md` (the first round, speed, adapter and latest round, cited by fn-190).

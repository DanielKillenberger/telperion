# Tree space: one reference axis, every setting by degree

## Goal

The programme's first claim is that every species is a point in one continuous space (`docs/tree-space.md`). Each species had been written on its own list of physiological ages with whole-number settings that step rather than slide, so fn-196 found a walk from the oak to the palm had no midpoint; every species spec's R3 walk (fn-194, fn-195, fn-196) depends on fixing that. AmapSim's reference axis is the botanical frame: one ordered chain of physiological ages every axis travels at its species' own rate (host, 2026-10-05). Structure only; dressing continuity is fn-198's.

## Decisions

Host, 2026-10-05, on the R0 proposal (`.flow/evidence/fn-206-tree-space-one-reference-axis-every/PROPOSAL.md`, N = 15 ages):

1. Each axis is keyed by its chain position; byte identity is a preference, so R1 is "unchanged in look" with a pixel diff.
2. A stop is a probability of continuing at the end of the lifespan; one mechanism (a `continuation` share).
3. Zones carry fixed roles: 0 bare base, 1 medial, 2 top whorl, 3 spare.
4. The oak's low fork and the beech's top fork stay separate ages.
5. "Unchanged in look" means the same species law, not the same individual; re-keyed seeds accepted where seed statistics match within spread.
6. A fractional lifespan ends with its last unit at share f and the next age's first unit at 1 − f in the same cycle; growth units keyed by the cycle they grow in; abortion hazard counts real time; closed form, `Living` and dormant buds follow.
7. Canonical form: an unused age carries its nearest used age's settings with no lifespan and no laterals.
8. "Never moves on" is continuation 0 on a finite lifespan (the 1000 sentinel removed); positive magnitudes mix on a log scale, angles/probabilities/shares linearly, from one scale table every setting must appear in (`species/scale.rs`).
9. The spruce's 0.14 m height change from continuous node bounds is accepted.
10. (decision 11) A stop all but made grows both outcomes; (12) a stop's outcome is one three-way distribution (carry on, relay, die); (13) the closed form counts the extra relay a move all but made grows.
11. (15) Oak to palm's early crown loss is accepted as inherent (branching compounds geometrically), not a pop; calibration of the lateral-probability scale is open for later. (16) Beech to spruce renders at age 40; shedding before growth (fn-210) is the remedy at 60.
12. (17) Codex findings that change no species or strip and sit only at an exact boundary are deferred. (18) Oak-to-palm stem girth swell is a calibration note. (19) Response exponents mix linearly. (20) The beech's extra wood at 60 years (length +32 %, axes +24 %, base radius +14 %, ~2 SE, height unchanged) is accepted as fn-206's change of law; the beech's pass stands.

## Requirements

- **R0:** Proposal listing every setting's continuous form, neutral value and closed-form effect, chain length and mapping, cost; host-reviewed — met (PROPOSAL.md).
- **R1:** Shared chain and continuous forms; beech, spruce, oak (round 4) and palm on the chain, unchanged in look at seeds 1 and 7 with a pixel diff; renumbering its own commit — met under decisions 5 and 6.
- **R2:** Closed form and A's oracle hold; walk tests for every new continuous form within bound 30, real-valued lifespan red first on the count-keyed form — met.
- **R3:** Bare strips beech → spruce → oak → palm at a fixed seed, viewed: no visible pop; then a five-seed beech sheet against `final21` — met (strips at seed 1, no pops).
- **R4:** Workspace gate; Codex review — met.

## Result

- Built: 15-age canonical chain, real buds, node bounds, shedding, fractional lifespans, the scale table, stop outcomes as shares, the closed form counting every grown element. A relay now stands at its share of the unit's scaled length (`b172c2d4`, also resolves fn-199's R2).
- Final strips (seed 1): beech to spruce at 40 (10.1 to 12.7 m, the host passed `walk5-40`), spruce to oak at 60 (16.9 to 19.2 to 17.6 m), oak to palm at 60 (17.6 to 22.6 m; crown 54 % left by t = 0.01, 1.3 % by 0.125 per decision 15). No pops. Strips were in ignored `raw/`.
- Codex SHIP after nine rounds; gate 1,071/0 at `fd7d9ffe`. Port onto master: Codex SHIP, gate 1,095/0 at `bc832d12`.

## Open

Deferred to continuity hardening (beside fn-199; also in `.flow/review-deferred/`): at a whole-number lifespan ± 1e-9, a vanishing end-of-age relay applies its full insertion angle (`grow/stop.rs`, a 0.686 m tip jump in the repro) and still counts toward `ended_relays` (`grow.rs`). Calibration of the lateral-probability scale (decision 15), oak-to-palm girth (18) and the beech's wood at 60 (20).

## Friction

- dcg hook blocks `git show ref:path > path` -> none needed; write via a script.
- wgpu out-of-memory on 80-year spruce stills (four retries, two stills missing) -> GPU memory budgeting or lighter LOD for comparison stills.
- Base-vs-branch statistics needed a second checkout -> a `space_stats` example per commit.
- Fractional-lifespan design question stopped the task -> answered by decisions 6 and 7.
- Each re-keying redrew every walk and fixture tree, surfacing latent steps (~4 hours) -> multi-seed walk and fixture tests, fixtures no key moves.
- `walk.sh` retried deterministic budget refusals as GPU faults (~15 min of lock time) -> fixed in the script; renderer could exit with a distinct code.
- telperion-core beech memory ceiling (7.30 vs 7.20 GB) failed under the gate, passed alone -> margin independent of machine load.
- Session tmpfs quota filled by other specs' scratch -> large renders on disk; clean finished scratch.

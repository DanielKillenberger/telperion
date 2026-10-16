# Codominant forks: one rule from the ground to the crown

## Goal

The owner's photograph from Basel's Schützenmattpark is a London plane (*Platanus × hispanica*): a short trunk forking at about three metres into three or four limbs of near equal girth, forking again higher up, each thick almost to the crown's edge. The generator could not draw it: a tree got more than one trunk only through the stems rows, placed at the root or low on the bole. The owner read the stems rows as one case of a general rule: a clump is a codominant fork at height zero, the plane's forks are the same event higher and repeated, and the seed decides per specimen whether a fork happens. The spec also absorbed fn-103 ("Leaders keep their girth"), whose single-continuation assumption a codominant fork breaks.

## Decisions

- user (2026-09-27): "I feel our generator fundamentally lacks the ability to have strong limbs that quickly taper at the edge of the crown to leaf level. Do we have the ability to create trees like this? if not do we have a spec to fix that?"
- user: "I also think currently the multple stem slider is probably not good design. We should have branching rules that make it more likely to grow equivelantly large stems to the first one."
- user: "but isn't 3 just another instance of a codominant fork happening lower in the tree? this seems like it should be one rule with different params. Then you can also randomly have different looking trees from the same species where some have codominant forks a the bottom some don't. that seems much more like trees work."
- user: "what's the separate taper profile spec for and don't we have a spec for that already?" ... "that makes sense"
- owner (2026-09-27, beech session): the beech needs fn-170 as well as fn-61 to achieve its look; "take over with your plan"
- Host decisions (2026-09-27, after Astra's review):
  - One fork decision per structural axis from its own keyed stream: a target height drawn from a bell (centre `forkHeight`, width `forkHeightSpread`) over the tree's height share, crossed at least one station above its birth; only the root forks at its base (a clump). No "fork where the axis stopped" fallback. Each part draws its own next fork, so forks repeat up the crown. The stem-placement path is removed.
  - Forks grow in, never pop: siblings take a weight ramping 0 to 1 over 0.05 of the rate; `forkWays` is real (2 to 4), its fraction the weight of one more part; parts fill four fixed slots so existing children never move.
  - Continuation is recorded on the node (`Node::codominant` weight), never guessed from fan order; every consumer reads the recorded role.
  - One radius allocation by role in both solves: lateral `lateralShare`, codominant sibling `forkBalance` times weight, primary all; wood conserved at every fork; neutral values reproduce the pipe model.
  - Rows: `codominance`, `forkHeight`, `forkHeightSpread`, `forkWays`, `forkDivergence`, `forkLean`, `forkLeanSpread` on `/skeleton/habit`; `lateralShare`, `forkBalance` on `/radii`. The five stems rows are retired and refused by name with their replacement.
  - `dbh_m` reports the largest continuation system crossing 1.3 m and their count.
- No taper-profile row (decided from R7's render); no London plane preset ships here; the node ceiling does not move; the growth path stays hidden.

## Acceptance Criteria

- **R1:** Codominance rows exist with rails, by name, on the wire, blended, in browser metadata and the dial table; rate zero gives no codominant fork on any preset — met.
- **R2:** One rule builds every multi-trunk tree; the stems rows and placement path are gone; retired rows refused with replacement named — met.
- **R3:** The seed decides each fork; share rises with the rate; forks and `forkWays` grow in without moving existing parts — met.
- **R4:** Codominant forks repeat at order one and higher — met.
- **R5:** Wood divides by role (fn-103's R1, R2); neutral rows give today's pipe model; conserved at every fork — met.
- **R6:** A codominant fork at any height is a surface fork; girth eases and contacts follow recorded roles; `dbh_m` by continuation system — met.
- **R7:** Birch re-expressed and shown before/after; an 8-tree London-plane candidate measured and shown to the owner — met (verdict below).
- **R8:** The beech's capability assessment names fn-170 for its codominant V fork — met.

## Result

- Neutrality: every preset other than the birch keeps skeleton positions, mesh, leaves, field and plan digests (7 presets, seeds 1 and 7); only Tree bytes moved (Node gained an `Option<f64>`); specimen snapshot schema 6.
- Birch at seed 1: fork at 0.96 m, leaning stem 28 degrees, bearing kept from the seed.
- Plane candidate (`.flow/evidence/fn-170-codominant-forks-one-rule-from-the/plane-candidate.json`, an oak overlay: 26 m, codominance 0.85, forkHeight 0.2 ± 0.15, ways 3.2, lengthTaper 0.2, lateralShare 0.45, forkBalance 0.9), 8 seeds: structural nodes 11,231-45,531 vs 10,141-11,635 without forks; twig nodes 204,469-238,769 vs 238,365-239,859 (both at the 250,000 ceiling, the forked tree gives up at most 15% of its twigs); median build 769 ms vs 700 ms; peak 619 MB vs 536 MB. Birch as shipped: 2,098-2,530 structural nodes, 2,851 ms, 176 MB.
- Owner verdict (2026-09-27): "birch looks good and the plane looks decent but yes it thins out too quickly for sure" — birch accepted; the plane's forks read but its limbs thin too early, so a taper spec followed (fn-177).
- Codex review SHIP after two fan-out rounds and a re-review.

## Open

Limb girth to the crown edge went to fn-177. The plane is onboarded by its own species spec.

## Friction

- The spec left row names, profile form and draw site to the host; 25 minutes of reading before any code -> name rows and draw rule before `ready`.
- The plan review and nine host decisions landed two hours into the build; four rewrote tested code (~45 minutes) -> run plan review before dispatch.
- A dial's wording reaches Jev request bodies, so a description fix re-recorded the beech tape (8 calls) -> a tape key hashing dial ids and values, not prose.

## Evidence

- `.flow/evidence/fn-170-codominant-forks-one-rule-from-the/plane-candidate.json` (the plane overlay later specs reuse).
- The before/after birch and plane still sheets were kept only in the ignored `raw/` directory.

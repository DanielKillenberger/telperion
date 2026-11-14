## Goal

Open-grown broadleaves read by their branching hierarchy: a few strong scaffold limbs carrying their girth far out, sparse middle orders, dense short twigs at the crown's edge. The owner's London plane photograph (`.flow/references/london-plane/schutzenmattpark-basel-2026-09-27.jpg`) and the beech's references show it; the generator did not (fn-177: every limb born thin and branching everywhere; the beech's runner Tunes kept crowns of similar-sized sticks). After fn-61, fn-170, fn-177 and fn-180 this was the capability the beech still lacked. Measure first, then decide the rows.

## Decisions

- owner (2026-09-27, on fn-177's plane candidate beside the Schützenmattpark photograph): "The structure of those trees is much different. Now we have 1 super thick center and then it tapers off to be thin very quickly anyway. The plane i sent has very thick multiple leaders and thick branches from that. Then there's actually very few branches off that secondary level that lead to many twigs with leaves."
- owner: "Is thie branching hierarchy part of 177? i think it would also fix the beech"
- owner (2026-09-28, on a beech Tune candidate): "why arey ou showing me this abomination?" (a crown that is a ball of long straight twigs with no limb structure)
- owner (2026-09-28): "ok merge it and spec"; built after fn-173, later re-sequenced after fn-183 (host, 2026-10-02).
- Host decision from R1 (2026-09-28): the capability is a twig shell (`twigShell`, `twigShellSoftness`), because the scaffold is already sparse and the twig layer bears shoots along every limb; the plane's thin first-order limbs and the beech's single stem are values (`lateralSpacing`, `lateralsPerStation`; `codominance` near fork height 0.3).
- Host build choices (2026-10-02): depth reads the smooth outline; the fade runs past `twigShell`; neutral is `twigShell` = 1; laterals only.
- Held (host, 2026-10-02, after the owner's verdict and Astra's visual review): the beech still reads as long straight rods — rule-grown, since scaffold laterals take the whole parent length × `lateralLengthRatio` (0.65) with 3° crookedness. A bounded comparison with existing rows came next.
- owner (2026-10-02), on deleting the shell: "y" — `sheddingThreshold` 0.25 clears the inner crown as the shell does and the stills show no difference. The shell's code stays on the unmerged branch `fn-182-branching-hierarchy-few-strong-limbs` (a046d8da, 3d8ac476, 768feab5).
- Paused (owner, 2026-10-02): the two supported code changes (a limb tip's continuation takes the ordinary shoot length; twig bearing stops depending on wood thickness) wait on fn-188. Their evidence is on branch `fn-182-girth` (`R7-GIRTH.md`, `R8-HORNS.md`, `ASTRA-THICKNESS-REVIEW.md`).
- Superseded by fn-190 (owner, 2026-10-03): the hierarchy is the new growth law's to deliver, by axis differentiation and competition rather than a twig shell or girth patches; this spec's evidence carries over.

## Acceptance Criteria

- **R1:** Per-order measurement of the beech, plane candidate and oak against the photographs — met: the plane's order-1 limbs are born at 0.18 of the stem (photograph 0.5 to 0.7); the twig law seeds 21 to 61 laterals along every limb (3.7 to 10.8 per metre) where the photographs show bare inner limbs; fine twigs sit evenly along parents, not at the edge; length ratios differ least; the beech's 32 m excurrent trunk keeps its limbs short (0.34).
- **R2:** Design decision recorded before code — met (twig shell).
- **R3:** New rows dormant and byte-identical at neutral, catalogued, blended, on dials — met on the unmerged branch.
- **R4:** Synthetic family: no twig-layer shoots deeper than the shell, continuous fade — met on the unmerged branch.
- **R5:** Plane and beech candidates shown to the owner beside the photographs — met; verdict: the plane "slightly closer but not there at all", the beech still rods.
- **R6:** Gates green; cost and sizes reported — met on the unmerged branch.
- **R7:** The girth rows walked one at a time on Candidate B — moved to fn-190 (evidence on `fn-182-girth`).
- **R8:** Host decision on thickness, including Astra's two verified limits (forks placed by absolute tree height, `scaffold/fork.rs:87`; laterals take a fixed share of the parent's whole length, `scaffold.rs:317`) — moved to fn-190.

## Result

The shell was dropped. Candidate B's direction went to fn-62 (values in `R5-RESULT.md`, "Bounded comparison"): `lateralLengthRatio` 0.3 from 0.65 ends the long rods (side branches 0.82 of the parent's remaining extent against 1.15; 6 of 293 longer than half their parent against 171 of 260), with `crookedness` 10, `pitchVariation` 16, `shortShootRadius` 0.3, `shortShootSpacing` 0.08 and shedding for the inner crown, on Hierarchy 1 (`codominance` 0.3, `forkHeight` 0.35, `forkWays` 4). It is sparse, low and wide against the reference. Branch thickness stays unsolved: at substantial divisions daughters stay about 0.78 of their parent across every row tested.

## Open

Thickness hierarchy and the two code changes are fn-190's.

## Friction

- No checklist of the sites a new row must touch; four found only by the gate -> proposal: an "adding a row" checklist in `docs/parameters.md` or a catalogue test naming each site.
- `growth_profile --features query-count` takes a preset, not a family -> proposal: `--family FILE`.
- The dcg hook blocks redirects to variable paths -> local setup, reported.

## Evidence

- `.flow/evidence/fn-182-branching-hierarchy-few-strong-limbs/R5-RESULT.md` (the shell's build, per-order results and the bounded comparison fn-62 starts from).

## Conversation Evidence

> owner (2026-09-27, fn-170 session, Basel Schützenmattpark photograph): "I feel our generator fundamentally lacks the ability to have strong limbs that quickly taper at the edge of the crown to leaf level."
> owner (2026-09-27, on fn-170's R7 London plane candidate): "birch looks good and the plane looks decent but yes it thins out too quickly for sure"

## Goal & Context
<!-- scope: business -->

A London plane's limbs, and a beech's, stay thick almost to the edge of the crown and then break into fine wood over a short distance. The generator's limbs thin steadily from the fork, so by mid-crown they have already become fine wood. fn-170 made the plane's codominant forks possible; on its 8-seed candidate (oak base, 26 m, `lengthTaper` 0.2, `lateralShare` 0.45) the owner confirmed the forks read and the limbs thin too early. This spec lets a table hold a limb's girth over most of its reach and taper it to leaf level near the crown's edge. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-27 on the fn-170 branch (PR #131), merged to master as `ed23f773`.** The radius solve (`crates/telperion-core/src/pipeline/radius.rs:121`) thins wood two ways: bottom up, each node's pipe is its children's pipes combined through `forkExponent`, so every lateral that leaves takes its share (scaled by fn-170's `lateralShare` and `forkBalance`); and `lengthTaper` adds an exponential thinning over the path length from the root. There is no row that makes a limb's girth depend on how far along its own reach a point is. [checked]

**Measured, 2026-09-27 (R1, `.flow/evidence/fn-177-limbs-hold-their-girth-to-the-crowns/R1-MEASUREMENT.md`).** On the plane candidate the tip count thins the limbs: at mid-path the thickest limbs keep 14% of their base girth, the pipe model accounts for 96% of that thinning (in log terms) and `lengthTaper` for 4%. No existing row gives a held girth that then falls: `lateralShare` at its floor holds the girth, but the limb ends blunt at 30% of its base and the tree regrows. The earlier claim that the profile acts "per limb system (fn-61's limb bound)" was unchecked and is wrong for this tree: on 7 of 8 seeds the thick wood is codominant stems, which fn-61's bound does not cover, and a path leaves the stems at 43% of its length with 25% of its girth. [checked]

**The rule (host, 2026-09-27).** One rule for all structural wood (stems, codominant siblings and limbs alike), with no separate case per kind. The root is a point, not wood: every part leaving it starts an axis, as do a lateral and a codominant sibling, and a fork's primary above the root carries its axis on. Each axis holds its own base girth, which is the pipe model's radius where the axis starts (its first node's start radius). A node's share of its axis's reach is t = d / (d + L): d is its path from where the axis leaves its parent, so every node of an axis lies past its start and a hold rising from zero moves no radius by a jump, and L is its path on along the axis's own continuation to that axis's tip. L does not run through the laterals, so a leader that ends in a whorl is not left blunt. A node's radius is the larger of the pipe model's radius and the held girth. The held girth is the base up to `girthHold`, then falls to the pipe radius by the tip, as a smoothstep in log radius (C1 at the hold point and where the fall ends). The fall lasts `girthHold / girthFall` of the reach and is cut at the tip. [checked]

**Rows.** Two rows in `/radii`. `girthHold` is the share of the reach, from 0 to 0.9: at 0 the tree is today's to the bit, and the ceiling leaves every axis at least a tenth of its reach to fall, so no tip ends blunt. `girthFall` is from 0.5 to 8, default 2: at 0.5 the fall is twice the hold, close to the pipe model's steady thinning, and at 8 it is an eighth of the hold, a break over a few stations of a long limb. Both rows are declared once in the catalogue, blended linearly and on the dials, and the hidden growth path ignores both. [checked]

**What is given up.** Where the held girth wins, a fork's parts carry more wood than their parent, so the pipe model's conservation at forks is given up over the hold. A solved tree's invariants hold: no node starts thinner than it ends, and a part's start is clamped to its parent's radius at the junction. The hold is applied once the twigs have grown, in the direct build's final radius pass, so the twig layer seeds on the pipe model's radii. The tree keeps the pipe radii of the wood it held (`Tree::pipe`), and every foliage decision about which wood bears leaves, and which child carries a run, reads them. As a result no twig or leaf borne is added or lost. A leaf still sits on the wood as drawn, so the envelope's interior cull, which reads where each leaf is, may keep a handful more or fewer: 3 of 83,000 on Ordinary at seed 1. [checked]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The measurement is reported: radius along the thickest limbs of the plane candidate at 8 seeds, split into the tip-count and length-taper parts. [checked]
- **R2:** `girthHold` and `girthFall` exist in `/radii` with the rails above. They are validated by name, on the wire, blended, in the browser metadata and on the dial table. At zero every shipped preset's skeleton, mesh, leaves and field match master (seeds 1 and 7). [checked]
- **R3:** On a synthetic limb, the girth stays within 1e-9 of its base over the hold and is the pipe radius from the end of the fall to the tip. A leader ending in a long lateral falls to the pipe radius at its tip. Walking either row in small steps moves every radius by a small step, from zero hold onwards, and the junction invariants hold throughout. [checked]
- **R4:** The plane candidate is rendered at the 8 seeds with the rows set, bare and in leaf, and the stills go to the owner with one question: do the limbs hold their girth to the crown's edge now? [inferred]
- **R5:** The workspace gate and `npm test` are green; build time and peak memory on the candidate are reported against fn-170's numbers. [inferred]

## Boundaries
<!-- scope: business -->

- Not a London plane preset: the plane is onboarded by its own species spec. Not the fork rule (fn-170). No change to how many twigs or leaves a tree bears.

## Strategy Alignment

- Serves "Growth and botanical fidelity": a limb that holds its girth to the crown's edge is the look of the open-grown broadleaves the catalogue needs. [strategy:Growth and botanical fidelity]

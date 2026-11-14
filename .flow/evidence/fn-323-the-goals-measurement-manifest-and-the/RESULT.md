# fn-323 result: the route stops

**Verdict, by the frozen rule in `MANIFEST.md`: stop.** No candidate passes the field limit at both ages, so the goal's Route stop applies and child 2 does not start. Order 5 passes at age 40 and fails at 80.

The result does not hang on the limit, which is a judgment, because both of the stop's tests fail. Order 5, the only candidate to pass anywhere, keeps 85% of the tree's nodes at age 40 and costs about 9.7 s of scaffold at age 80, against 50 ms. A cut cheap enough to matter loses the tree's shape, and a cut that keeps the shape costs seconds.

Measured 1 Nov 2026 at `0c731ed53`, RTX 3080, native, `--profile ci`. Seeds 1, 7, 42, 1234 and 5678; medians. Raw logs are in `raw/`.

## R4: what the field needs

The Gaussian density, the better of the two at nearly every row. Limits: wood and foliage at least 0.85, leaves at most 0.25, total within 1%, limbs at least 0.85. Every total was 1.000.

| Age | Keep order | Nodes kept | Wood | Foliage | Leaves | Limbs |
| --- | --- | --- | --- | --- | --- | --- |
| 40 | 1 | 2% | 0.562 | 0.474 | 0.969 | 0.167 |
| 40 | 2 | 7% | 0.635 | 0.468 | 0.643 | 0.296 |
| 40 | 3 | 26% | 0.761 | 0.609 | 0.348 | 0.485 |
| 40 | 4 | 57% | 0.879 | 0.766 | 0.149 | 0.701 |
| 40 | 5 | 85% | 0.951 | 0.890 | 0.051 | 0.877 (passes) |
| 80 | 1 | under 1% | 0.615 | 0.572 | 0.974 | 0.178 |
| 80 | 2 | 4% | 0.628 | 0.518 | 0.614 | 0.299 |
| 80 | 3 | 15% | 0.744 | 0.603 | 0.425 | 0.431 |
| 80 | 4 | 38% | 0.834 | 0.694 | 0.259 | 0.588 |
| 80 | 5 | 64% | 0.898 | **0.799** | 0.140 | **0.717** |

Node shares use the median tree: about 321,000 nodes at age 40 and 1.6 million at age 80.

**What this cannot show.** It replays density on a tree grown whole, and each subtree's centroid and spread come from the full tree. So this is an upper bound on a closure. It does not test coupling: the goal's evidence against the route, that light and shedding depend on the twig density a closure replaces, is still untested, and would only make this worse.

**A weakness in the candidates.** Foliage and limbs are the two that fail most. The full field flags foliage wherever a leaf's reach touches a cell, and a deposit flags it only at 0.5 leaves or more, so foliage is partly a flag rule, not leaf placement. The probe below shows that: leaf error is small while foliage stays low. A better flag rule could lift those two, but it would not change the cost below.

## R5: the floor's cost

One thread, per the protocol. Ages are for the oak.

| | Age 40 | Age 80 |
| --- | --- | --- |
| Full cycle, range over seeds | 1.13 to 2.02 s | 11.0 to 28.7 s |
| Its light | 0.07 to 0.10 s | 0.60 to 1.21 s |
| Cost per living apex-cycle | 3,037 to 3,227 ns | 3,649 to 4,119 ns |
| Shed share (phytomers grown and not kept) | 0.60 to 0.69 | 0.80 to 0.82 |

| Keep order | Scaffold at 40 (model) | Light at 40 (replay) | Scaffold at 80 (model) | Light at 80 (replay) |
| --- | --- | --- | --- | --- |
| 1 | 4 ms | 21 ms | 10 ms | 161 ms |
| 2 | 47 ms | 51 ms | 139 ms | 267 ms |
| 3 | 268 ms | 130 ms | 1,026 ms | 526 ms |
| 4 | 699 ms | 225 ms | 4,133 ms | 892 ms |
| 5 | 1,195 ms | 257 ms | 9,680 ms | 1,132 ms |

- **The scaffold is a model.** It assumes cost per apex holds across orders, which is unmeasured. No scaffold has been grown alone.
- **The light replay probably overstates.** At order 5 and age 80 it reads 1,132 ms as a median over seeds, above the whole cycle's measured light (median 0.87 s, range 0.60 to 1.21 s), though it reads a cut tree. It deposits every phytomer the kept axes ever grew, and the cycle appears to deposit fewer (inferred from the code, not measured). So treat it as a ceiling. The true cost of light on a coarse tree is **unknown**, and the verdict does not rest on it.
- **Where cost goes:** order 4 and above hold 93% of the apex-cycles at age 80 on seed 1. No browser or Wasm cost was measured.

Against the route's 50 ms, only order 1 is near it, at 10 ms of scaffold at age 80 before light, and order 1 has foliage 0.57 and limbs 0.18. Order 2 is already 139 ms.

Axes under 3 phytomers are counted apart in the `LIVE` lines: about half of the living apexes at order 5 and deeper, and 16% at order 0. They stay in the costs, as the manifest says.

## R3: limb count and shed share

`pipeline::measures::limb_count` and `shed_share`, each with tests on a tree built for it, and each refusing a tree it cannot read. At age 80 the oak has 707 to 844 limbs at the preset's order 2.

## R6: the full cycle's reach

| Executor | Age | Result |
| --- | --- | --- |
| Device, oak seed 1, grid 36 | 80 | 1.27 s |
| Device | 90 | 1.71 s |
| Device | 95 | refused: the cycle state pencils buffer needs 2,563,723,008 bytes, above the device's limit of 2,147,483,644 |
| Device | 100 | refused: the axes buffer needs 2,558,845,952 bytes |
| Device | 120 | refused: the axes buffer needs 5,067,452,416 bytes |
| CPU | 80 | 11.0 to 28.7 s on one thread |
| CPU | 120, 160, 200, 300 | refused: the tree outgrew its budget of 20,000,000 phytomers |

The CPU's limit is the request's phytomer budget, a setting, not the machine's. The ages between 80 and 120 were not tried. Seed 1234 already grows 17.1 million phytomers at age 80, so the budget is close there. So age 300 has no full-cycle reference on this machine at this budget, and R4 and R7 cannot be compared there, as the goal's Decision Context expected.

## After the verdict: a cut by subtree size

For the owner's options only, and no candidate of the manifest's. Nodes whose subtree holds fewer than N nodes stand as density (`subtree-40.log`, `subtree-80.log`; seed 1):

| Age | Under | Nodes kept | Wood | Foliage | Leaves | Limbs |
| --- | --- | --- | --- | --- | --- | --- |
| 40 | 10 | 42% | 0.866 | 0.446 | 0.052 | 0.439 |
| 40 | 100 | 10% | 0.636 | 0.308 | 0.314 | 0.293 |
| 80 | 10 | 44% | 0.918 | 0.546 | 0.031 | 0.539 |
| 80 | 100 | 12% | 0.755 | 0.449 | 0.180 | 0.432 |

Cutting only tiny subtrees places leaves well, at 0.03 to 0.05, so density can stand in for them. But it saves little: subtrees under 10 nodes still leave 42-44% of the nodes.

## Corrections while measuring

All numbers above are from the last rerun, and each fix implements the manifest and changes none of it.

- **Nearest cell:** the first runs sent a subtree reaching no cell centre to the wrong neighbouring cell half the time. An implausible probe result exposed it: leaf error 1.15 for twigs that should land where they grew. Fixed, with a test.
- **Limb owner (codex):** deposited leaves could not take a cell's limb from kept leaves. This one moved the result: order 5 at age 40 went from failing at limbs 0.842 to passing at 0.877. Fixed, with a test.
- **Protocol (codex):** the light's median held its warm-up, and short axes were not counted apart. Both fixed.
- **Reach** was not rerun, since none of these touch it.

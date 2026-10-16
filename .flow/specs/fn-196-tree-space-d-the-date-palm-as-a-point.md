# Tree space D: the date palm as a point

## Goal

Phase D of the tree-space programme (`docs/tree-space.md`; owner, 2026-10-03): the date palm (Corner's model: one unbranched stem; offshoots out of scope, the catalogue rules out a suckering clump) as a point in the tree space, values only from fn-190's MODEL-PALM.md. Today's palm is the catalogue's most realistic tree and is the bar (owner, 2026-10-03). Wrong-path stop as phase C, or the palm cannot match today's.

## Decisions

- Values in `crates/telperion-space/src/palm.rs` (SOURCES.md): one physiological age, readiness 0, 12 phytomers a year at 0.031 m, `secondary` 0, `pipe` 0.297 m (today's trunk radius); 18.6 m to the apex at 50 years, one width foot to crown. Dressed by the date-palm preset unchanged (fronds, skirt, 256-base lattice, acanthophylls, flare) through the conversion and `executor::expand`.
- Two design blockers built first as their own specs (host, 2026-10-05): fn-204 moves leaf-base clothing to the start of expansion (D1); fn-205 adds `form.secondary` for a columnar stem (D2).
- Host decisions on round 1 (2026-10-05): (1) the palm is judged at maturity; young palms wearing the adult crown and all 256 bases are a known gap for fn-198's station contract; (2) the space examples key the preset's seed to the engine seed for every species; (3) the reference's 15° lean is a known gap, judged against today's palm; (4) fn-204's P2 fixed: a tree already carrying exactly its bases is not clothed again (all eight preset digests unchanged).
- `still.rs` frames on the mesh bounds, adds a crown close-up and `--today` for the preset's own build.
- R3 split to task 2, blocked by fn-206: the oak has 10 physiological ages and the palm one, so there is no per-setting midpoint until both sit on one reference axis.

## Acceptance Criteria

- **R1:** The palm's values from MODEL-PALM.md, branching readiness zero; its organs reach expansion on the new stem — met.
- **R2:** The gate as in phase C, no visible regression against today's palm, framed on the whole tree — met (round 1b).
- **R3:** An oak-to-palm walk as still strips, no visible pop — met (task 2, on fn-206's chain).
- **R4:** The passed beech, spruce and oak sheets unchanged in look — met for beech and spruce (only lighting from fn-194's staging and leaf placement from the seed keying differ).

## Result

Gate (host, 2026-10-05): confident it reads as a date palm with no visible regression; Astra PASS in 3 of 3 samples (seeds 1, 2, 3, 4 yes; 7 borderline). The one visible change is the stem, a straight column against today's slightly kinked one; 25.3 to 25.7 m to the frond tips against today's 25.9 to 27.2 m. Both name one shared fault: fronds stiff and straight where the references' are soft and arching (a dressing gap, not a regression). R3 (seed 1, age 60, nine stills): the oak's crown goes early and steeply (54% of scaled length left at t = 0.01), accepted as inherent (fn-206 host decision 15); the stem thickens into the palm's column from t = 0.5; heights 17.6 to 22.6 m; no pop. Codex SHIP after one P2 fix. References now in `.flow/references/date-palm/`.

## Open

- Young palms: frond size and count and leaf-base count do not follow the stem's age (fn-198's station contract).
- The reference's lean and top curve, the drooping skirt, the hanging date clusters (fn-111), bases weathering smooth toward the foot, the dark fibrous trunk colour.
- Stiff fronds.

## Friction

- No `.flow/references/date-palm/` folder -> created in this spec.
- Leaf bases unreachable from an engine tree -> fn-204.
- No species-to-species walk and mismatched settings -> fn-206.
- GPU out of memory on 80-year spruces and a memory-ceiling test that moves with other workers' memory -> proposed: a ceiling independent of other processes, GPU memory budgeting in the still runner.

## Evidence

- `.flow/evidence/fn-196-tree-space-d-the-date-palm-as-a-point/SOURCES.md` (values and sources; cited by `palm.rs`)

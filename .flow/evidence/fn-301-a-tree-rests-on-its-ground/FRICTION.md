# fn-301 friction

## 2026-10-21 00:00, conductor (flow --auto), selection

- **Doing:** the pilot snapshot for fn-301 on its branch.
- **Slowed by:** the snapshot marks fn-301 ineligible: "parent branch fn-283-a-frond-draws-its-rachis not on origin". The branch already carries fn-283 (merged at d600fa12c), so the chain predicate's premise does not hold here; the run went ahead on the host's instruction.
- **Cost:** about 2 minutes.
- **Would remove it:** a chain predicate that accepts a parent already merged into the child's branch head.

## 2026-10-21 00:05, conductor, the unknowns

- **Doing:** checking the spec's "Unknown" section against the code.
- **Slowed by:** the spec names the dead skirt as fn-204's leaf bases; it is the rosette's dead fronds (`skirtFronds`, `skirtPitch` 165), drawn as fronds with `dead = true`. `docs/pipeline.md` still says four families have no engine species; all eight have `/space` rows.
- **Cost:** one exploration pass, about 5 minutes.
- **Would remove it:** the spec and the doc corrected (reported to the host).

## 2026-10-21 00:20, conductor, the R1 repro on the base

- **Doing:** the R1 probe (`tests/ground_reach.rs`, 168 builds) through `telperion-heavy 8`.
- **Slowed by:** the first run counted the trunk's ring at the root, which leans with the trunk, as wood below the ground on every preset; a second run left it out. Each run waited 1.5 to 3 minutes for admission and took 2.5 minutes.
- **Cost:** about 12 minutes, one rerun.
- **Would remove it:** a wood bound per run that names the trunk's foot, so a ground probe need not rebuild that rule.

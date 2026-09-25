# The tuning loop

One tuning revision is `tuning::command::run_with`, which the species
runner's Tune stage calls in process (`docs/species-runner.md`). It belongs
to `telperion-jev`, never the generator, renderer or shipped presets. Jev
selects directions; Rust computes every number. It never ships a species.

The typed config is `tuning::live::Config`: the preset and seed, the starting
overlay, the dial rows, owner notes, the measurement binary and profile, the
matched camera references, the reviewer adapter (`vision`, the
reference-first reviewer, `scripts/reference-first.py`) with its reference
inventory (`reference_first`), the contact-sheet adapter (`sheet`,
`scripts/contact-sheet.py`), references, quality anchors, the required
view/seed cells (fixed and fresh seeds), bundle strengths and tracks, the Jev
model, and the opening spend. Every image's bytes are hash-checked.
`Config::verify` checks that the config is whole; it asks for no calibration
and no authority.

A revision runs into its own directory and never twice into the same one:

1. The baseline is evaluated and measured; numeric failure stops before any
   render.
2. The reference-first reviewer assesses the current tree against the
   references (one repair call when its answer breaks a tidiness rule, then
   code trims it; fn-80).
3. A checkpoint turns the reviewer's findings and the inventory's traits into
   the revision's objectives, taken as they stand.
4. Each round asks Jev for a direction per dial (in batches when the table is
   large), draws one bundle of every supported dial at each configured
   strength per track, and buys one contact sheet to judge them. A better
   variant that breaks something is halved until the breaking dials are
   isolated; a worse bundle is cut by family. The kept variant takes a closing
   look, and the veto can take it back.
5. The revision ends when the reviewer passes every required cell (an owner
   look under bootstrap), when a round has nothing new to draw, or when
   `runaway_rounds` rounds in a row (default five) keep nothing: the
   no-progress stop.

Spend is counted in `run.json` and never capped. Judgments are acted on at
the confidence the labelled sets set (`data/thresholds.json`,
`tuning_min_confidence`; the gap-magnitude cut in its own labelled set).

Every save writes `run.json` and `result.json`: the outcome (why it stopped,
adoptions kept and rolled back, spend, the current tree with its overlay and
stills) and one gap entry per objective, whatever became of it, with the
attempts the reviewer graded on it and the reviewer's words. The runner's
Gaps stage classes them; the file mints nothing and is never readiness.

A reference inventory for a new species is built with `tuning::inventory::run`.

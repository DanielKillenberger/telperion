# Materials bundle run on the beech, 2026-09-21

Production `tuning-loop` at `248165c5` (CI-profile gate EXIT=0, 808 tests). Bootstrap run from bundle 1's tree, 91 material dials only, one track judged on the close trunk view `B-BASE` (one extra render per variant), strengths 0.5/1/2/4, contact sheet v2. Jev calibration re-qualified for this table (7/8, 4/5, 4/4). Owner's five priorities as approved; only `owner-materials` routed to tuning, which is correct for a material-only table. Raw state in ignored scratch `.flow/tmp/fn68-canonical/materials/`. The host stopped the run by hand after six rounds (it was buying a losing sheet per round); the interrupted reservation is "extra view capture".

Baseline trunk, reviewer: "The fine rectangular surface pattern looks artificial, and broad dark horizontal scars are too conspicuous."

Six rounds, 13 to 21 dials per bundle, 24 variants rendered, every one ranked below the current tree and graded worse; at four steps the bark goes nearly black. Reviewer's words across rounds: "continuous vertical wrinkles… corrugated rather than smooth", "dark olive-brown bark with heavy glossy folds", "large raised, angular bark plates", "lichen appears as simplified blue-grey spots".

Why, from the recorded moves: every bundle lowered all three bark colour channels (darker), lowered roughness (glossier), raised `ridge_scale` from 0.002 to 0.012 in one half step, and raised `plate_cell_scale` and `plate_identity` from zero, which switches bark plates on; the sensible moves in the same bundle (grain strength down, lichen coverage down) could not be told apart from them. The same directions repeated in all six rounds despite the reviewer's notes. All 91 rows fall in one dial family, so the newer split-on-worse would not have been able to cut these bundles either.

Reading. Not yet evidence of a material gap: the search never isolated a single sensible family. It is evidence that (a) material meanings such as "the green in the bark's own colour" give Jev nothing to reason with against "smooth gray bark", (b) dials that sit at zero and switch a feature on need to say so, and (c) some authored steps are far too large relative to the current value (`ridge_scale`).

Reviewer spend: 8 calls (two reviews, six sheets). Totals after the run: visual passes 69, evaluations 110, images 576, rounds 34.

# Reviewer-judged selection run on the beech, 2026-09-21

Production `tuning-loop` at `132bc06e` (CI-profile gate EXIT=0, 776 tests). Fresh bootstrap run, 109 score-visible dials, three candidates per round, `selection: visual`: an uncalibrated side-by-side progress review (requested `gpt-6-astra` medium through `scripts/progress-review-codex.py`) compares the current tree with each candidate at the same view and seed, renders labelled A and B by code, the candidate's side hidden. Owner confirmed the same three priorities against this run's packet ("confirm"). Raw state in ignored scratch `.flow/tmp/fn68-canonical/visual/`.

The first round aborted for free on an inert candidate's byte-identical still (fixed in `132bc06e`: code settles an inert move without a reviewer). Resumed with evidence and approval preserved; four rounds then ran without host intervention.

| Trial | Move (direction mass) | Reviewer, crown / hanging | Reviewer's regression note | Score (telemetry) | Outcome |
| --- | --- | --- | --- | ---: | --- |
| twig_hang small and substantial, twig_max_droop, twig_curtain_drop, twig_sag, twig_pendulous_length | 0.86–0.97 | — | stills byte-identical | 0.2432 | inert, settled by code, no reviewer call |
| irregularity small_increase (0.98) | worse / same | candidate fills the shoulders into a more continuous envelope | 0.2512 | refused |
| rise_secondary small_decrease (0.97) | **better / better** | the note faults the CURRENT tree, not the candidate | 0.2780 | refused by the blunt rule "any regression blocks" — a rule defect |
| lateral_pitch small_increase (0.85) | better / better | candidate exposes long bare crossing branches in the upper crown | 0.3236 | refused |
| shoulder small_decrease (0.82) | better / same | candidate exposes a long straight sparsely clothed lower-left limb | 0.2035 | refused |
| **pitch_variation small_increase (0.78), to 11** | **better / same** | none | 0.2644 | **adopted**; closing all-view review ran |
| irregularity substantial_increase (0.99), from the adopted tree | worse / same | candidate smooths the upper-right shoulder | 0.2674 | refused |
| rise_secondary substantial_decrease (0.96), from the adopted tree | better / better | one note faults the current tree, one says the candidate exposes long bare branches across gaps | 0.3054 | refused |

First live adoption, and the first live adopt-and-reassess lap (R10). The closing review still fails every cell: densely filled upper shoulders and a regular envelope, no spreading shelves or hanging outer masses, repeated straight ascending sprays, bark with horizontal markings and rectilinear texture. The run then paused on the byte-sized round reservation (452,516); nothing was spent on it.

Findings.
1. The score would have rejected the adopted move (0.2644 against 0.2432) and would have adopted `shoulder` (0.2035), which the reviewer says leaves a bare straight limb. The two judges disagree in both directions.
2. All six pendulous-twig dials are inert one at a time on this beech. Hanging foliage cannot be reached by single-dial moves; either the rows must move together or the beech's pendant path is off for a reason the table does not expose. Unknown which.
3. Rule defect: regressions are free text naming A or B, and the rule blocks on any regression, including ones that fault the current tree. `rise_secondary small_decrease` was better on both priorities with no candidate-side regression and was refused. Regressions need a side, and only candidate-side ones should block.
4. Every reviewer comment repeats the same missing thing: foliage organised around ascending branches, no spreading shelves, no leaf-weight droop.

Reviewer spend for this run: 12 calls, 333,755 tokens (three all-view reviews and nine side-by-side reviews; one earlier review was lost to the stray coverage row). Visual passes 44 of 60, evaluations 30 of 40, images 158 of 260, rounds 12 of 13.

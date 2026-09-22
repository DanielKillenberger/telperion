# Wide-table bootstrap run on the beech, 2026-09-21

Production `tuning-loop` at `e1ef8b0a` (CI-profile gate EXIT=0, 763 tests). Fresh run, visual bootstrap, round-22 rows, seed 1, 109 score-visible dials from `crates/telperion-jev/data/dials.json` (structural table sha256 `ed9fad82…`), proposals asked in three batches of at most 40, `max_candidates: 3`. Calibration re-qualified against the new table hash before the run: magnitude 7/8 with zero unsafe, direction 4/5, composed continuation policy 4/4, 16,003 Jev tokens. Raw state in ignored scratch `.flow/tmp/fn68-canonical/wide/`.

## Sequence

1. Authority, preparation charged once, baseline 0.24324871 (the fourth identical measurement today).
2. Initial review paid but refused by the binding for one stray coverage row naming a required cell; the pause was unrecoverable. Fixed in `e1ef8b0a`; scoped recovery kept the pass and the 40,000-token reservation charged. The repeat review emitted the same stray row, which was dropped into observations, and bound.
3. Priority pause. The host proposed the same three priorities against the new packet; owner: "ok go". Approval resumed with evidence preserved.
4. All-view review, seeds 1 and 42: all 14 cells fail. Defects: enclosed upper crown without branch-scale separation; no weighted hanging outer foliage; repeated straight ascending branch fans; bark with horizontal dark bands and embossed texture.
5. Routing: finding-0 tuning, finding-1 tuning, owner-materials appearance 0.77, risk bounded, handoff authorized.
6. Proposals: three Jev calls over 109 dials; 26 dials reached direction mass 0.5. The strongest: twig_hang up 0.99, rise_secondary down 0.98, irregularity up 0.98, twig_sag up 0.95, twig_max_droop up 0.94, twig_curtain_drop up 0.94, twig_pendulous_length up 0.94, shell_depth down 0.88, leaf_upward down 0.83, twig_reach up 0.81, limb_clumping up 0.81. These are the dials a person would reach for given "hanging outer foliage" and "separated foliage masses".
7. Three candidates measured:

| Candidate | Score | Note |
| --- | ---: | --- |
| twig_hang substantial_increase, hang 1.0 | 0.24324871 | stills byte-identical to the baseline, node count unchanged: the dial is inert on this beech by itself |
| irregularity substantial_increase | 0.26039929 | worse; same value as the earlier pilot, the measurement is deterministic |
| rise_secondary substantial_decrease, −0.5 | 0.28759647 | worse: occupied share 0.53 to 0.61 against a 0.46 target, outline deviation 0.17 to 0.09 against 0.23 |

8. No improvement, numeric stall. The next round's byte-sized reservation (430,204) did not fit and nothing was spent. The host stopped as it had said it would.

## Reading

Jev's part works: given a focused state and the whole table it selects coherent, relevant moves cheaply. Two things outside Jev stop progress. First, selection is by the five still-side numbers alone, and every move aimed at the owner's and the reviewer's complaints has made those numbers worse or left them unchanged; no such move is ever shown to the reviewer, so nobody learns whether it looks better. Second, single-dial moves cannot switch on behaviour that needs several parameters together: `hang` at 1.0 changed nothing, which suggests the beech's pendulous machinery is off until another row (pendulous length, sag or droop) is nonzero. Both are unknown until tested.

## Accounting

Tokens 1,150,908 of 1,500,000; visual passes 34 of 38; images 86 of 150; evaluations 16 of 26; rounds 7 of 10. This run: 224,430 tokens including the lost review (26,788) and its stranded 40,000 reservation, 4 visual passes, 5 evaluations, 28 images.

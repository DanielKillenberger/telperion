# Bootstrap beech pilot, 2026-09-21 (fn-68 R7/R10 live, fn-62 pilot round)

Production `tuning-loop` at `2fc48aac` (CI-profile gate EXIT=0, 104 suites). Visual bootstrap mode, round-22 beech rows, seed 1, `max_candidates: 1`. Reviewer: requested `gpt-6-astra` medium through `scripts/reference-first-codex.py`; Jev for text judgments. Raw state stays in ignored scratch `.flow/tmp/fn68-canonical/pilot/`. No retry of any judgment.

## Sequence

1. Authority pause, then scoped bootstrap authority. Preparation (beech Stage A) charged once: 759,511 to 780,159 tokens, 28 to 29 passes.
2. Baseline: gates pass, score 0.24324871. Initial whole-view review: two blockers (continuous rounded crown envelope with solid shoulders; sprays sweep upward with little hanging foliage), branching unknown behind foliage, finish meets the catalogue floor.
3. Priority pause. Owner reply to the packet summary: "confirm". Order: finding-0 crown shape and foliage organization; finding-1 hanging outer foliage; owner-materials.
4. Host error: the approval resume omitted `preserve_evidence`, so the identical baseline was re-bought (1 evaluation, 4 images). All-view review, seeds 1 and 42, 14 cells, all fail. New defects: repeated straight ascending sprays and crowded parallel axes in the bare view; bark with rectangular relief and strong horizontal bands against a smooth, finely marked reference.
5. Round preflight refused: 222,188 tokens reserved by byte count against 71,381 remaining. Caps raised under the owner's standing rule (tokens 917,431 to 1,080,000; images 54 to 58) through a scoped resume with evidence preserved; the image extension needed `2fc48aac`.
6. Routing, one Jev call, 9,801/177 tokens: finding-0 tuning 0.84; finding-1 tuning 0.75; owner-materials appearance 0.78. Pre-dispatch risk for the materials handoff: bounded 0.67, so the handoff is dispatch-authorized. Round boundary: code proceeded without a judgment (first attempt on this candidate).
7. Proposals, one Jev call, 11,232/464 tokens, threshold 0.5: no dial cleared it. irregularity chose small_increase at 0.39 with probability 0.50 small_increase and 0.44 substantial_increase; crookedness, leaves, limbs, spacing and taper answered insufficient_evidence (0.37, 0.54, 0.54, 0.18, 0.59). The run paused: "no supported proposal; bounded diagnosis required". No candidate was evaluated. The round is spent (3 of 3).

## Result against the handover's finish lines

Finish line 2 with a live routing result: one grounded, authorized handoff (materials, appearance) and two priorities routed to tuning for which Jev then supported no adjustment. The tuning pilot executed up to proposals and produced no candidate, so magnitude efficacy remains unvalidated (R7's error clause). One live lap reached a resumable pause through real models (R10's permitted ending); no tune-and-reassess step has ever run live.

Observation, not yet a change: on `irregularity` the direction was clear (0.94 of the probability on an increase) and only the magnitude was split, yet the single argmax confidence of 0.39 fell under the threshold. The acceptance rule conflates direction confidence with magnitude confidence. The frozen calibration ledgers store probabilities, so a direction-mass rule can be re-scored offline at no cost before anyone changes the rule.

Defect seen: `judgment_inputs[*].ledger` stayed null for the pre-dispatch risk call and for a proposal call that returns no proposals.

## Accounting after the pilot

Tokens 876,128 of 1,080,000. Visual passes 31 of 32. Image reservations 46 of 58. Evaluations 8 of 13. Rounds 3 of 3. This pilot alone: 876,128 − 780,159 = 95,969 tokens after preparation, 2 visual passes, 16 images, 2 evaluations.

## Free offline re-read of the frozen magnitude calibration (host, no model call)

Summing probability by direction over the ten frozen magnitude ledgers: every correctly answered case keeps its answer under a rule "direction mass at or above 0.5, take the small step when magnitude is split", and `lower-bound` (expected insufficient_evidence, increase mass 0.44) stays refused. The set contains no case with a clear direction and a split magnitude, so it neither supports nor contradicts that rule. Its cases state the wanted action in one sentence ("Increase by the small authored adjustment, to three limbs") and drew confidences of 0.78 to 1.0; the live proposal input was a 25 KB run summary with reviewer prose and drew 0.18 to 0.59. The frozen set does not represent the live input.

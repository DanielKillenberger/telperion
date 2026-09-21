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

## Second live round, same day (binary at `8eb33922`, gate EXIT=0, 104 suites)

Changes since the first round, all owner-approved: direction-mass acceptance (`direction-mass-v1`, threshold unchanged at 0.5), a focused proposal state, ledger links kept on every judgment, and evidence kept along a chain of cap-only resumes. Two scoped resumes preserved the baseline, both reviews and the approval: round cap 3 to 4 with image cap 58 to 62, then token cap 1,080,000 to 1,150,000 because the byte-sized round reservation (208,954) exceeded the 203,872 remaining; the refused preflight spent nothing.

1. Routing repeated the first round's answer: finding-0 tuning 0.83, finding-1 tuning 0.79, owner-materials appearance 0.74 (10,269/177 tokens). Pre-dispatch risk bounded 0.69 (8,750/46). The materials handoff was written a second time because handoffs are keyed to the exact run identity, which the cap raises changed; `handoffs.json` now holds two copies.
2. Round boundary: code proceeded, no judgment bought.
3. Proposals on the focused state: 5,306 bytes against 25,286 before, 3,876/456 tokens against 11,232/464. Direction mass per dial: irregularity up 0.97 (substantial_increase 0.57 alone), crookedness up 0.85, spacing up 0.68, taper down 0.62, limbs up 0.47 (refused), leaves none. With the first round's input five of six dials had answered insufficient_evidence.
4. One candidate, `max_candidates: 1`, highest mass first: irregularity substantial_increase, envelope irregularity to 0.34. Gates pass. Score 0.26039929 against the baseline's 0.24324871, so code rejected it and the shipped rows are untouched. B-WHOLE outline deviation moved away from the photograph (0.1698 to 0.1431, target 0.2274) and occupied share rose (0.5321 to 0.5464, target 0.4566).
5. Numeric stall recorded, round limit reached, run paused. No closing review ran because no candidate was adopted; visual passes stay at 31.

## What this establishes

R7: a live direction-and-magnitude round ran end to end on the beech: Jev chose a named action, code computed the value, measured it and rejected it on the score. One evaluated candidate, zero improvements. Against the recorded baselines (0.154 in 13 evaluations, 0.145 in 74) this pilot reached no improvement in 1 candidate evaluation; it is not a controlled comparison because the generator and renderer differ from the prototype's. Magnitude efficacy remains unvalidated: one rejected move proves the measuring and refusing, not that Jev's moves help.
R10: route, propose, evaluate, select and pause all ran live. The adopt-and-reassess step has still never run live, because nothing has improved yet.

Accounting: tokens 899,702 of 1,150,000; visual passes 31 of 32; images 50 of 62; evaluations 9 of 13; rounds 4 of 4. The second round cost 23,574 tokens, 1 evaluation, 4 images and no visual pass.

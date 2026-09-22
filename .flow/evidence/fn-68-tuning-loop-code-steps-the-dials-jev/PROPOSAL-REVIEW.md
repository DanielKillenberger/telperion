# Proposal design review — 2026-09-20

Status: offline design, not calibrated, not enabled in the live loop.

## What the recorded evidence establishes

The live flat-action question combines direction and magnitude. Its insufficient-evidence option covers either uncertainty. The independently tested direction question is not used by the live proposal path (`actions.rs`, `Live::propose`). Direction calibration therefore does not establish the live action decision.

The latest irregularity answer assigns 0.52 to small increase, 0.28 to substantial increase, 0.18 to insufficient evidence, and 0.01 each to hold and small decrease. Confidence is 0.42. Summed increase mass is 0.80; conditional small-size mass is 0.65. Neither calculation is a new model answer, a calibrated confidence, or permission to act. Splitting questions alone may still correctly abstain.

Magnitude calibration mostly tests interpreting explicit target numbers. `ambiguous-size` explicitly lacks both direction and size evidence. It does not test supported direction with unknown size. No held-out case tests a real overshoot history or selecting a useful experimental size from a visual defect. Historical direction labels are correlated; `regular-outline` was the one miss. Keep these original failures and receipts unchanged.

## Proposed contract

Ask for the next worthwhile bounded experiment, not a setting already known to work. A proposal never predicts acceptance: code measures it and visual review remains independent.

1. Direction Choice: increase, decrease, hold, insufficient evidence. Include only available directions. Identify the specific defect, dial meaning, current value, relevant same-revision observations and attributed historical failures. Hold means the dial already satisfies its requirement; insufficient evidence means no supported direction or relevance.
2. For supported directions only, conditional size Choice: small, substantial, insufficient evidence. Describe exact authored candidate values and bounds. Small means a justified conservative experiment when the larger step lacks support; it is not an automatic fallback from uncertainty. Substantial requires evidence for the larger probe, such as a relevant prior small trial being insufficient without regression. Neither safe bounds alone nor a vague visual defect establishes a useful experiment. Prior overshoot constrains the size. An unsupported direction must never be reversed merely because the opposite move is available.
3. Code consumes a pair only when both independently qualified judgments pass. It computes the existing authored value, validates the overlay, retains the four-candidate maximum, and measures before rendering. No summed-v1-confidence shortcut, confidence-scaled delta, automatic small-step fallback, or sweep.

Prefer a direction batch followed by a size batch only for eligible dials. This avoids asking both hypothetical size branches for every irrelevant dial. Compare actual serialized request cost before selecting it over a single speculative batch; TypeSafe supports both, and extra questions still cost tokens. Request count itself is not the optimization target.

## Qualification protocol before activation

Version the new questions and composed policy separately from v1. Existing model/version/table/state linkage and mandatory calibration remain. Do not mutate old labels or declare the failed pilot qualified.

Freeze new cases and thresholds before model calls. Use the existing 0.5 cut as a provisional experimental cut, not a claim of transferred calibration. Require at least 0.8 agreement for direction, size and the composed outcome separately, with zero unsafe composed proposals on the finite test set. Also report actionable-case coverage and inappropriate abstentions; always abstaining cannot pass. Do not multiply marginal probabilities into a claimed success probability.

Coverage must include: both directions; hold; missing/conflicting evidence; irrelevant dial; known direction but unjustified size; justified conservative probe; prior small step insufficient; prior overshoot; unavailable/discrete/fractional actions; and multiple defensible sizes with acceptable answer sets frozen beforehand. Group closely related cases so paraphrases cannot appear in both development and held-out sets. Previously inspected historical cases are development/replay evidence for v2, not fresh held-out proof. Authored policy tests cannot establish biological efficacy; a later separately authorized measured pilot remains necessary.

Seven offline composition checks passed using mocked answers: supported pair, uncertain direction, uncertain size, hold, insufficient size, unavailable size, and no opposite-direction fallback. They involved no model calls and prove only the proposed composition rules, not Jev adequacy.

## Budget and next boundary

Current cumulative spend remains 205,909 tokens, eight visual passes and two of two authorized rounds. No new render, model call, runtime policy change or cap extension occurred in this review. Proposed next stage is text-only Jev calibration with a 12,000-token maximum inside the existing 270,000 ceiling, pending owner approval. Check exact request reservations before dispatch; stop rather than shrink safety margins or silently exceed the allowance. Preserve all calibration costs separately and in cumulative accounting. Even passing that stage does not authorize another tuning round.

Sources: https://docs.typesafe.ai/primitives/choice.md and https://docs.typesafe.ai/patterns/fan-out.md (read 2026-09-20); local frozen cases, `pilot-corrected-outcome.json`, and the existing proposal implementation.

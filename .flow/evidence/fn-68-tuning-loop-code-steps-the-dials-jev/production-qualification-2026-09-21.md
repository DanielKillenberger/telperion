# Production qualification attempt, 2026-09-21

Host Claude Code `claude-fable-5-1`, under `owner-allowance-2026-09-21.md`. Commands are the shipped `tuning-loop` binary built at `a704f9ad` (CI-profile gate EXIT=0, 104 suites, on that commit). Reviewer: `scripts/reference-first-codex.py`, requested model `gpt-6-astra`, effort medium; the resolved model identity is not exposed. One attempt per step, no retry. Raw outputs stay in ignored scratch `.flow/tmp/fn68-canonical/`.

| Step | Result | Tokens in/out | Visual passes | Output sha256 |
| --- | --- | ---: | ---: | --- |
| 1 beech Stage A, 3 references, species `european-beech` | settled, verifies | 19,649 / 999 | 1 | inventory `4ee0fec10e894763295a3c4c01b5b250b7cd60844099caa166e9f3d707bfa023` |
| 2 birch Stage A, 1 reference, species `silver-birch` | settled, verifies | 18,437 / 810 | 1 | inventory `6a2c5ace2dc73cbc024edb7d67a959740c0e56980aed480464639c3735d2c2f6` |
| 3 freeze replay, 2 blinded cases | free | 0 | 0 | manifest `df4adb44e91d100781a7515db9faa9611f38d644ae6aa6bf2b42fbb3b4a49ffe` |
| 4 replay | positives 1, negatives 1, false_ready 0, **false_rejections 1**, abstentions 1 | 48,812 / 2,902 | 2 | result `9972b6d7c52e128d637ddfef4760ce868b43bd2237297e4bae0712db0fc9e7d8`, journal `8f05ccc600002dc78a6b9d57a58acd79c634ad84bd69c04b438bce2593bc9b42` |

Two free pre-dispatch refusals happened and cost nothing: a `__TODO` note key in the draft config (strict parser), and a beech replay case listing two references against the three-reference inventory.

## Outcome

The reviewer rejected the birch positive. Its blockers: exposed bark reads dark slate-gray with pale streaks where the reference is whitish with dark markings (core trait SB-002 fail), and bare radiating twig fans dominate the upper crown. Six core traits passed, two were unknown. The beech negative was rejected on foliage organization (upright plumes against spreading layered masses); its cell is Unknown because the render is clipped, so that case can never score ready and proves little about the reviewer.

With a false rejection, `Config::verify` demands a bounded convergence run. None exists and none may be manufactured. `tuning-loop preflight` on the filled canonical config (identity `ff7f086a…`) reports `calibration_error: stricter role: bounded convergence remains unproven`. The canonical run was therefore not started: no Jev call, no render, no evaluation, no image reservation. This is finish line 3 of the handover, a precise blocker, not success.

The birch case is a camera reframe of accepted historical geometry and that raster was never owner-accepted, so the rejection may be a correct reading of that image (dark bark) and not a reviewer error. That is unknown.

## Accounting after this attempt

Experiment tokens 688,550 + 20,648 + 19,247 + 51,714 = **780,159** of 902,431. Visual passes 25 + 4 = **29** of the approved 32. Image reservations 30, evaluations 6, rounds 2, unchanged. The beech Stage A is spent now even though no run has charged it; a future run with these pins charges it through `charge_preparation`, so its opening balance must be 759,511 tokens and 28 passes to avoid double counting.

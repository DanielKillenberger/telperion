# fn-342-where-todays-oak-goes-wrong-to-the-eye.1 Trace the jaggedness and the oak at 300 to their causes

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Traced, on today's unmodified cycle, the oak's jagged limbs (40, 120) and its form at 300 (seeds 1001, 1007), by lay-step cause tags, neutral-setting runs and a yearly record inside one grow. The note is .flow/evidence/fn-342-where-todays-oak-goes-wrong-to-the-eye/RESULT.md; the probe is probes/turns.patch, reverted from the crates.

Every joint of 10 degrees or more comes from a change of axis. Relays are 71 to 92% of those joints on limbs and boughs; fork and lateral takeovers, at the 34 to 57 degree insertion angle, are 67 to 74% on all wood 1 cm or more, mostly branch wood. Wander, tropism, sag, straightening and take-over each turn a joint by under 1 degree. The measured kink density per drawn pixel does not show 40 worse than 120; the eye's reading at 40 is recorded and left open to the host. At 300 every stem limb dies by the balance, when the upkeep of the dead wood it still holds (8 to 10 times its living upkeep) cancels its light. Limbs are born only at the apex, so none replaces them low down. Above 7 m the trunk is a chain of 35 to 67 limb relays.

Tier: single minted implicit task (model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none - only .flow/ and evidence change)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5eeda12bc9b9410bdad9af28140c358b144d6dac
- Tests: probes/runs.sh turns 40|120 [neutral], years 300 (telperion-heavy 8 --threads 4)
- PRs:
# Onboarding examples

Two populated, independently writable packets: [oak](oregon-white-oak/PACKET.md) and [spruce](norway-spruce/PACKET.md). Each contains its own profile, reference inventory, complete parameter manifest and separate specimen list. They reuse attributed evidence, not newly independent research or botanical approval.

`later-protocol.json` extends the original two-species/twelve-case cohort to three species/eighteen cases under `fn19-onboarding-v2-illustrative`. `later-references.json` adds one attributed Scots pine description. `frozen-identity.json` pins the original exact protocol/reference hashes and cases; originals are never rewritten. `verify.py` asserts this and calls the real task-2 validator with the frozen schema. The saved `validation.json` is the observed result; reruns print new evidence to stdout without overwriting it.

```bash
cargo build --release -p telperion-core --example geometry_benchmark
python3 .flow/evidence/fn19/onboarding-examples/verify.py
```

[Scots pine](scots-pine/profile.json) is an intentionally incomplete documentation fixture. [OSU's species description](https://landscapeplants.oregonstate.edu/plants/pinus-sylvestris), accessed 2026-09-06, describes paired needles with a persistent sheath. No source image was inspected for this fixture. Dimensions, detailed anatomy and calibration remain unmet. Its `species.json` copies spruce's full parameter *shape* only to exercise parsing; those values are not pine parameters. Required paired-needle anatomy and preset identity are unsupported, and actual admission returns `unsupported-anatomy`, `implemented=false`. Do not run generation or claim an admitted runnable third species.

`seed-audit.json` records a local tracked-text integer-token audit and three OS-random u32 reservations. These are ungenerated illustrative seeds, not a production evaluation freeze; audit all concurrent reservations before future use. Existing holdout-at-freeze labels preserve historical cases, not freshness after inspection.

Negative controls cover duplicate identities, unresolved reference IDs, profile hash mismatch, fixed/holdout reuse and missing capabilities. `workflow-examples.json` covers competing shared writes, coordinated single ownership, resource windows and absent expert feedback. Those checks illustrate coordinator decisions; the native validator has no ownership scheduler. See the [guide](../../../../docs/species-onboarding.md) for recovery and dispatch gates.

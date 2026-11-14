I measured the fix branch (fn-349 at 210f97006) with `retained` at 0, 0.5 and 1 on every PA, set through the still runner's existing `--girth` flag. At 0 every tree reproduces fn-349 node for node.

- **Old oak:** at 0.5 and 1 the trunk at 1.3 m no longer falls from 120 to 300 on any seed; at 1 it grows 13 to 28 %. The 300-year oak on seed 1001 stands 23 m instead of 7.7 m, and the oak at 200 on seed 1012 is drawn instead of refused.
- **Cost of that:** the trunks are 3 to 5.4 m across, and the old bases are drawn as stepped collars.
- **One-thread cost** (oak 300, seed 1001): unchanged, 120.6 to 122.0 s and 1.99 GiB.
- **Young trees** (oak 40 and 80, beech 40): only the trunk thickens, by 39 to 84 %.
- **What `retained` covers (code):** retired subtrees keep their frozen pipe at 1. A fading kept branch loses f(1 - f)r² of section until it is shed; this is inferred and was not measured.

Tables, sheets and the read are in RESULT.md. The probe patch is probes/kept.patch; it was reverted and the fn-349 checkout is clean.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: this is a measurement. No product code lands, no preset value changes, no gate, no codex review. (model: claude-opus-5-5)

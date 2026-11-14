# fn-316 friction

## 2026-10-25, the host
- **dcg blocks every redirect into the scratchpad** (`> $S/x`, `> /home/.../scratchpad/gate316.log`): four commands rewritten with `tee` or the Write tool, about 5 minutes. What would remove it: a literal scratchpad path allowed by dcg, or the habit of `tee` from the start (already noted in earlier specs' friction).
- **The CPU reference build overflows in the dev profile** (`telperion-space/src/shed.rs:211`, subtract with overflow) under `field_tests`; the first device run cost 50 s and a rerun under `--profile ci`. Unchecked on the base; RESULT.md lists it.
- **The spec's exact-count line was written unchecked** and cost a review round: AGENTS.md's "a spec rests only on claims somebody ran" applied to my own spec.

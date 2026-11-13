# fn-361 friction

## 13 Nov 2026, 03:13 (builder, Opus)

- **Doing:** R3, the narrow suites after adding `/space/ages/6/form/dominance = 0.5` to the oak's values.
- **What hindered it:** `space_presets::every_engine_preset_carries_the_engines_species` asserts the preset's `/space` species equals `telperion_space::oak()`, built in `crates/telperion-space/src/oak.rs`. That function has no dominance at the limb (0.0), so the row cannot pass without a change under `crates/*/src`, which the spec ("no code changes") and the brief forbid. The assertion's only difference is `dominance: 0.5` against `0.0`.
- **Cost:** about 25 minutes and one narrow-suite build before the stop showed; no pin could be repinned.
- **What would have removed it:** the spec checking whether each engine preset row is mirrored in the space crate's species function (the birch's 0.2 is, at `birch.rs:314`) before saying the row needs no code.

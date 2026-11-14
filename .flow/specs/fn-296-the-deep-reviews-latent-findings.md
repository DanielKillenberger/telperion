# The deep review's latent findings

The owner's in-depth codex review of 0.2.0 (2026-10-19) raised findings the host and a verifier judged latent: no shipped preset or normal package use reaches them, or they are hardening. The release takes the reachable ones in fn-297; these follow it. Each was checked against the code at `d7309355e`; outputs are in fn-297's evidence.

After the release (host, 2026-10-19).

## Requirements

- **R1 (C.1, C.2):** the same-value writes in `retire_axis.wgsl` `open_main` (`R_OPEN`) and `retire_units.wgsl` (`A_PHYTS`) have one writer or an atomic store.
- **R2 (C.8):** `select.wgsl` culls a thinned leaf at the sphere it is drawn at (enlarged about its seat by `1/sqrt(kept)`).
- **R3 (C.9):** `establish.wgsl` `bearing()` propagates lowering from the parent in depth order, or faults on exhausting its walk; never returns zero silently after 64 edges.
- **R4 (D3:D.1):** a growth state spent by a failure releases its device buffers and keeps only its failed-state metadata.
- **R5 (E1:E.1):** `Metrics::total_ms` either covers the whole preparation or is renamed to what it measures; `stages.totalMs` in the package says the same.
- **R6 (E3:E.1):** the device-wide curve `OnceLock` (`curve.rs:53`) cannot hold unfilled programs after a refused or cancelled prepare.
- **R7 (E3:E.2):** organ upload runs the size and level checks leaf upload runs, before any buffer is replaced.
- **R8 (E3:E.3):** the `frames` example keeps the family's leaf reflection when it applies a material variant.
- **R9 (E4:E.1):** in a crown mixing wood leaves and fronds, a frond box's reach is its stored thickness; only wood capsules are padded.
- **R10 (E5:E.1):** `blend::centroid` of angular rows is permutation-invariant, as its doc promises; two-point behaviour unchanged.
- **R11 (E6:E.1, E6:E.2):** an empty intermediate presentation snapshot does not fail growth (the collapse error stays for an empty final tree); the key-span helper represents `CONTINUATION = u64::MAX`.
- **R12 (open items):** the late fault reads (light-point capacity after the gather, FAULT_TWINS after the last read) move before their writes; the wake traversal's 4,096-axis / 65,536-link cap faults instead of truncating.
- Each repro runs as a test before its fix (AGENTS.md, "Gates and checked claims"); a finding that does not reproduce is recorded and closed with no change.

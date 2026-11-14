# A tree grown twice on the device grows the same field

## Goal & Context

Found by the host on 28 Oct 2026 during fn-319: the device field is not repeatable within one process. The owner wants a high-quality 0.2.0 (27 Oct 2026, "We want high quality 0.2.0"), and fn-318's pregeneration rests on byte equality between runs.

Reproduced on F4 before fn-319 (`81ba1c655`) and at the fn-319 head (`547241e0c`), RTX 3080 natively, `Generator::grow_field` called repeatedly on one generator, n 24 (`/tmp/fn319/birch-probe.rs`, `/tmp/fn319/repeat-probe.rs`; evidence in fn-319's `FIELD-PROGRAMS.md`):

- silver birch seed 5678 on F4: every one of 30 later grows differs from the first; seed 1: 3 of 30.
- Oregon white oak seed 1 at the fn-319 head: 2 of 8 repeats differ, the frame among them (the tree's box moved), 959 cells' wood radius and 390 cells' leaves.
- Ordinary seed 5678: 1 of 8 differs in one cell's leaves by 2.4e-4 (an order-dependent float sum).
- `every_preset_grows_its_field_from_its_parameters_as_from_its_id` failed once on silver birch for this reason.

Unknown: whether the drawn tree itself differs between grows (growth on the device) or only the field's read-out, and whether the renderer's device build shares the cause.

## Acceptance Criteria

- **R1:** the cause is named from a measured repro, per class (a moved box, an order-dependent float sum), before any fix.
- **R2:** every shipped preset at seeds 1 and 5678 grows a byte-identical field on 30 repeats in one process and across processes on the same GPU, as an ignored measurement test, and an ordinary test repeats the worst case found.
- **R3:** where the cause also reaches the renderer's device build, the same holds for its digest.
- **Errors:** no error surface; a fix that changes answers states the change with the fn-319 answer hashes.

## Boundaries

Cross-GPU agreement stays fn-318 R4's. No preset change.

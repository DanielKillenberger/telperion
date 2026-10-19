# Executors agree on the drawn tree, not its keys

## Goal & Context

The owner (2026-10-16) accepted the oak's device/CPU difference at 40 for the release: one node at `FINEST` rounded apart by f32 and f64, and 185 relabelled keys at matching positions and radii, nothing drawn differing (fn-259 RELEASE.md); the agreement test that compares drawn geometry is after the release. Astra's review (ASTRA-RELEASE-REVIEW.md, "After 0.2.0" 4) concurs.

## Requirements

- **R1:** cross-executor agreement compares the delivered wood and foliage within measured tolerances at every preset's shipped age, so labels alone do not fail an accepted tree; exact repeatability within an executor, the discrete identities a consumer reads, attachment and finiteness checks stay.
- **R2:** a threshold-adjacent node (radius within rounding of `FINEST`) is judged the same on both executors, or the difference is bounded by the tolerance.

## Boundaries

After the release (fn-237).

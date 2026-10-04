# Foliage selection dispatches past 65,535 workgroups

## Goal & Context

The renderer's foliage selection (`crates/telperion-render/src/select.rs:365`) dispatches one dimension of `instances / WORKGROUP` groups. Above 65,535 groups (about 4.2M instances at a workgroup of 64) wgpu panics: "dispatch group size dimension ([74572, 1, 1]) must be less or equal to 65535" (fn-194 FRICTION.md, 2026-10-04). The spruce had to thin its needles from 2.5 to 3.5 mm to stay under the limit, and Astra's three samples on round 4 all failed it for missing needle mass. The generation path already splits a large dispatch over a second dimension (`generation/io.rs:133`). Obvious fix under AGENTS.md "Friction reports": the cause is named and the remedy changes no product behaviour (host, 2026-10-04).

## Requirements

- **R1:** The select passes that dispatch per instance split their groups over x and y as `generation/io.rs` does, and the shaders reconstruct the flat index from both and skip past the end. The per-level pass is unchanged.
- **R2:** A test, red first on the base, that selects a foliage set larger than 65,535 workgroups without a panic and selects the same instances a flat dispatch would for a set under the limit (byte-identical selection below the limit).
- **R3:** Every shipped preset renders byte-identically (the existing headless and catalogue tests stay green).
- **R4:** Workspace gate green.

## Boundaries

The dispatch only. No change to which leaves are selected, to LOD or to the frame budget.

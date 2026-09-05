# Surface comparison

From the repository root, after installing the pinned Rust toolchain and npm dependencies:

    node crates/telperion-core/tests/surface_reference.mjs

This exports ordinary, Telperion, Laurelin, empty and capped solved trees from pinned final FN6, computes Three vertex normals, and runs the ignored native comparison. Optional positional arguments select cases. REFERENCE_OUTPUT selects the temporary output directory. Large buffers are not committed. The exporter verifies reference provenance; an unavailable reference fails visibly.

The runner diagnoses matched topology with index equality, scale-aware float32 position tolerance, unit-normal tolerance and bounds. Current outputs match exactly. A deliberate geometry change should replace elementwise comparison with suitable invariants and recorded drift, not preserve a known reference defect. These checks establish the surface stage against solved trees, not the new grower, browser appearance, or total generation performance.

The regular cargo suite covers taper, closed outward winding, normal length, bounds, socket containment, zero-length edges, reversal, determinism, valid emptiness, malformed inputs, float32 limits and allocation failure. The surface rejects parameters outside their documented TS rails rather than silently clamping them; lobe count still raises radial resolution to four samples per lobe. An unrepresentable or collapsed float32 surface returns an error with no partial mesh.

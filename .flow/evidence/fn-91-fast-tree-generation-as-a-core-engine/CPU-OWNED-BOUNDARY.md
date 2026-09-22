> Historical raw-output references: see the [archive and recovery instructions](README.md).

# CPU-owned surface boundary — source analysis, not implementation

The .8 GPU-assisted CPU-owned oak control spends roughly185–188ms in ordinary CPU wood construction, while resident GPU delivery avoids that expansion. This remains a distinct bottleneck after shared skeleton improvements. The .3 diagnostic attributes comparable shares to vertex/index emission and normal accumulation; its intrusive timings are approximate, not current production per-stage costs.

A potential bounded alternative to full GPU mesh readback is parallel native expansion of independent surface runs into disjoint slices of the final CPU arrays. Each run has independent vertices and normals; preserving sorted run order, per-run triangle order and f64-to-f32 accumulation order can preserve output bytes. This has not been built or timed. It does not promise browser CPU-output gains or10x delivery.

Memory is the admission constraint. Building separate per-worker meshes and then flattening them can duplicate the complete output and is unsuitable. Preallocated final slices avoid that copy, but worker stacks/scratch and run-offset metadata still count. The current serial builder reserves all output arrays before its per-run loop while retaining paths, distances and sampling scratch. A two-phase design could emit positions/coordinates, then release unneeded preparation storage before allocating final normals/indices; it must demonstrate a lower or equal peak including worker resources. Calling this allocation-free would be false.

Collapsed triangles and zero-normal facing fallback complicate a two-phase path because canonical facing uses transported frames. Retaining a facing vector for every vertex would be too expensive. A bounded parallel capability path could detect such runs and drop candidate allocations before using the unchanged serial builder, or compute fallback frames only where needed. Neither option is selected here. Stable run tables, compaction, bounds, error ordering, actual thread availability and failed thread creation require explicit design. Native threading should not impose a thread requirement on Wasm consumers.

This is a candidate architecture for a later measured screen, not an approved change or task state. Full GPU readback remains another possible design, but transfer completion and overlapping source/staging/CPU ownership must be counted before selecting it. No CPU-output speedup or memory qualification follows from the resident GPU results.

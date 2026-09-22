> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Station preparation candidate

Host approved the shared stage on 2026-09-20 before production edits. Child membership is represented by count and last child; the last index is consumed only for a sole child. Node enumeration, bearing classification, branch equality and run order stay unchanged.

The frame iterator consumes the same normalized segments and adjacent averaged tangents. Its carried normal is orthogonalized against the averaged tangent and normalized before output projection. The station projection modifies a separate tuple; it never feeds the next transport. The terminal point frame is no longer calculated because no station consumes it. Canonical frame code remains private to tests.

Ordinary rotation uses the existing Rodrigues helper with normalized (cross length, clamped dot) instead of atan2 followed by sine/cosine. The canonical branch remains at dot below -0.999999, at an uncertain norm, and at the original antiparallel threshold. No parameter acceptance range changes. Full prepared records compare all nonframe fields and tangent bits exactly, with component frame error bounded at 1e-10; the smaller numerical unit fixtures use 1e-12 against canonical and independent orthonormal checks.

## Explicit allocation proof

The child array is 16 bytes per node instead of 24 bytes per node plus all nonempty child heap allocations. Both are destroyed before bearing_runs returns. Run allocations, ordering and capacities are unchanged.

Prepared runs retain exactly the existing per-run points and along arrays. There is no retained-longest scratch: a long early run cannot inflate late-output residency. Segment directions, averaged tangents and frame tuples become constant iterator state instead of three vectors. Output StationSegment count, order and push schedule are unchanged, hence output capacity is unchanged. The public CPU placement path still collects station frames, now N-1 tuples into an exact iterator collection instead of N tuples grown geometrically, and no segment/tangent vectors. CPU per-run explicit allocations cannot increase for any run ordering; all removed storage coexisted with the same points/along/output in the baseline. Allocator bookkeeping and driver storage are not measured by this proof.

One baseline/candidate screen uses separate Cargo roots and target directories. Each fixture runs immediately baseline then candidate, first plus three warm samples. Serialization and comparisons happen outside timing. The runner derives from .9 but stops after preparation, avoiding full CPU foliage work.

## Admission overlap and ownership

The host admitted the fixed overlap after the native screen despite the50% aspiration miss. `begin_positions` owns its compact surface, resident position/metadata buffers, five scratch buffers and a `PendingRead`. The read constructor submits its copy and calls map_async synchronously. `complete_positions` polls/awaits that already-started read, checks full geometry admission and only then creates UploadedWood. Existing contact-bearing and direct-test callers use the wrapper and retain their dependency order.

Zero-contact preparation captures its station Result while the ticket is live, then always completes the ticket and pops GPU error scopes before inspecting that Result. A station error keeps its former precedence. Unsupported station capability explicitly drops any admitted candidate and clears retained-position flags before canonical fallback. Attempted GPU peak snapshots remain recorded. An error while starting the ticket drains submitted work before scope cleanup. No foliage kernel sees pending geometry.

Position upload time now includes starting status readback. Position wait time measures only the wait remaining after CPU stations. Descriptor time includes CPU stations; early wood time excludes that CPU duration. These stage numbers describe host walltime sections and cannot be added as estimates of independent GPU execution time. Completed-frame walltime is the qualification metric.

The diagnostic measures actual child, retained outer/inner run, points, distances and output capacities. Its maximum conservatively includes the previous output allocation during growth. The whole new overlap envelope adds that entire station maximum to previous-tree GPU, base CPU, position CPU (even metadata already dropped) and position GPU peak. Native usize-based vector bookkeeping conservatively exceeds corresponding Wasm bookkeeping. Old and new phase envelopes use maxima, never summed nonoverlapping rejected-candidate lifetimes. Allocator, queue-write staging, driver/compiler and deferred-destruction peaks remain unmeasured.

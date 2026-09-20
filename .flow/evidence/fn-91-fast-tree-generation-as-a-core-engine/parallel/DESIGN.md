# Bounded native surface candidate

The proposed ordinary CPU wood path uses eight workers at most, selected once from available_parallelism, with a minimum of 250,000 vertices and two workers. PreparedSurface and contacts continue on the existing serial path. Wasm excludes the module at compilation. Threads have explicit 64 KiB requested stacks and nonrecursive kernels. No external dependency or unsafe output initialization is needed.

The measured capacity inventory in capacity.log comes from a scratch copy of e453ac51 before production edits. Its arrays use actual Vec::capacity times element size. The serial floor sums all output, paths, distance, ordering, angular and sample/frame scratch arrays concurrently reserved by the existing builder. Stable sorting also allocates transient workspace while outputs are live; excluding this transient from the serial floor is conservative. The candidate sorts before output allocation and drops its ordered vector before workers launch.

| Case | Serial live-capacity floor | Candidate phase A | Candidate phase B | Margin |
| --- | ---: | ---: | ---: | ---: |
| oregon-white-oak 1 | 213,845,663 | 83,602,039 | 210,772,016 | 3,073,647 |
| oregon-white-oak 7 | 244,426,544 | 95,378,600 | 240,860,592 | 3,565,952 |
| norway-spruce 1 | 174,684,573 | 68,489,757 | 173,014,688 | 1,669,885 |
| norway-spruce 7 | 165,388,647 | 64,905,015 | 163,915,136 | 1,473,511 |

All values are bytes and include the 16 KiB fixed control allowance. Phase A owns final zero-initialized position/coordinate vectors, shared paths/distance/angular input, 32-byte run descriptors, and eight longest-run sample/frame/segment scratch sets. Each descriptor combines the prepared Run shape and radius, preserving sorted ties. Contiguous partitions balance cumulative vertex counts, with run boundaries preventing any shared writable vertex. Generic shared emission keeps the same sample/frame/angular formulas and cap ordering.

After phase A joins, paths/distance/angular and scratch are dropped. Phase B allocates final normals/indices and run table, retaining only the 32-byte descriptors. It partitions contiguous disjoint position/normal/index slices and uses the canonical triangle order, float64 cross product and float32 accumulation. The final bounds scan remains canonical. A fixed-size handle/partition array requires no heap-sized run scheduling queue.

The table reserves 64 KiB stack, 4 KiB guard and 64 KiB opaque runtime allowance per worker. Phase B charges both generations of eight workers to cover retired stack reservations. Fixed local descriptors/handles and scope packet costs fit inside the runtime allowance. This is an explicit conservative accounting envelope on the measured x86_64 Linux host, not a portable proof of allocator, TLS, stack-cache or whole-process RSS behavior. Thread stack minimum/guard requests differ by platform. Qualification must retain that limitation and observed RSS separately.

The implementation checks the same formulas against actual input capacities before admission, adding 16 KiB fixed control allowance. If either candidate phase exceeds the conservative serial floor, it uses serial build. Thread availability failure or a single-core result also uses serial. Any spawn failure, worker error, collapsed triangle or zero normal joins all started workers and drops candidate vectors before one unchanged serial retry. The retry has threading disabled, so fallback occurs at most once per call. The all-four screen requires zero fallback; unsupported parameter sets remain usable through serial. Errors are produced by the canonical serial path after candidate cleanup, retaining error ordering and collapse counts.

The smallest screen is the radius-order runner with its full bitwise mesh/compact/contact record, immediate baseline and one candidate, oak/spruce seeds 1/7, first plus three warm runs. Instrumented capacities run separately. At least 2x warm wood speed on both oak specimens is required; 6x is the aspiration. There is one worker configuration and no tuning loop. A miss archives/reverts the candidate. Actual native CPU-output and GPU-assisted CPU-output qualification follows only a host retention decision.

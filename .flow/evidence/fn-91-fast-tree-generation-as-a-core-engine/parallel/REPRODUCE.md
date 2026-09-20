# Reproduce the bounded candidate

Start from e453ac51b2a9024ac2e54ff42cb2d72b84f32bff in a disposable checkout with its own Cargo target directory. The baseline source capacity probe is capacity.patch plus parallel_capacity.rs copied into crates/telperion-core/examples. Build and run that example in release mode to recover capacity.log.

For the bitwise paired screen, reuse ../radius-order/radius_order.rs as the core example and its setup.py addition of diagnostic_edges to CompactWithContacts. Build and preserve the baseline executable before applying candidate.patch. That patch contains the retained production implementation, its focused tests, the example seed diagnostic and truthful limit inventory. The final tests-only relocation does not alter the kernels in the original screen source hashes.

Build and preserve the candidate executable after applying the patch. prepare-screen.py documents the exact scratch-only relaxed counters and JSON field used for the measured candidate. screen.sh specifies the serial first-plus-three-warm all-four protocol and complete byte comparisons. Its scratch-path.txt should name the disposable checkout. analyze.py summarizes the result. For the candidate capacity snapshot, prepare-capacity.py shows the separate untimed instrumentation.

For complete owned output, compile generation_gpu once from each revision, preserving distinct executables. Both use the example-only GENERATION_SEED reader in candidate.patch. measure.py specifies all four specimens, both owned-output backends, first plus three warm, initialization and wait4 maximum RSS. summarize-delivery.py compares medians and the original CPU-owned baseline. Update executable paths in the copied script to the preserved pair. Use a separate target directory per checkout; sharing one caused the rejected cache observation documented in REPORT.md.

All builds and timed processes are serialized. Full mesh dumps and executables stay outside the repository; their hashes and source hashes are recorded here. The archived scratch binary includes diagnostics only where declared. No new browser timing matrix is needed for this Linux x86_64-only path.

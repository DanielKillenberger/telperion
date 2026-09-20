> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Reproduction

Archive bf3e2fbb671f6832a8a5112a5b21691cd9d1e253 into two disposable directories named baseline and candidate under one parent. Give each its default local Cargo target directory. Copy station_screen.rs into each crates/telperion-core/examples/. Apply candidate.patch to the candidate root. Write paths.json with {"root":"/absolute/parent"}; build each example with cargo build --release -p telperion-core --example station_screen from its root. Run screen.py once; it performs the immediate baseline/candidate first-plus-three-warm matrix and compares every dumped station field and repeat outside timing. Preserve these binaries before the diagnostic instrumentation changes them.

For the allocation diagnostic, run capacity.py once on those disposable source roots, rebuild both examples, then run capacity-run.py. It records actual child/run/points/along/frame/output capacities and conservatively includes old output allocation during growth. Diagnostic timings are ignored. The baseline browser package was copied before rebuilding; baseline-browser-package.json records every package hash and saved location. Its Wasm SHA exactly matches the retained .8 package in ../position-integration/browser-candidate.json.

Renderer test logs capture explicit release --lib targets with test(generation::) and one test thread. late-station-red.log records the new caller-lifecycle check failing before the cfg(test) result seam was connected; renderer-tests-final.log records all20 passing after connection. The seam changes no production code behavior.

#!/usr/bin/env bash
# fn-371 R4/R6: the three repinned tests again, the larger tests that grow
# every preset's drawn tree, and the two sub-second guards. One test at a
# time (ground_reach and suite::mesh each peak near 4.5 GiB). Run inside one
# telperion-heavy job.
set -u
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
cargo test --profile ci -p telperion-core --no-fail-fast \
  --test catalogue_identity --test ground_reach --test material_detail -- --test-threads 1
echo "integration_rc=$?"
cargo test --profile ci -p telperion-core --lib --no-fail-fast -- --test-threads 1 \
  cycle_cost suite::mesh suite::foliage
echo "lib_rc=$?"
cargo test --profile ci -p telperion-core --test generation_limit_guard
echo "guard_core_rc=$?"
cargo test --profile ci -p telperion-render --lib compile_size
echo "guard_render_rc=$?"
echo "TESTS DONE"

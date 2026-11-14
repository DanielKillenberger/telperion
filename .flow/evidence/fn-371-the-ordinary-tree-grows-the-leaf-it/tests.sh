#!/usr/bin/env bash
# fn-371 R4/R6: the narrow tests that read the ordinary tree's values, every
# failure collected (no fail-fast). Run inside one telperion-heavy job.
set -u
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$ROOT" || exit 1
echo "== integration"
cargo test --profile ci -p telperion-core --no-fail-fast \
  --test catalogue_identity --test space_walks --test walk_sizes --test space_presets \
  --test light_rows --test grown_total --test preset_values --test capability \
  -- --test-threads 2
echo "integration_rc=$?"
echo "== lib"
cargo test --profile ci -p telperion-core --lib --no-fail-fast -- --test-threads 2 \
  cycle_cost suite::drop suite::sag suite::strands suite::twig_generations suite::sweep \
  suite::field_plan suite::growth_reference suite::generation_limits suite::outline \
  suite::limb_clumping suite::specimen_cache suite::codominance suite::crown_reference \
  relay_tests branching::audit branching::specimen fork_tests family::tests pipeline::tests
echo "lib_rc=$?"
echo "TESTS DONE"

#!/usr/bin/env bash
# fn-351 speed round 2: the narrow proofs that a change left every tree as
# it was, by name, in one queue admission:
#
#   TELPERION_HEAVY_RESERVE=8 telperion-heavy 4 --threads 4 -- \
#     .flow/evidence/fn-351-020-the-release-candidate/speed2-tests.sh <label> [all]
#
# - the engine's own: the paused grower's equalities, the wood's, the
#   light's, many cores, and the retirement's against the reference
#   retiring nothing;
# - the pinned digests: every preset's tree at sixteen (`PINNED`), and the
#   catalogue's identity;
# - with `all`, every preset grown again and on any cores (about 3 minutes).
# Logs land in raw/speed2/<label>/tests/.
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
out="$here/raw/speed2/$1/tests"
mkdir -p "$out"
cd "$root" || exit 1
rc=0
cargo test --profile ci -p telperion-space --lib -- \
  grow::cycle::paused::tests:: \
  girth::tests:: \
  light::tests:: \
  grow::tests::many_cores_grow_the_tree_one_does \
  grow::growing_tests::a_growing_phytomer_holds_what_growth_reads \
  grow::tests::an_axis_and_its_draws_stay_compact \
  grow::cycle::tests::the_working_set_and_retirement_leave_the_tree_as_it_was \
  grow::cycle::tests::woken_buds_that_stop_at_once_retire_as_the_reference_has_them \
  grow::cycle::tests::letting_go_of_retired_records_leaves_the_tree_as_it_was \
  >> "$out/space.log" 2>&1 || rc=1
cargo test --profile ci -p telperion-core --lib -- \
  every_preset_grows_the_tree_it_grew_at_sixteen \
  >> "$out/pin.log" 2>&1 || rc=1
cargo test --profile ci -p telperion-core --test catalogue_identity \
  >> "$out/catalogue.log" 2>&1 || rc=1
if [ "${2:-}" = all ]; then
  cargo test --profile ci -p telperion-core --lib -- \
    --ignored every_preset_grows_the_same_tree_again_and_on_any_cores \
    >> "$out/repeat.log" 2>&1 || rc=1
fi
echo "tests rc=$rc $(date '+%F %T')" >> "$out/STATUS"
grep -h "^test result\|FAILED\|panicked\|failed" "$out"/*.log | tail -20
exit "$rc"

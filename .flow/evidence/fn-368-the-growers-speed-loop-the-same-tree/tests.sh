#!/usr/bin/env bash
# fn-368 R3: the narrow tests after a kept round, by name, through the queue.
#
#   .flow/evidence/fn-368-the-growers-speed-loop-the-same-tree/tests.sh <label>
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
out="$here/raw/$1"
mkdir -p "$out"
cd "$root" || exit 1
export TELPERION_HEAVY_RESERVE=8
rc=0
# The paused grower's tests, many cores, the size tests and the retirement's
# own equality with the reference retiring nothing.
telperion-heavy 4 --threads 4 -- cargo test --profile ci -p telperion-space --lib -- \
  grow::cycle::paused::tests:: \
  grow::tests::many_cores_grow_the_tree_one_does \
  grow::growing_tests::a_growing_phytomer_holds_what_growth_reads \
  grow::tests::an_axis_and_its_draws_stay_compact \
  grow::cycle::tests::the_working_set_and_retirement_leave_the_tree_as_it_was \
  grow::cycle::tests::woken_buds_that_stop_at_once_retire_as_the_reference_has_them \
  grow::cycle::tests::letting_go_of_retired_records_leaves_the_tree_as_it_was \
  > "$out/space.log" 2>&1 || rc=1
# The preset digest pin, then every preset again and on any cores (ignored
# outside the gate, run by name).
telperion-heavy 4 --threads 4 -- cargo test --profile ci -p telperion-core --lib -- \
  every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial \
  > "$out/pin.log" 2>&1 || rc=1
telperion-heavy 4 --threads 4 -- cargo test --profile ci -p telperion-core --lib -- \
  --ignored every_preset_grows_the_same_tree_again_and_on_any_cores \
  > "$out/repeat.log" 2>&1 || rc=1
echo "tests rc=$rc $(date '+%F %T')" >> "$out/STATUS"
exit "$rc"

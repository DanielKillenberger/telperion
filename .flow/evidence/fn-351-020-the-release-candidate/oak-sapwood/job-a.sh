#!/usr/bin/env bash
# fn-351, OAK-SAPWOOD-BY-SIZE.md: the narrow tests, the two probes and the
# small trees, as one heavy job (the queue admits slowly; one admission).
#
#   telperion-heavy 4 --threads 4 -- bash job-a.sh
#
# The probes are the scoreboard's (fn-352 scoreboard/probe) with one line
# added to engine.rs, `println!("DIGEST {:016x}", tree_digest(&s))`: the
# "before" one built against .worktrees/cand's crates (the base commit), the
# "after" one against this checkout's, each into its own target.
set -u
ROOT=/home/daniel/Projects/telperion/.worktrees/oakwood
RAW=$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/raw
OUT=$RAW/oakwood
PEAK=$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/peak.py
mkdir -p "$OUT"
cd "$ROOT" || exit 1
step() { echo "== $* $(date +%T)"; }

step space unit tests
cargo test --profile ci -p telperion-space --lib -- girth species 2>&1 | tail -15
step refusals
cargo test --profile ci -p telperion-space --test refusals 2>&1 | tail -8
step walks of the new rows
WALK=form.sapwood cargo test --profile ci -p telperion-space --test walks \
  every_setting_changes_the_tree_by_degree 2>&1 | tail -25
step reference rewritten
TELPERION_WRITE_REFERENCE=1 cargo test --profile ci -p telperion-core --test parameter_reference 2>&1 | tail -6
step core presets and identity
cargo test --profile ci -p telperion-core --no-fail-fast --test space_presets --test preset_values \
  --test catalogue_identity --test light_rows --test controls 2>&1 | grep -E "^test |test result|left|right|panicked|^  [0-9]" | tail -60
cargo test --profile ci -p telperion-core --lib catalogue 2>&1 | tail -6

for side in cand oakwood; do
  step build probe "$side"
  cargo build --profile ci --manifest-path "$RAW/probe-$side/Cargo.toml" \
    --target-dir "$RAW/target-$side" 2>&1 | tail -3
done

tree() {  # <side> <preset> <age|preset> <seed>
  local log="$OUT/$1-$2-$3-$4.log"
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && return 0
  python3 "$PEAK" "$RAW/target-$1/ci/scoreboard-probe" tree "$2" "$3" "$4" "$OUT/none" none > "$log" 2>&1
  echo "$1 $2 $3 $4: $(grep -c '^TREE' "$log") $(tail -1 "$log")"
}

step other presets, both sides
for p in norway-spruce european-beech silver-birch date-palm ordinary laurelin telperion; do
  for side in cand oakwood; do tree "$side" "$p" preset 1001; done
done
step the oak at 40 and 80, both sides
for age in 40 80; do
  for seed in 1001 1007 1012; do
    for side in cand oakwood; do tree "$side" oregon-white-oak "$age" "$seed"; done
  done
done
step the old oak, seed 1001, after
for age in 120 200 300; do tree oakwood oregon-white-oak "$age" 1001; done
step done

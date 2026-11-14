#!/usr/bin/env bash
# fn-351, OAK-SAPWOOD-BY-SIZE.md: the old oak on seeds 1007 and 1012 with the
# rows by height, measured without a drawing (job-a.sh's "after" probe).
#
#   telperion-heavy 8 --threads 4 -- bash job-b.sh [seed...]
set -u
ROOT=/home/daniel/Projects/telperion/.worktrees/oakwood
RAW=$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/raw
OUT=$RAW/oakwood
PEAK=$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/peak.py
mkdir -p "$OUT"
cd "$ROOT" || exit 1
cargo build --profile ci --manifest-path "$RAW/probe-oakwood/Cargo.toml" \
  --target-dir "$RAW/target-oakwood" 2>&1 | tail -3
for seed in "${@:-1007 1012}"; do
  for s in $seed; do
    for age in 300 120 200; do
      log="$OUT/oakwood-oregon-white-oak-$age-$s.log"
      [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && continue
      python3 "$PEAK" "$RAW/target-oakwood/ci/scoreboard-probe" tree oregon-white-oak "$age" "$s" "$OUT/none" none > "$log" 2>&1
      echo "$age $s: $(tail -1 "$log") $(date +%T)"
    done
  done
done

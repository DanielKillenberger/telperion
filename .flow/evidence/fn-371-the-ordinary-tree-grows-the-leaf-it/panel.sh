#!/usr/bin/env bash
# fn-371 R3: the ordinary tree at 40 and 80 on seeds 1, 7, 1001 and 1007,
# measured by fn-352's scoreboard probe (height, spread, trunk at 1.3 m,
# drawn leaf, phytomers, seconds) and fn-370's leaf probe (the grower's
# leaf, the mean blade from the rows), one process a tree. With STILLS=1 it
# draws only the sheet's tiles instead (40 on 1001 and 1007, whole and mid),
# which needs the GPU. Run inside one telperion-heavy job.
#   bash panel.sh <label>
set -u
F="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(git -C "$F" rev-parse --show-toplevel)"
S="$ROOT/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard"
T="${BIN:-$F/raw/target/ci}"  # BIN: a saved build's binaries (raw/bin-before)
OUT="$F/raw/${1:?label}"
mkdir -p "$OUT/stills"
if [ "${STILLS:-0}" = 1 ]; then
  for seed in 1001 1007; do
    python3 "$S/peak.py" "$T/scoreboard-probe" tree ordinary 40 "$seed" \
      "$OUT/stills/ordinary-40-$seed" whole,crown > "$OUT/stills/ordinary-40-$seed.log" 2>&1
  done
  echo "STILLS DONE"
  exit 0
fi
for age in 40 80; do
  for seed in 1 7 1001 1007; do
    python3 "$S/peak.py" "$T/scoreboard-probe" tree ordinary "$age" "$seed" x none \
      > "$OUT/tree-$age-$seed.log" 2>&1
    "$T/leaf-probe" ordinary "$age" "$seed" > "$OUT/leaf-$age-$seed.log" 2>&1
  done
done
ROWS=1 "$T/leaf-probe" ordinary 40 1001 > "$OUT/rows.log" 2>&1
grep -h '^TREE ' "$OUT"/tree-*.log | sed 's/^TREE //' > "$OUT/measures.jsonl"
grep -h '^LEAF ' "$OUT"/leaf-*.log | sed 's/^LEAF //' > "$OUT/leaf.jsonl"
echo "PANEL DONE"

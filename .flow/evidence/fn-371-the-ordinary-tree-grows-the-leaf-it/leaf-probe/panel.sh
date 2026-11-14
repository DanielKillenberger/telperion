#!/usr/bin/env bash
# fn-370 R1/R5: every shipped preset at its preset age on seeds 1001 and
# 1007, the oak also at 120; one LEAF line each, with its seconds,
# appended to raw/<label>.jsonl. Run inside one telperion-heavy job.
#   bash panel.sh <label>
set -u
D="$(cd "$(dirname "$0")/.." && pwd)"
PROBE="$D/raw/target/ci/leaf-probe"
OUT="$D/raw/${1:?label}.jsonl"
TREES="${TREES:-oregon-white-oak:preset norway-spruce:preset european-beech:preset silver-birch:preset date-palm:preset ordinary:preset laurelin:preset telperion:preset oregon-white-oak:120}"
for t in $TREES; do
  for seed in ${SEEDS:-1001 1007}; do
    p="${t%%:*}"; a="${t##*:}"
    TIMEFORMAT="TIME $p $a $seed %R"
    { time "$PROBE" "$p" "$a" "$seed"; } >> "$OUT" 2>&1
  done
done
echo "DONE" >> "$OUT"

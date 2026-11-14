#!/usr/bin/env bash
# fn-344 host follow-up 2: A and B on seed 1001 grown to 150, not drawn, with
# the balance pass's whole-tree numbers every 10 years (BAL lines). B stops
# at eight times the most A had held by that year (A's 300 grow's curve) or
# 12.5 GiB resident:
#   telperion-heavy 14 --gpu 2 --threads 4 -- bash probes/balance.sh
set -u
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for v in A B; do
  set=$([ "$v" = B ] && echo B || echo "")
  ABLATE="$set" ABLATE_CENSUS=1 PROBE_YEAR=1 PROBE_BALANCE=1 ABLATE_GROW_ONLY=1 \
    ABLATE_CURVE="$E/raw/curve-300-1001-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=12.5 \
    python3 "$PEAK" target/ci/examples/space_oak "$out" 150 --seeds 1001 --tag "${v}bal" \
    --shots bare > "$out/oak-150-1001-${v}bal.log" 2>&1
  echo "balance $v rc=$? $(date +%T)"
done

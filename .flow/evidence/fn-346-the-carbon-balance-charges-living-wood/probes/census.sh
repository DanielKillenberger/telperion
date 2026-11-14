#!/usr/bin/env bash
# fn-346 R1: the oak grown, not drawn, with fn-344's yearly census and BAL
# lines and, every 10 years, every living tip's path (PROBE_PATHS).
# Stops at 8 GiB resident or eight times the most A had held (fn-344's
# curve-300-1001-peak.txt).
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/census.sh <A|B> <age> <seed>
set -u
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
v="$1"; age="$2"; seed="$3"
out="$E/raw/$v-$seed"; mkdir -p "$out"
set_=$([ "$v" = B ] && echo B || echo "")
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
ABLATE="$set_" ABLATE_CENSUS=1 PROBE_YEAR=1 PROBE_BALANCE=1 ABLATE_GROW_ONLY=1 \
  PROBE_PATHS="$out" ABLATE_CURVE="$F/raw/curve-300-1001-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=8 \
  python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "$v" \
  --shots bare > "$out/run.log" 2>&1
echo "census $v $seed rc=$? $(date +%T)"

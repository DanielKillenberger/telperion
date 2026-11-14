#!/usr/bin/env bash
# fn-344 host follow-up: B (dead wood not charged) grown to an age with its
# yearly census, stopped at eight times the most A had held by that year or
# 20 GiB resident; with GROW_ONLY=1 it is not dressed or drawn. The GPU is
# requested by the still runner, so it runs under the GPU admission:
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/b-census.sh <age> <seed>
#   GROW_ONLY=1 ... for the census alone; unset for the stills (tag Bh).
set -u
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
age="$1"; seed="$2"
curve="$E/raw/curve-$age-$seed-peak.txt"
[ -f "$curve" ] || awk '{if ($2 > m) m = $2; print $1, m}' "$E/raw/curve-$age-$seed.txt" > "$curve"
kind=$([ -n "${GROW_ONLY:-}" ] && echo census || echo stills)
log="$out/oak-$age-$seed-Bh-$kind.log"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
ABLATE=B ABLATE_CENSUS=1 PROBE_YEAR=1 ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=20 \
  ABLATE_GROW_ONLY="${GROW_ONLY:-}" \
  python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag Bh \
  --shots bare,whole,bole > "$log" 2>&1
echo "B $age $seed $kind rc=$? $(date +%T)"

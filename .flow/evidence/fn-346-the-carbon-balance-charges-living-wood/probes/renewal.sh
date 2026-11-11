#!/usr/bin/env bash
# fn-346 renewal census, with probes/renewal.patch applied: the oak on
# seed 1001, grow only, every 10 years births by source, deaths by cause,
# living tips by row, sleeping buds held and the light at shed places.
# Stops at eight times the most today's oak had held, or 8 GiB.
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/renewal.sh <age> "<tag>/<letters>/<lmax>/<factor> ..."
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/renewal"
mkdir -p "$out"
age="$1"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $2; do
  IFS=/ read -r tag set lmax factor <<< "$run"
  [ "$set" = - ] && set=""
  [ "$lmax" = - ] && lmax=""
  [ "$factor" = - ] && factor=""
  env RENEW_LIFE=1000 ${lmax:+PIPES_LMAX=$lmax} ${factor:+PIPES_LMAX_FACTOR=$factor} \
    PROBE_YEAR=1 PROBE_BALANCE=1 PROBE_RENEWAL=1 ABLATE="$set" ABLATE_CENSUS=1 ABLATE_GROW_ONLY=1 \
    ABLATE_CURVE="$F/raw/curve-300-1001-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=8 \
    python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds 1001 --tag "$tag" \
    --shots bare > "$out/oak-$age-1001-$tag.log" 2>&1
  echo "renewal $tag $age rc=$? $(date +%T)"
done

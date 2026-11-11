#!/usr/bin/env bash
# fn-346 "the balance counts what would remain", with probes/remain.patch
# applied: the oak's census to an age, grow only, with the renewal census
# (births, deaths, living tips by row, sleeping buds) and the old-tree
# census (every limb and bough line's end); stops at eight times the most
# today's oak had held, or 8 GiB.
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/remain.sh <age> <seed> "<tag>/<letters> ..."
# Letters: PR is Q (with L_max 34.8 x 2); M the rule; S rule B; U the
# square charge. FACTOR sets L_max's factor (2 unset).
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/remain"
mkdir -p "$out"
age="$1"; seed="$2"
curve="$F/raw/curve-300-$seed-peak.txt"
[ -f "$curve" ] || curve="$F/raw/curve-300-1001-peak.txt"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $3; do
  IFS=/ read -r tag set <<< "$run"
  env PIPES_LMAX=34.8 PIPES_LMAX_FACTOR="${FACTOR:-2}" ABLATE="$set" ABLATE_CENSUS=1 ABLATE_GROW_ONLY=1 \
    PROBE_YEAR=1 PROBE_BALANCE=1 PROBE_RENEWAL=1 PROBE_OLDTREE=1 \
    ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=8 \
    python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare > "$out/oak-$age-$seed-$tag.log" 2>&1
  echo "remain $tag $age $seed rc=$? $(date +%T)"
done

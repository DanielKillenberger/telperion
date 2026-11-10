#!/usr/bin/env bash
# fn-345 R2/R3: today's tree (letters "-") beside the probe (letters "BN":
# dead wood not charged, and each zone's node count times
# K * max(0, 1 - L / LMAX), L the tip's path from the root collar), with
# probes/n.patch applied. Run from the repo root inside telperion-heavy:
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/n.sh "<runs>"
# runs: <species>/<letters>/<age>/<seed>/<tag>/<K>/<LMAX>/<grow|draw|census>
#   K and LMAX "-" leave them unset (no restating; rule dormant).
#   grow:   settled, not drawn, 10-year whole-tree record;
#   census: grow with the yearly census and 10-year record, stopped at
#           eight times the most today's oak had held (oak) or 20 GiB;
#   draw:   leafless, leafed and lower-trunk stills;
#   time:   grow only, pinned to one core, no yearly records: the
#           one-thread seconds.
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $1; do
  IFS=/ read -r sp set age seed tag k lmax mode <<< "$run"
  [ "$set" = - ] && set=""
  [ "$k" = - ] && k=""
  [ "$lmax" = - ] && lmax=""
  curve=""
  [ "$sp" = oak ] && curve="$F/raw/curve-300-$seed-peak.txt"
  [ -f "$curve" ] || curve=""
  only=1; log="$out/$sp-$age-$seed-$tag-$mode.log"
  [ "$mode" = draw ] && { only=""; log="$out/$sp-$age-$seed-$tag.log"; }
  pin=""; rec=1
  [ "$mode" = time ] && { pin="taskset -c 3"; rec=""; }
  env ${k:+ABLATE_NODE_K=$k} ${lmax:+ABLATE_LMAX=$lmax} ${rec:+PROBE_YEAR=1 PROBE_BALANCE=1} \
    ABLATE="$set" ABLATE_CENSUS=1 ABLATE_GROW_ONLY="$only" \
    ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=20 \
    $pin python3 "$PEAK" "target/ci/examples/space_$sp" "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare,whole,bole > "$log" 2>&1
  echo "$mode $sp $tag $age $seed rc=$? $(date +%T)"
done

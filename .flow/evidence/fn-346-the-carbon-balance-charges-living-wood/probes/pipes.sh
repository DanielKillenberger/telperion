#!/usr/bin/env bash
# fn-346 run: today's tree (letters "-") beside the probe, with
# probes/pipes.patch applied. Letters: P = the restated charge (a living
# tip costs size x path / LMAX, dead wood nothing, no per-metre upkeep);
# PR = P2, P with continuations and relays tested as laterals are.
# Run from the repo root inside telperion-heavy:
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/pipes.sh "<runs>"
# runs: <species>/<letters>/<age>/<seed>/<tag>/<LMAX>/<factor>/<mode>
#   LMAX "-" leaves the rule dormant; factor "-" is 1.
#   census: grow only, yearly census and the 10-year BAL record, stopped
#           at eight times the most today's oak had held, or 8 GiB;
#   draw:   leafless, leafed and lower-trunk stills (stop at 20 GiB);
#   time:   grow only, pinned to one core, no yearly records.
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $1; do
  IFS=/ read -r sp set age seed tag lmax factor mode <<< "$run"
  [ "$set" = - ] && set=""
  [ "$lmax" = - ] && lmax=""
  [ "$factor" = - ] && factor=""
  curve=""
  [ "$sp" = oak ] && curve="$F/raw/curve-300-$seed-peak.txt"
  [ -f "$curve" ] || curve=""
  only=1; rss=8; log="$out/$sp-$age-$seed-$tag-$mode.log"
  [ "$mode" = draw ] && { only=""; rss=20; log="$out/$sp-$age-$seed-$tag.log"; }
  pin=""; rec=1
  [ "$mode" = time ] && { pin="taskset -c 3"; rec=""; rss=20; }
  env ${lmax:+PIPES_LMAX=$lmax} ${factor:+PIPES_LMAX_FACTOR=$factor} ${rec:+PROBE_YEAR=1 PROBE_BALANCE=1} \
    ABLATE="$set" ABLATE_CENSUS=1 ABLATE_GROW_ONLY="$only" \
    ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=$rss \
    $pin python3 "$PEAK" "target/ci/examples/space_$sp" "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare,whole,bole > "$log" 2>&1
  echo "$mode $sp $tag $age $seed rc=$? $(date +%T)"
done

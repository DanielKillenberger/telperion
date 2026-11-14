#!/usr/bin/env bash
# fn-344 host follow-up 3: variant H (B plus every new internode shortened
# by max(0, 1 - L / 34.8 m), L the path from the root collar to its tip at
# the start of the year), and A and B again for their whole-tree records,
# each stopped at eight times the most A had held by that year (A's 300
# grow's curve for the seed) or 20 GiB resident:
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/h.sh grow "<runs>"
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/h.sh draw "<runs>"
# runs: <letters>/<age>/<seed>/<tag>, letters "-" for A. grow is not drawn
# (logs ...-<tag>-grow.log); draw makes leafless, leafed and lower-trunk
# stills (tag <tag>, logs ...-<tag>.log).
set -u
E=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $2; do
  IFS=/ read -r set age seed tag <<< "$run"
  [ "$set" = - ] && set=""
  if [ "$1" = grow ]; then only=1; log="$out/oak-$age-$seed-$tag-grow.log"; else only=""; log="$out/oak-$age-$seed-$tag.log"; fi
  ABLATE="$set" ABLATE_CENSUS=1 PROBE_YEAR=1 PROBE_BALANCE=1 ABLATE_GROW_ONLY="$only" \
    ABLATE_CURVE="$E/raw/curve-300-$seed-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=20 \
    python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare,whole,bole > "$log" 2>&1
  echo "$1 $tag $age $seed rc=$? $(date +%T)"
done

#!/usr/bin/env bash
# fn-346 the last measurement, with probes/last.patch applied (stays.patch,
# G never set): stills drawn with the settle's sag on or off.
#   telperion-heavy 22 --gpu 2 --threads 4 -- bash probes/last.sh "<runs>"
# runs: <species>/<letters>/<age>/<seed>/<tag>/<LMAX>/<sag: on|off>/<draw|census>
#   letters "-" is today's tree; LMAX "-" leaves the charge dormant; L_max
#   undoubled; sag off adds the example's --unsagged.
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for run in $1; do
  IFS=/ read -r sp set age seed tag lmax sag mode <<< "$run"
  [ "$set" = - ] && set=""
  [ "$lmax" = - ] && lmax=""
  flag=""; [ "$sag" = off ] && flag="--unsagged"
  curve="$F/raw/curve-300-1001-peak.txt"
  only=""; rss=20; log="$out/$sp-$age-$seed-$tag.log"; rec=""
  [ "$mode" = census ] && { only=1; rss=8; log="$out/$sp-$age-$seed-$tag-census.log"; rec=1; }
  env ${lmax:+PIPES_LMAX=$lmax} PIPES_LMAX_FACTOR=1 ${rec:+PROBE_YEAR=1 PROBE_BALANCE=1} ${PROBE_SCALES:+PROBE_SCALES=1} \
    ABLATE="$set" ABLATE_CENSUS=1 ABLATE_GROW_ONLY="$only" \
    ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=$rss \
    python3 "$PEAK" "target/ci/examples/space_$sp" "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare,whole,bole $flag > "$log" 2>&1
  echo "$mode $sp $tag $age $seed rc=$? $(date +%T)"
done
# Leaf-gap check (PROBE_SCALES): the settled phytomers by scale, grow only.
#   PROBE_SCALES=1 ... bash probes/last.sh "oak/<letters>/300/1001/<tag>/34.8/on/census"

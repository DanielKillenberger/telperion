#!/usr/bin/env bash
# fn-343 ablation: stills and measures of today's oak with one cause off
# (probes/ablate.patch applied; space_oak built with it), inside one GPU job:
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/stills.sh
# Variants: A today; B no limb/bough abortion; C no wander; D no sag; E branch
# and twig insertion halved; F branch never moves on; G = B, E and F.
set -u
E=.flow/evidence/fn-343-a-line-that-changes-hands-carries-on
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
for v in A B C D E F G; do
  case $v in A) set="" ;; G) set="BEF" ;; *) set="$v" ;; esac
  for run in "40 1001,1007" "120 1001"; do
    set -- $run
    ABLATE="$set" target/ci/examples/space_oak "$out" "$1" --seeds "$2" --tag "$v" \
      --shots bare,whole,limb,spray > "$out/oak-$1-$v.log" 2>&1
    echo "oak $v $1 rc=$?"
  done
done

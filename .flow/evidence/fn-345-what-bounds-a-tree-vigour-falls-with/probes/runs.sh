#!/usr/bin/env bash
# fn-345 R1, run inside `telperion-heavy 12 --threads 4` from the repo root,
# with fn-344's oldoak.patch and probes/stem.patch applied:
#   stem: A and H (BH) on seed 1001 at 120, grow-only, with the stem line's
#         phytomers and laterals below 10 m dumped (PROBE_STEM).
#   mem:  B at a 300-year request on seed 1001, grow-only, twice, exactly
#         as fn-344 follow-up 1 ran it (no PROBE_BALANCE), stopped at 6 GiB.
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
if [ "$1" = stem ]; then
  for v in A BH; do
    set=$([ "$v" = A ] && echo "" || echo "$v")
    ABLATE="$set" ABLATE_CENSUS=1 PROBE_YEAR=1 PROBE_STEM=1 ABLATE_GROW_ONLY=1 \
      ABLATE_CURVE="$F/raw/curve-300-1001-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=10 \
      python3 "$PEAK" target/ci/examples/space_oak "$out" 120 --seeds 1001 --tag "$v-stem" \
      --shots bare > "$out/oak-120-1001-$v-stem.log" 2>&1
    echo "stem $v rc=$? $(date +%T)"
  done
fi
if [ "$1" = mem ]; then
  for k in 1 2; do
    ABLATE=B ABLATE_CENSUS=1 PROBE_YEAR=1 ABLATE_GROW_ONLY=1 \
      ABLATE_CURVE="$F/raw/curve-300-1001-peak.txt" ABLATE_FACTOR=8 ABLATE_RSS_GIB=6 \
      python3 "$PEAK" target/ci/examples/space_oak "$out" 300 --seeds 1001 --tag "Bmem$k" \
      --shots bare > "$out/oak-300-1001-Bmem$k.log" 2>&1
    echo "mem $k rc=$? $(date +%T)"
  done
fi

#!/usr/bin/env bash
# fn-342 R2: leafless stills of today's oak (whole: `bare`; mid: `limb`)
# through the clean space_oak example (built from the unmodified tree,
# sha1 e75014a2dfe7416a1d372c390009acdce5991941), inside one GPU job:
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash <this file> <ages> <seeds>
set -u
E=.flow/evidence/fn-342-where-todays-oak-goes-wrong-to-the-eye
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/stills"
mkdir -p "$out"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
python3 "$PEAK" target/ci/examples/space_oak "$out" $1 --seeds "$2" --shots bare,limb \
  > "$out/oak-${1// /_}-$2.log" 2>&1
echo "oak $1 $2 rc=$?"

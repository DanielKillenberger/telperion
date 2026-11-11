#!/usr/bin/env bash
# fn-346 old-tree measurement on Q (the charge by size and path, L_max
# doubled, dead wood nothing, the main line tested), with
# probes/oldtree.patch applied. Grow only.
#   history: every limb and bough line's record and end, reach by height,
#            limbs lost and the framework born near them (to 300; stops at
#            eight times the most today's oak had held, or 8 GiB);
#   shape:   each living stem-line limb of the settled tree at each age,
#            sagged and unsagged (the sag rule off; growth never reads it).
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/oldtree.sh history <seed>
#   telperion-heavy 12 --gpu 2 --threads 4 -- bash probes/oldtree.sh shape <seed>
set -u
F=.flow/evidence/fn-344-the-old-oak-which-cause-carries-the
E=.flow/evidence/fn-346-the-carbon-balance-charges-living-wood
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
out="$E/raw/oldtree"
mkdir -p "$out"
mode="$1"; seed="$2"
curve="$F/raw/curve-300-$seed-peak.txt"
[ -f "$curve" ] || curve="$F/raw/curve-300-1001-peak.txt"
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
run() {  # age tag log extra-flags...
  local age="$1" tag="$2" log="$3"; shift 3
  env PIPES_LMAX=34.8 PIPES_LMAX_FACTOR=2 ABLATE=PR ABLATE_CENSUS=1 ABLATE_GROW_ONLY=1 \
    ABLATE_CURVE="$curve" ABLATE_FACTOR=8 ABLATE_RSS_GIB=8 "${ENVS[@]}" \
    python3 "$PEAK" target/ci/examples/space_oak "$out" "$age" --seeds "$seed" --tag "$tag" \
    --shots bare "$@" > "$log" 2>&1
  echo "$mode $tag $age $seed rc=$? $(date +%T)"
}
if [ "$mode" = history ]; then
  ENVS=(PROBE_YEAR=1 PROBE_BALANCE=1 PROBE_OLDTREE=1)
  run 300 Qold "$out/history-$seed.log"
fi
if [ "$mode" = shape ]; then
  ENVS=(PROBE_SHAPE=1)
  for age in 120 160 200 240 280; do
    run "$age" Qshape "$out/shape-$age-$seed.log"
    run "$age" Qflat "$out/shape-$age-$seed-unsagged.log" --unsagged
  done
fi

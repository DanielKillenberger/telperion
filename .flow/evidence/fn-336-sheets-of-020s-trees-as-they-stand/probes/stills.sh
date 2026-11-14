#!/usr/bin/env bash
# fn-336: every still the sheets hold, one tree per process so each tree's
# wall time and peak memory are its own (fn-330's peak.py). Full cycle: the
# presets set no /fidelity, so the grow is at fidelity 0 and the coarse rows
# stay dormant. Run inside one GPU job, serially:
#   telperion-heavy 10 --gpu 2 --threads 4 -- <this file>
# Writes raw/<species>/<species>-<age>-<seed>-<shot>.png and a log per tree.
set -u
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
E=.flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
grow() {  # <example> <out dir> <age> <seed> <shots>
  mkdir -p "$E/raw/$2"
  local log="$E/raw/$2/$1-$3-$4.log"
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && return 0  # kept from an earlier pass
  python3 "$PEAK" "target/ci/examples/$1" "$E/raw/$2" "$3" --seeds "$4" --shots "$5" > "$log" 2>&1
}
# R1: each real species at its preset age (40 for all six), seeds 1, 7, 42.
for example in space_oak space_spruce space_beech space_birch space_palm space_ordinary; do
  for seed in 1 7 42; do grow "$example" "${example#space_}" 40 "$seed" whole,crown,spray,low; done
done
# R2: the old oak, whole tree.
for age in 120 200 300; do
  for seed in 1 7 42; do grow space_oak oak-old "$age" "$seed" whole; done
done
# The hero pose cut off the trunk base of seed 7 at 200 and 300. The same tree
# (the leaf count matches the space example's) through the headless still,
# age laid over the preset, camera pulled back; same size and sun.
mkdir -p "$E/raw/remake"
for age in 200 300; do
  log="$E/raw/remake/oak-$age-7.log"
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && continue
  printf '{"age": %s}\n' "$age" > "$E/raw/remake/age$age.json"
  python3 "$PEAK" target/ci/examples/headless --preset oregon-white-oak --seed 7 \
    --family "$E/raw/remake/age$age.json" --size 960x720 --view whole \
    --scene '{"sunAzimuth":115,"sunElevation":60}' \
    --camera '{"fill":0.72,"targetHeight":0.45}' \
    --out "$E/raw/remake/oak-$age-7-whole.png" > "$log" 2>&1
done

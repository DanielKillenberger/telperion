#!/usr/bin/env bash
# fn-350: the fix branch (fn-349 at 210f97006, probes/kept.patch applied
# uncommitted) built and drawn in the fn-349 checkout, every PA's
# `retained` set by the still runner's existing `--girth retained,chi`
# flag (chi held at the preset's own leaf_girth, 1 on every PA of both the
# oak and the beech: tolerance.rs sets it on the preset, beech.rs's 0 is
# overwritten; checked by retained 0 reproducing fn-349's node counts).
# One tree per process, so a refusal ends only that tree.
#   bash run.sh beech   re-runs only the beech (its first run held chi 0).
#   bash run.sh build | draw | cost      (from anywhere)
set -u
B=/home/daniel/Projects/telperion/.worktrees/fn-349
E=/home/daniel/Projects/telperion/.worktrees/fn-319/.flow/evidence/fn-350-does-a-kept-pipe-hold-the-old-oak-up
OUT="$E/raw/stills"
PEAK=/home/daniel/Projects/telperion/.worktrees/fn-319/.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
cd "$B" || exit 1
one() {  # <species> <chi> <age> <seed> <retained>
  local sp="$1" chi="$2" age="$3" seed="$4" r="$5"
  local tag="R$r" shots=bare,whole
  # Retained 0 is the fix branch as it stands: its stills exist in fn-349's
  # raw stills; grow it for the measures only.
  [ "$r" = 0 ] && shots=none
  telperion-heavy 20 --gpu 2 --threads 8 -- python3 "$PEAK" \
    "target/ci/examples/space_$sp" "$OUT" "$age" --seeds "$seed" \
    --girth "$r,$chi" --tag "$tag" --shots "$shots" \
    > "$OUT/$sp-$age-$seed-$tag.log" 2>&1
  echo "draw $sp $age $seed $tag rc=$? $(TZ=Europe/Zurich date +%T)"
}
case "$1" in
  build)
    telperion-heavy 8 --threads 8 -- cargo build --profile ci \
      -p telperion-render --example space_oak --example space_beech \
      -p telperion-core --example early > "$E/raw/build.log" 2>&1
    echo "build rc=$?"
    ;;
  draw)
    for r in ${2:-0 0.5 1}; do
      for seed in 1001 1007 1012; do
        for age in 120 200 300; do one oak 1 "$age" "$seed" "$r"; done
      done
      for age in 40 80; do one oak 1 "$age" 1001 "$r"; done
      one beech 1 40 1001 "$r"
    done
    ;;
  beech)
    for r in 0 0.5 1; do one beech 1 40 1001 "$r"; done
    ;;
  cost)
    # One-thread seconds and peak memory, the oak at 300 on seed 1001, as
    # fn-349's cost.sh took them (`early live`, pinned to one core).
    for r in 0 0.5 1; do
      line=$(PROBE_RETAINED="$r" telperion-heavy 12 --threads 1 -- taskset -c 3 python3 "$PEAK" \
        target/ci/examples/early live oregon-white-oak 300 1001 2>&1 | grep '^PEAK\|^PROBE')
      echo "COST oak 300 1001 R$r $line" | tr '\n' ' '
      echo
    done
    ;;
esac

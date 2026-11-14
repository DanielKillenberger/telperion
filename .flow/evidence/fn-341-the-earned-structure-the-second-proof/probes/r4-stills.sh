#!/usr/bin/env bash
# fn-341 R4: the stills, the cycle and the probe (G and P; U left out), one tree per
# process through fn-330's peak.py, with probes/earned.patch applied and the
# space examples built. Run inside one GPU job:
#   telperion-heavy <GB> --gpu 2 --threads 4 -- bash <this file> <part>
# Parts: oak, oak300, beech, others. Writes raw/stills/<mode>/.
set -u
E=.flow/evidence/fn-341-the-earned-structure-the-second-proof
PEAK=.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/probes/peak.py
OAK="10:0.23:25:0.37:0:16:100:20.4:200:25.9"
BEECH="0:0.331:1:0.331:0:70:0:70:0:70"
still() {  # <mode> <example> <rings or -> <ages> <seeds> <shots>
  local out="$E/raw/stills/$1"
  mkdir -p "$out"
  unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
  if [ "$1" = probe ]; then
    [ "$3" != - ] && export TELPERION_RINGS="$3"
    export TELPERION_POSTURE=1241  # U is left out (R3-ANSWERS.md)
  fi
  python3 "$PEAK" "target/ci/examples/$2" "$out" $4 --seeds "$5" --shots "$6" > "$out/$2-${4// /_}-$5.log" 2>&1
  echo "$1 $2 $4 $5 rc=$?"
}
case "$1" in
oak) for m in cycle probe; do still $m space_oak "$OAK" "40 120" 1001,1007 whole,crown; done ;;
oak300) for m in cycle probe; do for s in 1001 1007; do still $m space_oak "$OAK" 300 $s whole,crown; done; done ;;
beech) for m in cycle probe; do still $m space_beech "$BEECH" 40 1001,1007 whole,crown; done ;;
others)
  # No checked R and N for these: the cycle's girth under P.
  for ex in space_spruce space_birch space_palm space_ordinary; do
    for m in cycle probe; do still $m $ex - 40 1001 whole; done
  done ;;
esac

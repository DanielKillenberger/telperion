#!/usr/bin/env bash
# fn-341 R3: the earned rules run alone, then added, through the R1 probe
# (`structure_measures`), with probes/earned.patch applied and the core test
# binary built. Each rule is switched by the environment the space crate
# reads at every grow (girth/rings.rs, `from_env`):
#   bash .flow/evidence/fn-341-the-earned-structure-the-second-proof/probes/r3-runs.sh <bin> <preset> <age> <seeds> <rules: cycle|G|GP|GPU|GU> [tag]
set -u
B="$1"; P="$2"; A="$3"; S="$4"; RULES="$5"; TAG="${6:-}"
R=.flow/evidence/fn-341-the-earned-structure-the-second-proof/raw
# Traits (R3-G-PAPER.md): the oak's R from Phillips' two groves by height and
# N from European oak by age; the beech's R Bartelink's, N Gebauer's dye.
case "$P" in
  oregon-white-oak) RINGS="10:0.23:25:0.37:0:16:100:20.4:200:25.9" ;;
  european-beech) RINGS="0:0.331:1:0.331:0:70:0:70:0:70" ;;
  *) RINGS="${RINGS_OVERRIDE:-}" ;;
esac
# Eq. 39's coefficient from Alméras and Fournier's Fig. 8 angiosperm means.
POSTURE=1241
unset TELPERION_RINGS TELPERION_POSTURE TELPERION_LIVING
case "$RULES" in *G*) [ -n "$RINGS" ] && export TELPERION_RINGS="$RINGS" ;; esac
case "$RULES" in *P*) export TELPERION_POSTURE=$POSTURE ;; esac
case "$RULES" in *U*) export TELPERION_LIVING=1 ;; esac
STRUCT_PRESET="$P" STRUCT_AGE="$A" STRUCT_SEEDS="$S" \
  telperion-heavy 8 --threads 4 -- "$B" structure_measures --ignored --nocapture > "$R/r3-$P-$A-$RULES$TAG.log" 2>&1
echo "$P $A $RULES rc=$?"

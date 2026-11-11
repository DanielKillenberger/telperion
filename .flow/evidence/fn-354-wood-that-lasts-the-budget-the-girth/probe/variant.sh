#!/usr/bin/env bash
# fn-354 probe: the environment a variant grows a preset under, for the
# scoreboard's TREE_ENV option. The values are PROBE-BRIEF.md's, unchanged.
#
#   variant.sh <B|G1|G2|SKETCH> [trace] <preset>
#
# B prints nothing (today's girth and sag). G1: the rule, today's sag. G2:
# the rule, elastic sag. SKETCH: today's girth with the last cycle's rough
# layout grown (the check that it leaves the tree as it was). `trace` adds
# the yearly RING lines.
set -eu
variant="$1"
shift
trace=""
if [ "$1" = trace ]; then
  trace="TELPERION_RING_TRACE=1"
  shift
fi
preset="$1"

# R (m² leaf per cm² sapwood), N (sapwood rings), leaf years; 4ρg/E per metre.
case "$preset" in
  oregon-white-oak) rule="0.30:20:1" sag="5.0e-6" ;;
  european-beech) rule="0.331:50:1" sag="4.1e-6" ;;
  norway-spruce) rule="0.36:30:6" sag="6.1e-6" ;;
  silver-birch) rule="0.331:40:1" sag="3.7e-6" ;;
  *) echo "variant.sh: no values for $preset" >&2; exit 2 ;;
esac

case "$variant" in
  B) ;;
  SKETCH) echo "TELPERION_LAST_SKETCH=1" ;;
  G1) echo "TELPERION_GIRTH=$rule $trace" ;;
  G2) echo "TELPERION_GIRTH=$rule TELPERION_SAG=$sag $trace" ;;
  *) echo "variant.sh: no variant $variant" >&2; exit 2 ;;
esac

#!/usr/bin/env bash
# argv: out.jsonl seed [new-only]; every preset: old at its default, new at its
# default and new at master's default age.
cd /tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/fn258
OUT=$1; SEED=$2; ONLY=$3
declare -A OLDAGE=([european-beech]=120 [oregon-white-oak]=100 [norway-spruce]=100 [silver-birch]=70 [date-palm]=100 [ordinary]=100 [laurelin]=100 [telperion]=100)
for sp in date-palm ordinary norway-spruce silver-birch oregon-white-oak european-beech laurelin telperion; do
  [ -z "$ONLY" ] && ./run.sh old "$OUT" $sp $SEED
  ./run.sh new "$OUT" $sp $SEED
  ./run.sh new "$OUT" $sp $SEED --age ${OLDAGE[$sp]}
done
echo sweep-done

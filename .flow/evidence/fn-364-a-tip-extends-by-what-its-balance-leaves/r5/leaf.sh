#!/bin/bash
# fn-364 R5: the drawn stations of every R5 tree, fn-363's branch and this
# one's final station rule (f6c00cc49), each runner in a target of its own.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
export TELPERION_HEAVY_RESERVE=8
for side in 363 leaf364; do
  r=$D/runner$side; [ $side = leaf364 ] && r=$D/runner-leaf364
  name=leaf${side#leaf}
  cd "$r" || exit 1
  CARGO_TARGET_DIR=$D/target-$name telperion-heavy 8 --threads 4 -- cargo build --profile ci >> "$D/build-$name.log" 2>&1
  echo "build $name rc=$? $(date '+%F %T')" >> "$D/times.txt"
  cp "$D/target-$name/ci/heldrun" "$D/bin/$name" || exit 1
done
young=""
for p in oregon-white-oak ordinary norway-spruce european-beech silver-birch date-palm laurelin telperion; do
  ages="40 80"
  [ $p = telperion ] && ages="40 80 82"
  for a in $ages; do for s in 1 7; do young="$young $p:$a:$s:4"; done; done
done
old=""
for a in 120 200 300; do for s in 1001 1007 1012; do old="$old oregon-white-oak:$a:$s:12"; done; done
for side in leaf364 leaf363; do bash $D/runs.sh "$young" $side; done
for side in leaf364 leaf363; do bash $D/runs.sh "$old" $side; done
echo "LEAF DONE $(date '+%F %T')" >> $D/times.txt

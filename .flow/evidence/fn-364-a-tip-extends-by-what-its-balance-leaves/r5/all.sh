#!/bin/bash
# fn-364 R5: every tree fn-363's R5 grows, on this branch, then fn-363's
# station plan for the same trees: the young presets first, then the old oak.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
young=""
for p in oregon-white-oak ordinary norway-spruce european-beech silver-birch date-palm laurelin telperion; do
  ages="40 80"
  [ $p = telperion ] && ages="40 80 82"
  for a in $ages; do gb=4; [ $p = oregon-white-oak ] && [ $a = 80 ] && gb=6; for s in 1 7; do young="$young $p:$a:$s:$gb"; done; done
done
bash $D/runs.sh "$young" this
old=""
for a in 120 200 300; do for s in 1001 1007 1012; do gb=12; [ $a = 120 ] && gb=8; old="$old oregon-white-oak:$a:$s:$gb"; done; done
bash $D/runs.sh "$old" this
echo "ALL DONE $(date '+%F %T')" >> $D/times.txt

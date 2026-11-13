#!/bin/bash
# fn-363 R5: every tree before and after, the other presets first.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn363
young=""
for p in ordinary oregon-white-oak norway-spruce european-beech silver-birch date-palm laurelin telperion; do
  ages="40 80"
  [ $p = telperion ] && ages="40 80 82"
  for a in $ages; do gb=4; [ $p = oregon-white-oak ] && [ $a = 80 ] && gb=6; for s in 1 7; do young="$young $p:$a:$s:$gb"; done; done
done
for side in before after; do bash $D/runs.sh $side "$young"; done
old=""
for a in 120 200 300; do for s in 1001 1007 1012; do gb=24; [ $a = 120 ] && gb=12; old="$old oregon-white-oak:$a:$s:$gb"; done; done
for side in before after; do bash $D/runs.sh $side "$old"; done
echo "ALL DONE $(date '+%F %T')" >> $D/times.txt

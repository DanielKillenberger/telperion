#!/bin/bash
# trial.sh <tag> : trunk close-up for today's oak and spruce with the current build, against full quadrature
D=/home/daniel/Projects/telperion/.worktrees/prod/.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/raw/shade
L=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/gpu.lock
cp /home/daniel/Projects/telperion/.worktrees/prod/target/release/examples/f3_shade $D/f3_$1
mkdir -p $D/$1
for t in "today oak 1" "today spruce 1"; do F3_VIEW=${VIEW:-trunk} flock $L $D/f3_$1 $t $D/$1 4 2>&1; done
for t in today-oak today-spruce; do for v in ${VIEW:-trunk}; do
 ref=$D/trunk_old/$t-1-$v-s4.png; [ "$v" != trunk ] && ref=$D/prepass/$t-1-$v-s4.png
 echo "$t $v vs full detail: $(compare -metric RMSE $ref $D/$1/$t-1-$v-s4.png null: 2>&1)"
 montage -label 'full quadrature' $ref -label "$1" $D/$1/$t-1-$v-s4.png -tile 2x1 -geometry +4+4 -pointsize 18 $D/sheets/$t-$v-$1.png
done; done

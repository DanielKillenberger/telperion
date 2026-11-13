#!/bin/bash
# fn-363 R5: grower and settle only, one tree a process, one at a time.
#   runs.sh <before|after> "<preset>:<age>:<seed>:<GB> ..."
# A tree whose log already ends EXIT 0 is kept.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad
O=$D/fn363
mkdir -p "$O/logs"
export TELPERION_HEAVY_RESERVE=8 OLDOAK=1 SURPLUS=1
side=$1
for job in $2; do
  IFS=: read -r preset age seed gb <<< "$job"
  name=$side-$preset-a$age-s$seed
  grep -q "EXIT 0" "$O/logs/$name.out" 2>/dev/null && continue
  echo "$name start $(date '+%F %T')" >> "$O/times.txt"
  telperion-heavy "$gb" --threads 4 -- python3 "$D/peak.py" "$O/bin/heldrun-$side" "$preset" "$age" "$seed" > "$O/logs/$name.out" 2> "$O/logs/$name.log"
  echo "$name end rc=$? $(date '+%F %T')" >> "$O/times.txt"
done
echo "$side batch done $(date '+%F %T')" >> "$O/times.txt"

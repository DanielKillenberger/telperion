#!/bin/bash
# fn-364 R5: one tree a process, one at a time, through the queue.
#   runs.sh "<preset>:<age>:<seed>:<GB> ..." [this|leaf363]
# this: this branch's grower and settle, then its station plan (LEAFCHECK).
# leaf363: fn-363's station plan alone (its grower rows are its builder's,
# scratchpad/fn363/logs/after-*, never grown twice).
# A tree whose log already ends EXIT 0 is kept.
D=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad
O=$D/fn364
mkdir -p "$O/logs"
export TELPERION_HEAVY_RESERVE=8 OLDOAK=1 SURPLUS=1
side=${2:-this}
bin=heldrun
[ "$side" = leaf363 ] && bin=leaf363
[ "$side" = leaf364 ] && bin=leaf364
[ "$side" = r8 ] && bin=heldrun-r8
[ "$side" = leafr8 ] && bin=leafr8
for job in $1; do
  IFS=: read -r preset age seed gb <<< "$job"
  name=$side-$preset-a$age-s$seed
  grep -q "EXIT 0" "$O/logs/$name.out" 2>/dev/null && continue
  echo "$name start $(date '+%F %T')" >> "$O/times.txt"
  telperion-heavy "$gb" --threads 4 -- python3 "$D/peak.py" "$O/bin/$bin" "$preset" "$age" "$seed" > "$O/logs/$name.out" 2> "$O/logs/$name.log"
  echo "$name end rc=$? $(date '+%F %T')" >> "$O/times.txt"
done
echo "batch $side done $(date '+%F %T')" >> "$O/times.txt"

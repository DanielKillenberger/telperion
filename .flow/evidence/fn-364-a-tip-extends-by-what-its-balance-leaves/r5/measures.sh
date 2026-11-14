#!/bin/bash
# fn-364 R5: MEASURES.md from r5/head.md and the table as the logs stand.
E=/home/daniel/Projects/telperion/.worktrees/fn-364/.flow/evidence/fn-364-a-tip-extends-by-what-its-balance-leaves
S=/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/fn364
python3 -I "$S/tables.py" "$S/table-now.md" > /dev/null
cat "$E/r5/head.md" "$S/table-now.md" > "$S/measures-new.md"
cp "$S/measures-new.md" "$E/MEASURES.md"

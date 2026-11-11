#!/usr/bin/env bash
# fn-352 R1: the scoreboard. Draws and measures the fixed panel on the
# checkout this file sits in, and writes scoreboard/<label>/ (RUN.md, the
# measures, the cost, TABLES.md, sheets/*.jpg); raw stills and logs go to the
# ignored raw/<label>/.
#
#   bash run.sh <label> [stage...]
#
# Stages, in order when none is named: build panel cost wasm sheets tables.
#   build   the probe (probe/, a crate outside the workspace) into raw/target
#   panel   every tree of the panel, one process each, inside ONE heavy job
#   cost    native one-thread seconds and peak memory; a thousand oaks at 40
#   wasm    the package's Wasm times by fn-329's protocol (npm run build)
#   sheets  the JPEG sheets from the panel's stills
#   tables  TABLES.md from the measures
# SMOKE=1 draws one seed (1001) of each tree, without the oak's other seeds.
# Heavy jobs run one at a time through telperion-heavy; nothing here starts
# two. A tree already measured under this label is kept (its log ends
# `exit 0`), so a stopped run resumes.
set -u
S="$(cd "$(dirname "$0")" && pwd)"
E="$(dirname "$S")"
ROOT="$(git -C "$S" rev-parse --show-toplevel)"
LABEL="${1:?usage: run.sh <label> [stage...]}"
shift
STAGES="${*:-build panel cost wasm sheets tables}"
OUT="$S/$LABEL"
RAW="$E/raw/$LABEL"
PROBE="$E/raw/target/ci/scoreboard-probe"
mkdir -p "$OUT" "$RAW/stills"
cd "$ROOT" || exit 1

# The panel (host's design, fn-352 R2): every shipped preset at its preset
# age, and the oak at 40, 80, 120, 200 and 300.
PRESETS="oregon-white-oak norway-spruce european-beech silver-birch date-palm ordinary laurelin telperion"
OAK_AGES="40 80 120 200 300"
SHEET_SEEDS="1001 1007 1012"
MEASURE_SEEDS="1001 1002 1003 1004 1005 1006 1007 1008 1009 1010 1011 1012"
if [ "${SMOKE:-0}" = 1 ]; then SHEET_SEEDS=1001; MEASURE_SEEDS=1001; OAK_AGES=""; fi

state() {  # the machine as the run found it
  echo "- $(TZ=Europe/Zurich date '+%Y-%m-%d %H:%M %Z') $1: load $(cut -d' ' -f1-3 /proc/loadavg);" \
    "memory available $(free -g | awk '/^Mem:/{print $7}') of $(free -g | awk '/^Mem:/{print $2}') GiB;" \
    "GPU $(nvidia-smi --query-gpu=memory.used,memory.total --format=csv,noheader 2>/dev/null | tr -d ' ' || echo none) used;" \
    "heavy jobs running: $(systemctl --user list-units --plain --no-legend 'telperion-heavy-*' | wc -l)"
}

header() {
  [ -s "$OUT/RUN.md" ] && return
  {
    echo "# Scoreboard run \`$LABEL\`"
    echo
    echo "- Commit: \`$(git rev-parse HEAD)\` on \`$(git rev-parse --abbrev-ref HEAD)\`$(git diff --quiet HEAD -- crates || echo ', crates DIRTY')"
    echo "- Machine: $(lscpu | sed -n 's/^Model name: *//p'), $(nproc) threads, $(free -g | awk '/^Mem:/{print $2}') GiB, $(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null || echo 'no GPU')"
    echo "- Seeds: sheets $SHEET_SEEDS, and 7 for the oak; measures $MEASURE_SEEDS"
    echo "- Smoke: ${SMOKE:-0}"
    echo
    echo "## Machine load"
    echo
  } > "$OUT/RUN.md"
}

build() {
  cp Cargo.lock "$S/probe/Cargo.lock"
  telperion-heavy 8 --threads 16 -- cargo build --profile ci \
    --manifest-path "$S/probe/Cargo.toml" --target-dir "$E/raw/target" \
    > "$RAW/build.log" 2>&1
  local rc=$?
  echo "build rc=$rc"
  [ $rc = 0 ] || { tail -30 "$RAW/build.log"; exit 1; }
}

one() {  # <preset> <age|preset> <seed> <shots>: one tree, its own process
  local name="$1-$2-$3" log
  log="$RAW/stills/$name.log"
  [ -s "$log" ] && grep -q '^PEAK .* exit 0$' "$log" && return 0
  python3 "$S/peak.py" "$PROBE" tree "$1" "$2" "$3" "$RAW/stills/$name" "$4" > "$log" 2>&1
  echo "$name rc=$? $(grep -o '"height":[0-9.]*\|"refused":.\{0,60\}' "$log" | head -1) $(TZ=Europe/Zurich date +%T)"
}

# Inside the heavy job: every tree, serially.
_panel() {
  for p in $PRESETS; do
    for seed in $MEASURE_SEEDS; do
      shots=none
      case " $SHEET_SEEDS " in *" $seed "*) shots=whole,crown,spray,low ;; esac
      one "$p" preset "$seed" "$shots"
    done
  done
  for age in $OAK_AGES; do
    for seed in 7 $MEASURE_SEEDS; do
      shots=none
      case " 7 $SHEET_SEEDS " in *" $seed "*) shots=whole,bare,crown,spray,low ;; esac
      one oregon-white-oak "$age" "$seed" "$shots"
    done
  done
  [ "${SMOKE:-0}" = 1 ] || one oregon-white-oak preset 7 whole,crown,spray,low
}

panel() {
  echo "$(state 'panel starts')" >> "$OUT/RUN.md"
  # 16 GiB covers the old oak's two grows one after the other (fn-345: 5 GiB peak).
  SMOKE="${SMOKE:-0}" telperion-heavy 16 --gpu 2 --threads 24 -- bash "$S/run.sh" "$LABEL" _panel \
    | tee "$RAW/panel.log"
  echo "$(state 'panel ends')" >> "$OUT/RUN.md"
  grep -h '^TREE ' "$RAW"/stills/*.log | sed 's/^TREE //' > "$OUT/measures.jsonl"
  grep -h '^PEAK ' /dev/null "$RAW"/stills/*.log | wc -l
}

cost() {
  echo "$(state 'cost starts')" >> "$OUT/RUN.md"
  : > "$OUT/cost.jsonl"
  # One thread, pinned to one core: every preset at its preset age, the oak
  # at 40, 120 and 300; seed 1001; grow, expand and dress, as one process.
  for t in $(for p in $PRESETS; do echo "$p:preset"; done) oregon-white-oak:40 oregon-white-oak:120 oregon-white-oak:300; do
    local p="${t%%:*}" a="${t##*:}"
    line=$(telperion-heavy 12 --threads 1 -- taskset -c 3 python3 "$S/peak.py" "$PROBE" cost "$p" "$a" 1001 2>&1)
    json=$(echo "$line" | sed -n 's/^COST //p')
    peak=$(echo "$line" | sed -n 's/^PEAK seconds \([0-9.]*\) rss_gib \([0-9.]*\) exit \([0-9]*\)/"process_seconds":\1,"rss_gib":\2,"exit":\3/p')
    echo "${json%\}},$peak}" | tee -a "$OUT/cost.jsonl"
  done
  # A thousand distinct oaks at 40 across every core.
  [ "${SMOKE:-0}" = 1 ] && n=32 || n=1000
  line=$(telperion-heavy 24 --threads "$(nproc)" -- python3 "$S/peak.py" "$PROBE" many oregon-white-oak 40 1 "$n" 2>&1)
  json=$(echo "$line" | sed -n 's/^MANY //p')
  peak=$(echo "$line" | sed -n 's/^PEAK seconds \([0-9.]*\) rss_gib \([0-9.]*\) exit \([0-9]*\)/"process_seconds":\1,"rss_gib":\2,"exit":\3/p')
  echo "${json%\}},$peak}" | tee "$OUT/many.json"
  echo "$(state 'cost ends')" >> "$OUT/RUN.md"
}

wasm() {
  echo "$(state 'wasm starts')" >> "$OUT/RUN.md"
  telperion-heavy 8 --threads 8 -- npm run build > "$RAW/npm-build.log" 2>&1 \
    || { echo "npm run build failed"; tail -20 "$RAW/npm-build.log"; return 1; }
  telperion-heavy 4 --threads 1 -- taskset -c 10 node "$S/wasm.mjs" "$ROOT/dist" 0,1 \
    | tee "$OUT/wasm.jsonl"
  echo "$(state 'wasm ends')" >> "$OUT/RUN.md"
}

sheets() { python3 -I "$S/board.py" sheets "$OUT" "$RAW/stills"; }
tables() { python3 -I "$S/board.py" tables "$OUT"; }

if [ "$STAGES" = _panel ]; then _panel; exit 0; fi
header
for stage in $STAGES; do
  case "$stage" in
    build | panel | cost | wasm | sheets | tables) "$stage" ;;
    *) echo "no stage $stage"; exit 2 ;;
  esac
done

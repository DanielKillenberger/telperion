# drawcost.sh <label> row=value ... : S-WHOLE-sized whole view, triangles drawn
cd /home/daniel/Projects/telperion/.worktrees/fn-34-integration
label=$1; shift
python3 - "$@" <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-daniel-Projects-telperion/2ea2e05b-73d5-4017-a7cb-770a299a821a/scratchpad/jev')
from leaf_trial import patch, SPECIES
rows = dict(a.split('=', 1) for a in sys.argv[1:])
import pathlib; pathlib.Path('/tmp/claude-1000/-home-daniel-Projects-telperion/2ea2e05b-73d5-4017-a7cb-770a299a821a/scratchpad/jev/species.bak').write_text(SPECIES.read_text())
if rows: SPECIES.write_text(patch(SPECIES.read_text(), "pub(super) fn silver_birch(", rows, within="    p.element = ElementParams {"))
PY
cargo build --release -p telperion-render --example headless -q 2>/dev/null
out=$(target/release/examples/headless --preset silver-birch --seed 1 --view whole --size 1378x1440 --out /tmp/claude-1000/-home-daniel-Projects-telperion/2ea2e05b-73d5-4017-a7cb-770a299a821a/scratchpad/jev/dc.png 2>&1 | sed 's/.*foliage instances; //')
cp /tmp/claude-1000/-home-daniel-Projects-telperion/2ea2e05b-73d5-4017-a7cb-770a299a821a/scratchpad/jev/species.bak crates/telperion-core/src/presets/species.rs
echo "$label: $out"

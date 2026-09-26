# The recorded beech run (fn-149 R6, fn-157 R1 and R5)

`tests/replay.rs` replays this recording from the bare seed through Start
with no network and no key, and through Tune's first revision on a machine
with a hardware GPU (`max_rounds` 1: the baseline and one round; host,
2026-09-26). It is the runner's proof, recorded live on 2026-09-26 from the
beech's native range (fn-157).

- `seed/manifest.json`: the bare seed, no source; `taxon.native_range` is
  Europe, with the German and French names (host decision, 2026-09-26).
- `seed/packet/capability.json`: the host's capability assessment.
- `tuning.json`: the tuning config the recording ran with; the test points
  its run paths (`profiles`, the ledgers) at a scratch directory.
- `tape/`: every external answer a replay asks for, keyed by a stable hash
  of its request (`crate::tape`, `scripts/tape-adapter.py`). Recorded live
  on 2026-09-26 (`species european-beech --record`) and extended the same
  day (`--extend`, no Firecrawl call) for the agreement rule, the chosen
  shot and Tune's first revision: 37 Firecrawl answers (the searches, the
  Wikipedia lead, a scrape of each gathered document and the EUFORGEN
  guideline's PDF parse), 176 Jev calls, 19 Wikimedia Commons answers and
  images, and 8 vision adapter answers (the screen, the inventory, the
  shot's two looks, the baseline's comparisons and round 1's comparison and
  sheet). Tune's first round was extended again on the parameter catalogue's
  dial table (fn-152), whose order batches the proposal questions
  differently: 8 Jev calls and one sheet, recorded 2026-09-26. Every answer
  is refiled under today's keys, the newest kept where two became one
  question (`tape_trim`, `tape-adapter.py rekey:`). A replay opens every
  file here.

The repository is public, so a page that is not openly licensed keeps only
what the run quoted from it (owner, 2026-09-25): the passages that reached a
Jev request, joined by a lone full stop, and as bytes only its licence tags
and statements (`tape_trim`, `telperion_jev::tape::trim`; the replay test
`the_recordings_keep_only_the_passages_the_runs_quoted` fails on anything
more). A page no stage read keeps nothing, and a replay drops it as
empty. A PDF keeps its magic bytes, and its parse only the quoted passages.
Search results keep their title, address and snippet. The Wikipedia
article "Fagus sylvatica" stays whole, as its licence allows (Wikipedia
contributors, CC BY-SA 4.0, read as a lead for its references), and the
Commons photographs keep their files (each under its own open licence; the
Commons answer beside it names author and licence).

To record again (a pipeline change that moves a request):

```sh
bash -ic 'target/release/species european-beech --record <dir>/tape \
  --catalogue <dir>/catalogue --run-dir <dir>/run --tuning <dir>/tuning.json --until start'
```

from a folder holding this seed and assessment, then trim the recording
before it is committed and replace `tape/`:

```sh
cargo run -p telperion-jev --bin tape_trim -- <dir>/tape
cargo run -p telperion-jev --bin tape_trim -- <dir>/tape --check
```

Keep only what a replay opens: replay the trimmed tape through Start and
copy the files it read, for example by watching the tape with
`inotifywait -m -r -e open` during the replay.

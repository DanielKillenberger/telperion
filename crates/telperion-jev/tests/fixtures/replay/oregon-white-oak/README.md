# The recorded Oregon white oak run (fn-157 R5)

`tests/replay.rs` replays this recording from the bare seed through Start
with no network and no key, and checks that every value the aggregate
settled lands within `catalogue/oregon-white-oak`'s profile ranges.

- `seed/manifest.json`: the bare seed, no source, shaped as the beech's;
  `taxon.native_range` is western North America, British Columbia to
  California, with the Canadian name Garry oak.
- `seed/packet/capability.json`: the host's capability assessment of
  2026-09-26 (woody axes, a lobed blade, an alternate petiole; all
  expressed).
- `tuning.json`: the tuning config the recording ran with, the beech's with
  the oak's names; the test points its run paths at a scratch directory.
- `tape/`: recorded live on 2026-09-26 (`species oregon-white-oak
  --record`): 33 Firecrawl answers (the searches, the Wikipedia lead, a
  scrape of each gathered document and the forstpraxis yield table's PDF
  parse), 95 Jev calls, 3 Wikimedia Commons answers (the Commons categories
  file no single oak, so no photograph was kept).

The repository is public, so a page that is not openly licensed keeps only
what the run quoted from it (owner, 2026-09-25): the passages that reached a
Jev request, joined by a lone full stop, and as bytes only its licence tags
and statements (`tape_trim`, `telperion_jev::tape::trim`; the replay test
`the_recordings_keep_only_the_passages_the_runs_quoted` fails on anything
more). A page no stage read keeps nothing, and a replay drops it as
empty. A PDF keeps its magic bytes, and its parse only the quoted passages.
Search results keep their title, address and snippet. The Wikipedia
article "Quercus garryana" stays whole, as its licence allows (Wikipedia
contributors, CC BY-SA 4.0, read as a lead for its references), and the
Commons photographs keep their files (each under its own open licence; the
Commons answer beside it names author and licence).

To record again (a pipeline change that moves a request):

```sh
bash -ic 'target/release/species oregon-white-oak --record <dir>/tape \
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

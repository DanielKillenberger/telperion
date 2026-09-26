# The recorded beech run (fn-149 R6, fn-157 R5)

`tests/replay.rs` replays this recording from the bare seed through Start
with no network and no key. It is the runner's proof, recorded live on
2026-09-26 through gather, read and aggregate (fn-157).

- `seed/manifest.json`: the bare seed, no source.
- `seed/packet/capability.json`: the host's capability assessment.
- `tuning.json`: the tuning config the recording ran with; the test points
  its run paths (`profiles`, the ledgers) at a scratch directory.
- `tape/`: every external answer a replay asks for, as the live run of
  2026-09-26 received it (`species european-beech --record`), keyed by a
  stable hash of its request (`crate::tape`, `scripts/tape-adapter.py`): 23
  Firecrawl answers (six searches, the Wikipedia lead and a scrape of each of
  the 16 gathered documents; two were refused for want of Firecrawl credits
  and stay dropped),
  232 Jev calls (48 span labels, 184 appearance levels), 17 Wikimedia
  Commons answers and images, 2 vision adapter replies. Answers the live run
  received that a replay never asks for (a first photograph search, the
  label calls the zero-span rule no longer makes) were left out, so a replay
  opens every file here.

The repository is public, so a page that is not openly licensed keeps only
what the run quoted from it (owner, 2026-09-25): the passages that reached a
Jev request, joined by a lone full stop, and as bytes only its licence tags
and statements (`tape_trim`, `telperion_jev::tape::trim`; the replay test
`the_recording_keeps_only_the_passages_the_run_quoted` fails on anything
more). A page nothing was quoted from keeps nothing, and a replay drops it as
empty. Search results keep their title, address and snippet. The Wikipedia
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

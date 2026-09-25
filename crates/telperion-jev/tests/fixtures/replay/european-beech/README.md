# The recorded beech run (fn-149 R6)

`tests/replay.rs` replays this recording from the bare seed through Start
with no network and no key. It is the runner's proof: the machinery, not the
beech's values, which fn-157 replaces.

- `seed/manifest.json`: the bare seed, no source admitted.
- `seed/packet/capability.json`: the host's capability assessment.
- `tuning.json`: the tuning config the recording ran with; the test points
  its run paths (`profiles`, the ledgers) at a scratch directory.
- `tape/`: every external answer the live run of 2026-09-25 received
  (`species european-beech --record`), keyed by a stable hash of its request
  (`crate::tape`, `scripts/tape-adapter.py`): 57 Firecrawl answers, 158 Jev
  calls, 28 Wikimedia Commons answers and images, 2 vision adapter replies.
  A replay opens every file here.

The repository is public, so a page that is not openly licensed keeps only
what the run quoted from it (owner, 2026-09-25): the passages that reached a
Jev request, joined by a lone full stop, and as bytes only its licence tags
and statements (`tape_trim`, `telperion_jev::tape::trim`; the replay test
`the_recording_keeps_only_the_passages_the_run_quoted` fails on anything
more). Search results keep their title, address and snippet. Two sources stay
whole, as their licences allow: the Wikipedia article "Fagus sylvatica"
(Wikipedia contributors, CC BY-SA 4.0, read as a lead for its references) and
the DOAJ record, and the Commons photographs keep their files (each under its
own open licence; the Commons answer beside it names author and licence).

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

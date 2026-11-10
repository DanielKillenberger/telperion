# fn-344 friction

## 10 Nov 2026, worker: the shell hook refused two in-place helper commands

- **Doing:** editing `still.rs` from a heredoc'd Python script, and writing a scratch file to `/dev/shm` to compare two census curves.
- **What slowed it:** the local `dcg` hook refused both (a redirect to a variable path, and a truncating redirect to a home-like path). It was redone with the Edit and Write tools and a reducer script under `probes/`.
- **Cost:** about 3 minutes and two rewrites.
- **What would remove it:** nothing in the repository; it is a local setup rule, and the brief already warned of it. Reported, not to be specced.

## 10 Nov 2026, worker: the census's memory stop had to be lowered after the first ages ran

- **Doing:** the 120-year runs of all variants inside `telperion-heavy 12`.
- **What slowed it:** E at 120 ended its grow at 6.6 GiB and peaked at 9.1 GiB while dressed and drawn, so the first stop at 10 GiB resident in the grow would have let a grow end where drawing it meets the 12 GB cap. The stop was lowered to 8 GiB and the example rebuilt before the 200 and 300 runs.
- **Cost:** one rebuild, about 1 minute.
- **What would remove it:** a still runner that refuses a tree whose drawing will not fit before it dresses it, by an estimate from the grown tree's node count. Not needed for this child.

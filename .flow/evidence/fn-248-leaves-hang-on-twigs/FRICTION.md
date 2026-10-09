# fn-248 friction

## 2026-10-09, task 1 (R1 measurement)

- **No public leaf-to-wood record, and no wood age on `Tree`.** I needed to know which wood each leaf sits on and how old that wood is. Nothing public answers either question. Placement keeps no node index, and `plan::runs` / `bearing_of` are crate-private. The conversion drops each phytomer's `cycle`. Working around it took a code survey and an example that rebuilds both: it matches each plan descriptor to its node by bit-equal position, and it regrows the engine structure to look up each node's cycle by key. Cost: about 15 minutes and roughly 140k subagent tokens for the survey. What would remove it: a measurement accessor that returns each planned station's node, plus each node's birth cycle carried as record data, the way `grown` is.
- **`cargo clippy -p telperion-core` fails on the base.** The lib has 26 clippy errors that predate this branch, so a lint check of the new example cannot run cleanly. The example builds without warnings. Cost: 2 minutes. What would remove it: a clean lib clippy, or CI running it.
- **The dcg hook blocks `> $VAR/file` redirects.** A command that wrote a log through a shell variable path was refused, and I had to retype it with a literal path. Cost: 1 minute. This is a local setup issue, reported here and not specced.

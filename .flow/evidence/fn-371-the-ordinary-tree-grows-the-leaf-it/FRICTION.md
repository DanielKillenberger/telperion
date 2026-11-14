# fn-371 friction

## 14 Nov 2026, 06:32, builder (task 1): the probes overwrite their own "before" build

- **Doing:** measuring the ordinary tree before and after the leaf rows, with fn-352's scoreboard probe and fn-370's leaf probe built into one target directory.
- **Slowed by:** the presets are compiled into the probes, so the rebuild after the row change overwrites the only binaries that draw the candidate's tiles. The candidate's mid view was not yet drawn when the rebuild started; the binaries were copied aside by hand (`raw/bin-before/`) while the Wasm step still ran, before cargo reached them.
- **Cost:** about 2 minutes; a full rebuild of the base in a second worktree had it been missed.
- **What would have removed it:** the panel script drawing every tile a sheet needs (measures and stills) in one pass per build, or `build.sh` keeping each build's binaries under the commit it built.

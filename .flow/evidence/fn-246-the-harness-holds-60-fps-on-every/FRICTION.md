# fn-246 friction

## 2026-10-09: the Two Trees' baseline takes ten minutes

- **Doing:** R1's per-pass baseline, best of 3 processes per species.
- **Slowed by:** the frame probe measures every view with 140 untimed and 136 timed frames, and Telperion's whole view is 120 ms a frame, so one Telperion process is about 90 s and the two trees' three runs each took 9.5 minutes of the 10-minute foreground limit.
- **Cost:** about 10 minutes of wall time; a near miss on the command timeout.
- **Would have removed it:** a view filter on the probe (added in `examples/frames.rs` after this run, so the relief variants measured only the bare views).

## 2026-10-09: the command guard refuses a redirect into a variable path

- **Doing:** the first smoke run of the frame probe.
- **Slowed by:** the shell guard (dcg) blocks `> $R/file` because it cannot prove where `$R` points.
- **Cost:** one retry, under a minute.
- **Would have removed it:** nothing in the repository; written with literal paths since. Local setup, reported only.

## 2026-10-09: the wood's same-picture claim could only be checked after building it

- **Doing:** step 2, the wood fix's cause and design before code.
- **Slowed by:** whether fewer bark cells is "the same picture" is not decidable from the shader text. The only check is the bark tests, and they need the rule built. The stated design passed timing and then failed `bark_resolution`, so it was written, built, tested, measured and reverted.
- **Cost:** about 25 minutes and two GPU test runs.
- **Would have removed it:** a stated picture tolerance for wood changes, for example "the bark tests' bounds", decided by the host before dispatch. The stop rule would then have had a number to apply.

## 2026-10-09: the bark truth had to be built before the prefilter could be judged

- **Doing:** the host's prefilter decision, judged against a 4x supersampled truth.
- **Slowed by:** no tool drew the bark test views at 4x with the geometry held, so `examples/bark_truth.rs` was written first. The resolution tests' grazing masks are CPU rasters inside the test file, so the tool uses the clay-room wood mask for every view.
- **Cost:** about 20 minutes; each truth render is about 45 s.
- **Would have removed it:** a shared fixture list for the bark views (camera, family, size), which the tests and the tool both read.

## 2026-10-09: other sessions loaded the machine during timing

- **Doing:** the interim and prefilter timing runs.
- **Slowed by:** the load average reached 20 to 24 during some single runs (other sessions building). GPU timestamps are less affected than wall time, but those rows are one run each and are marked with their load.
- **Cost:** no reruns yet; R3 would need best-of-3 at low load.
- **Would have removed it:** the shared "machine busy" marker fn-185 already proposed. Local setup, reported only.

## 2026-10-09: the crown truth's absolute bound could not be met by any threshold

- **Doing:** fitting the thinning threshold against the 4x crown truth.
- **Slowed by:** the decision set the bark tests' 3.0 mean as the starting bound, but an unthinned crown is already 2.4 to 15 code values from its 4x truth. The bound had to be restated as an increase over the unthinned distance before anything could be fitted.
- **Cost:** about 10 minutes, and one rerun of the fit table.
- **Would have removed it:** measuring the unthinned baseline's own distance before the bound is set. The fit now has it as its T=0 column.

## 2026-10-09: no numpy for the oracle; large raw stills cannot be removed by the agent

- **Doing:** the error decomposition behind the wood stop, then cleanup.
- **Slowed by:** the system Python has no numpy (`uv run --with numpy --with scipy` worked). The shell guard refuses `rm -rf`, so the 4.2 GB of crown stills in `raw/crown-truth4` and `raw/crown-T*` stay on disk. They are gitignored.
- **Cost:** a few minutes, plus about 5 GB of disk until removed by hand.
- **Would have removed it:** nothing in the repository. Local setup, reported only.

## 2026-10-09: the crown bound was already spent when the card arrived

- **Doing:** fitting the floret card's threshold.
- **Slowed by:** the bound is one budget (3.0 mean over the unthinned view), and the 2 px leaf thinning had already used 2.96 of it on Telperion's whole view. The card could only be fitted inside the remaining 0.04, so the fit had no room before it started. That was visible from the earlier table but only became decisive once the card was built.
- **Cost:** about 40 minutes building, fitting and backing out the card.
- **Would have removed it:** fitting the leaf threshold and the card together against one shared budget, or stating a budget per mechanism.

## 2026-10-09: the cold-compile timing test fails when the render package runs under load

- **Doing:** the final gate, `telperion-render` under the GPU lock.
- **Slowed by:** `generation::fine::hang_tests::every_pipeline_within_2_5_s_but_grow_within_20_s_cold` measured the `grow` compile at 20.6 s and then 26.0 s against a 20 s bound, while other sessions held the load at 10 to 27 and the package's own tests ran beside it. Alone, at load 3, it passed in 23.5 s wall. The test times a CPU-bound driver compile, so it fails on a loaded machine. The change does not touch it.
- **Cost:** one rerun, about 5 minutes. The workspace gate also no longer fits one 10-minute foreground call at this load: core took 6.9 min and space 7.5 min, so the gate ran package by package.
- **Would have removed it:** running that test serially, or measuring the compile as CPU time rather than wall time; and a gate split that fits the foreground limit.

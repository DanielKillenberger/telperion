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

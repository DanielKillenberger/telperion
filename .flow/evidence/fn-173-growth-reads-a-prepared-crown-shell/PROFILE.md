# Growth profile, master f9487810 (2026-09-27)

fn-172, fn-173, fn-174 and fn-175 cite this profile.

## Method

- **Timing:** `examples/growth_profile.rs`, six samples, first dropped, medians. Native release build with frame pointers and line tables, on a shared machine, so absolute times drift. The oak read 78 ms in one run and 51 ms in another.
- **Attribution:** fn-124's scratch SIGPROF sampler (`.flow/evidence/fn-124-growth-in-half-the-time/growth_sampler.rs`, with `pipeline::skeleton(&f)` replaced by `pipeline::build(&f, Request::default())`) and its `analyze-native.py`, at 10 kHz. Its stage classifier predates the fn-158 file moves, so most samples read "stage unknown". The function table below is inclusive and complete.
- samply could not run because `perf_event_paranoid` is 2 on this machine.

## Growth time

| Preset | Seed 1 ms | Seed 7 ms | Nodes (seed 1 / 7) |
|---|---:|---:|---:|
| Oregon white oak | 79.6 | 77.7 | 125,087 / 139,040 |
| Norway spruce | 41.6 | 39.5 | 95,389 / 90,439 |
| Silver birch | 136.3 | 266.9 | 68,501 / 84,894 |
| European beech | 100.4 | 97.2 | 187,968 / 190,975 |
| Date palm | 1.7 | 1.8 | 552 / 553 |
| Ordinary | 45.9 | 49.7 | 11,567 / 11,927 |
| Telperion | 391.0 | 56.2 | 75,697 / 16,993 |

## Where the time goes (inclusive share of growth, sampler on)

| Fixture (ms per build) | Largest costs |
|---|---|
| Birch 7 (268.4) | `Curtain::admits` 81.3%, `in_band` 80.3%, `lower_surface` 78.6%, `radius_at` 53.3%, `noise::seeded` 22.7%, libm `expm1` 13.9% and `exp` 11.6% self |
| Telperion 1 (400.0) | `GrowthBias::apply` 47.0%, `Noise::curl` 42.4%, `Noise::at` 41.2%, `shed` 25.6%, `distance_to_profile` 19.7% |
| Ordinary 1 (47.7) | `finish` 48.7%, `shed` 47.4%, `distance_to_profile` 38.7%, `quadrant` 15.0% |
| Beech 1 (103.2) | `radius_toward` 32.7%, `rejected` 31.4%, `radius_at` 22.6%, `quadrant` 21.4%, `noise::seeded` 8.2%, radius solve 8.2% |
| Oak 1 (51.3) | `rejected` 26.8%, `radius_at` 25.1%, `quadrant` 24.1%, `limit_turn` 8.4%, `child_radius` 6.4% |

Raw sample files stay in the session scratchpad and are not kept.

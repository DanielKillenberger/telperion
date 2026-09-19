## Goal & Context
<!-- scope: business -->

Nothing bounds what the species suite holds. One test keeps `SEEDS_IN_FLIGHT = 4` specimens at once, cargo runs the four heavy tests together on any host with four or more cores, and the product is whatever the host's core count and the specimen's size happen to multiply to. Measured on the fn-85 build with 32 cores: the binary peaks at 8,637 MB and passes, and the same binary at `--test-threads=1` peaks at 3,944 MB. On a box with less headroom the kernel decides instead, which is the OOM fn-84's FRICTION.md recorded, where cargo reported `signal: 9` as a failing test and every suite after `species` never ran. [user]

The constant is the evidence for why a constant is the wrong mechanism. `species.rs:429` says "a spruce placement is about 350 MB". The real figure was 941 MB before fn-85 and is 470 MB after, and fn-86 will make it 88 MB. The number has drifted twice, in both directions, and no test noticed. [user]

This spec makes the specimen say what it will cost, computed from the tree it just grew, and admits seeds by dividing what the machine has by that answer. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The prediction is computed, never recorded.** A function in the core takes the grown tree and the family and returns the bytes a finished specimen will hold, by multiplying counts by the sizes the builders actually use: stations by `size_of` of the stored leaf record, wood vertices by positions plus normals plus coords, triangles by the index element. No figure in it is written down as a literal, so the encoding change in fn-86 moves it without anyone editing this spec's code. [inferred]
- **It runs before the expensive part.** `branching::generate` is 54 ms on the spruce and 119 ms on the beech, while surface and foliage are 210 ms and 3,328 ms. The tree carries the station and vertex counts, so the prediction is an O(nodes) pass that allocates nothing and can gate admission before a single matrix or vertex exists. [user]
- **Admission divides what the machine has.** The suite reads `MemAvailable` from `/proc/meminfo` once at start, keeps a reserve, and admits `floor(available / predicted)` seeds, bounded below by 1 and above by the core count. A host with room runs as wide as it did; a host without runs narrower instead of being killed. [inferred]
- **The cost is not charged twice.** The prediction is per specimen and the admission is per in-flight seed, so a test that admits N holds N predictions' worth. Nothing in the harness stores a per-species number. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A core function returns the predicted resident bytes of a specimen from its grown tree and its family, allocating nothing and building no mesh or placement buffer. Errors: a tree with no nodes predicts zero rather than failing.
- **R2:** The prediction is within 15 percent of the measured resident size for all five catalogue species, checked against the lengths the builders actually allocate rather than against a recorded figure.
- **R3:** No literal byte count for a specimen, a leaf or a vertex exists anywhere in the prediction or the harness. A test changes the stored leaf's size and asserts the prediction moves with it, which is what fails today.
- **R4:** The species suite admits seeds from available memory and the prediction, floor 1 and ceiling the core count, and `SEEDS_IN_FLIGHT` is gone. On any host the binary's peak stays under the reserve the admission left itself, asserted by the suite polling its own VmHWM.
- **R5:** Wall time does not regress on a host with room: the suite at 32 cores stays within 15 percent of the 41 s it runs today, and a host with less memory runs narrower and slower rather than being killed.

## Boundaries
<!-- scope: business -->

- The leaf encoding is fn-86 and the cull copy was fn-85. This spec changes no generator output and no stored byte; it only predicts and schedules.
- The wood mesh's own size is not reduced here. After fn-86 it is the largest remaining term per specimen, 168.3 MB on the spruce, and that is its own question.
- No CI configuration and no `package.json` thread cap. A static cap was considered and rejected: it bounds only the outer factor, does nothing above four threads on this suite, and still leaves one test holding four specimens.

## Decision Context

- The owner chose on 2026-09-19 that the memory fix is fn-86 and that this scheduling work is captured separately, with the requirement that the figure be calculable from the tree instantly rather than a constant maintained over time. [user]
- Measured on 2026-09-19 on the `ci` profile, peak RSS polled from `/proc/<pid>/status` VmHWM: the whole binary at 32, 4, 2 and 1 threads peaks at 8,637 / 8,631 / 5,178 / 3,944 MB and finishes in 41.1 / 40.8 / 63.9 / 93.1 s. Above four threads the cap changes nothing, because only four tests are heavy. [user]

# fn-259 friction

- 2026-10-15 22:25, the release fork, gate on the R7 format branch: `telperion-heavy 20` (fn-271's declared gate size) waits for 20 GB of room on a 31 GB desk where Chrome, Steam and several Claude sessions hold about 16 GB; room read 6-12 GB for the first 30 minutes. Cost: the gate cannot start while the owner's desktop is in use. What would remove it: a gate whose whole-preset tests fit a smaller cap (fn-271's "a smaller core-lib peak, later work"), or the gate run on a quiet machine or CI.
- 2026-10-15 22:24, the site bench: F4's field package grew the spruce at seed 1234 in 7.0 s and was killed at the 6 GB cap on the next seed (Node holding several Wasm instances' memory between releases). The site-call bench needs one Node process per tree, as fn-245's PROFILE did.

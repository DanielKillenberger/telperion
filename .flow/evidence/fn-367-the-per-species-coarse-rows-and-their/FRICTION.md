# fn-367 friction

## 14 Nov 2026, 02:31, the Wasm "before" build queued behind the edits (worker)

Doing: R5 needs the Wasm module's raw bytes before and after. I queued the base build in the task's worktree, then went on to the deletion while it waited.
Hindered: the queue held it for over three minutes ("room 2 GB for 8", then a yield PAUSE), so it had not started compiling when the source was already edited; it would have measured the edited tree as the base. I stopped it and rebuilt the base from a `git archive` of `78a8e6a94` in the scratchpad, with its own target.
Cost: about 4 minutes of queue time, plus one extra cold release Wasm build (a second target directory).
Would have removed it (02:31 entry): on my side, take the before bytes from an export of the base, never from the working tree while edits proceed. In the repo, a recorded Wasm size per commit (CI's artifact budget already measures the package) would make "before" a lookup, not a build.

## 14 Nov 2026, 02:47, the digest pin read red after the deletion, with no base reading (worker)

Doing: R3, the pin's fine end after the deletion.
Hindered: all eight fine-end digests differ from the pinned table, including the palm and the ordinary tree, which carry no coarse rows. The pin was last moved at `986867c3e`; fn-364's grower change (`f4ce1ded4`) landed after it, so the base may already be red. The repo keeps no receipt of the pin at the base, so telling "inherited" from "caused" takes a second cold native build of the base export (the lib test also `include_str!`s `.flow/evidence` files, so a `crates/`-only export does not compile; a second export of the whole tree).
Cost: a cold `ci` build of telperion-core's lib tests in a second target directory, about 5 to 10 minutes of queue, plus one failed attempt.
Would have removed it: a merge into a child branch that moves a digest pin re-pins it in the same merge (or the gate's receipt names the pin red at that head), so a child starts from a green pin.

## 14 Nov 2026, 02:56, the second narrow round waits for memory (worker)

Doing: the re-pinned identity, the catalogue tests, the every-preset repeat test, the package's parameter tests and the two guards, in one queued job.
Hindered: the queue has held it since about 02:49 with 0 to 3 GB of room for its 8 GB; nothing of mine is running. The work itself is a few minutes.
Cost: so far 7 minutes of waiting, on top of about 12 minutes of earlier queue waits in this task.
Would have removed it: nothing in the repo; it is the desk's load (a local condition, reported, not specced).

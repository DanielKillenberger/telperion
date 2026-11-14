# Friction, fn-365

## 14 Nov 2026, 02:30: a dependency listing waits for 8 GB

- **Doing:** R1's reading, `cargo tree -p telperion-core --no-default-features --features json`, which reads `Cargo.lock` and compiles nothing.
- **Slowed by:** the run's rule queues every cargo command through `telperion-heavy 8` with an 8 GB reserve, and the machine had 2 to 5 GB of room. The listing waited from 02:26:54 behind memory it does not use, and the yield watcher paused the queue besides.
- **Cost:** 5 minutes 41 seconds (queued 02:26:54, admitted 02:32:35, done 02:32:38); the test build queued after it was admitted first of wall time before the listing could start, for a command that runs in under a second.
- **What would remove it:** a queue class for cargo's metadata commands (`tree`, `metadata`, `locate-project`) that admits them without a memory reserve, or a rule that names them as not heavy.

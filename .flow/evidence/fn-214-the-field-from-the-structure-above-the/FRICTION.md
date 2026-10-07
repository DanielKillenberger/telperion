# fn-214 friction


## 2026-10-07 10:57, task 1: timings under other workers' load

- **Doing:** timing the coarse field against 0.1.4 in Wasm and natively, then the coarse build's stages on one core.
- **Slowed by:** load ran 15 to 155 on 32 cores across the hour (another worker's binaries). One native stage breakdown read the predictor at 86 to 120 ms and the settle at 552 to 625 ms against 18 ms and 80 to 95 ms a minute earlier, so it was thrown away; the Wasm sweep ran at load 22 to 41 and needed a second, quieter pass (load 13) for the gate's comparison.
- **Cost:** about 6 minutes of reruns and a pinned (`taskset`) pass.
- **Would remove it:** the desk-wide timing lock fn-198's FRICTION.md already names; nothing new.

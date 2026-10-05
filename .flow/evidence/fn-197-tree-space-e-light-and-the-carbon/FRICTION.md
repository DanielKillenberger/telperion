# fn-197 friction

## 2026-10-05, design research: no per-stage timing in the engine

- **Doing:** costing the light and carbon options against today's 80-year grow time.
- **Slowed by:** `grow()` is the only public entry, and nothing reports how its time splits between the cycle loop, `assign`, `shed`, `thicken`, `place` and sag. I wrote a throwaway example that times whole grows and a sag-off variant, then deleted it. That gives the sag pass by difference, and the rest stays unknown.
- **Cost:** about 10 minutes and two release builds. The split the design needs most (the cycle loop against `place`) is still unknown.
- **Would remove it:** a `timings` example, or a feature-gated stage timer in `grow.rs`, that prints each stage's time for the shipped species at 80 years.

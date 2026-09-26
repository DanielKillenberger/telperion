# fn-159 friction

## 2026-09-26: dormancy controls that were not controls
- **Doing:** writing the behavioural dormancy test.
- **Slowed by:** two "awake" controls chose a value that the row already acts like. Pendulous radius 0.9 is below the default of 1, and every twig is under both. Max taper exponent 1 was never reached. One claim, lobeDepth's, also turned out false. These cost about 10 minutes and 5 test runs.
- **Would have removed it:** a catalogue row that states the value at which it is inert, so a test can pick the opposite end mechanically.

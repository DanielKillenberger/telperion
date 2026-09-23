# Friction: fn-114

## 2026-09-23: the fn-80 record cannot say which rung each round kept

- **Doing:** building R3's replay of the palm's fn-80 rounds from the committed evidence.
- **Hindered by:** `result.json` keeps each attempt's reviewer words and per-priority grades but not the values a bundle moved or which variant a round adopted, and `run.json` with its route notes was not committed. So the replay could not follow the dial along the path the run took. It replays every round at its top rung instead and says so.
- **Cost:** about 10 minutes of reading evidence, and a replay that shows a reach rather than a trajectory.
- **What would remove it:** `result.json` attempts carrying each move's `from` and `to`, and a flag on the attempt a round adopted and whether it stood.

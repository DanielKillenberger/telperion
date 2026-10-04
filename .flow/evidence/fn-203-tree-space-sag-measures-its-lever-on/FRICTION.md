# fn-203 friction

## 2026-10-05: R1 as written is not red on the old sag, and the design meets two cases it does not name

- **Doing:** writing R1 red first, then R3.
- **What slowed it:** a free-air branch on the old sag never bends past vertical, because the 0.15 rad backstop holds it. So R1 as worded cannot be red. Probing the round-9 spruce showed the loops form where boughs rest on the ground, not in free air. Building the design then raised two cases the spec does not name: an upright trunk buckles, and wood landing on the ground turns a corner.
- **Cost:** about 60 minutes of probes and three walk runs. The task stops for the host.
- **What would have removed it:** running R1's repro on the old code before the spec was marked ready (AGENTS.md "Gates and checked claims").

## 2026-10-05: the landing corner under heavy sag

- **Doing:** making `wood_lands_on_the_ground_without_a_corner` pass under host decision 2.
- **What slowed it:** heavy-sag wood now hangs nearly vertical when it reaches the ground. The landing ease flattens only the step's direction, so the angle turns sharply over the last step or two. Four approaches were tried, about 40 minutes:
  - easing by angle broke the straight-down landing test;
  - easing the running direction too, for all wood, kept twigs from reaching the ground;
  - easing it only for sagging wood is a switch at sag 0;
  - the chord look-ahead (built) brought the corner from 1.10 to 0.52 rad.
- **Cost:** about 40 minutes. Stopped for the host.
- **What would have removed it:** the landing behaviour under large-deflection sag being part of the spec's design.

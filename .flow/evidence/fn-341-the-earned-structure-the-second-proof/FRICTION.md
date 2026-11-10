# fn-341 friction

## 10 Nov 2026, R1 measuring, the worker

- **Doing:** reading each limb's angle and reach on the settled engine tree for R1.
- **What slowed it:** a limb in the engine is not one axis. It is a chain of relay axes, each about 0.2 m long (an apex that dies is carried on by a relay; the oak's limb has abortion 0.15 and relay 1.0). The first probe read a lateral's heading along its own axis and so measured only its first 0.2 m; the chord came back 0.2 m for limbs 5 to 12 m long. Nothing in `telperion-space` or `pipeline::measures` gives a limb as one path, so the probe had to walk the thickest wood itself.
- **Cost:** one rebuild and a second run of every preset and age, about 10 minutes of wall time and the oak at 300 grown twice (about 5 minutes on four threads).
- **What would have removed it (the limb walk):** a measure helper that follows a lateral's thickest wood across relays and continuations (the walk `pipeline::foliage::clumping::systems` does by start radius on the drawn tree, offered on the engine's `Structure`).

## 10 Nov 2026, R3 G's paper check, the worker

- **Doing:** reading the leaf above the oak's trunk at 1.3 m every year to 300, for G's paper check.
- **What slowed it:** a tree at a year exists only after a settle of a copy (RETHINK, "a year is not a state"), so the trace settled 300 times. The oak at 300 took about 20 minutes on four threads, against about 1.5 minutes for one grow. G's probe as built pays the same: it settles a shed, unlaid copy every year to lay its rings.
- **Cost:** about 20 minutes of wall time for one seed. R4's oak at 300 on two seeds, timed on one thread, would cost far more.
- **What would have removed it:** the leaf a section carries, read from the grower's own state each year (the approach's "a year is a state", item 6), with no settle of a copy.

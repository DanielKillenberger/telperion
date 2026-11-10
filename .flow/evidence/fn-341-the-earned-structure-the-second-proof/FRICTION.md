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

## 10 Nov 2026, R3 U on the oak at 300, the worker

- **Doing:** the probe read of the oak at 300 under G, P and U, before R4's stills.
- **What slowed it:** under U the oak's held structure grows without the bound the frozen upkeep gave it. It was killed at a 24 GB cap after about 7 minutes, and so was U alone after about 3. Nothing warned before the cap. The run at 120 had already shown twice the cycle's axes, but not at what rate the structure grows past it.
- **Cost:** about 12 minutes of wall time on two killed runs.
- **What would have removed it:** a census of held phytomers per year, printed as the grow goes, so a run outgrowing the desk is seen and stopped early. Today the census is read only at the end.

## 10 Nov 2026, R4 stills, the worker

- **Doing:** R4's stills, one GPU job through `telperion-heavy 12 --gpu 2`.
- **What slowed it:** the job waited for VRAM. Another program on the desk held about 5.2 GB of the card's 10 GB, so free VRAM stood below the 3 GB the queue needs. That is a local setup matter, reported and not specced.
- **Cost:** the stills waited for as long as the card stayed full; the CPU reads ran meanwhile.
- **What would have removed it:** nothing in the repository. A smaller `--gpu` claim would let the job start, if a still is measured to need less.
- **Later the same hour:** the wait was the queue's own rule, not the claim's size. `telperion-heavy` admits a GPU job only while other processes hold under half the card ("the owner games"), and a game held 5.2 GB of 10. A 1 GB claim waits just the same. The stills wait for the card; the worker does not override the rule.

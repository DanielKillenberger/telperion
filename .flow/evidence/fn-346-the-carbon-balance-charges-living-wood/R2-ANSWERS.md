# fn-346: the host's read of the probe and what is measured next, 11 Nov 2026

## The host's verdict on P1 and P2 (viewed: sheet-oak-1001.jpg, sheet-others.jpg)

- **Oak 40: the same** as today's.
- **Oak 80: slightly worse.** The same tree with a thinner crown; it still reads as an oak.
- **Oak 120: worse.** The skeleton is good, and arguably better than today's: wide (26 m), with many limbs leaving low, where today's is a two-limb V. But it is unclothed: bare limbs with tufts at their ends.
- **Oak 200 and 300: much worse.** Limbs are lost whole and nothing replaces them. P1 ends as a looping limb on a stump, P2 as a bare spike.
- **Every other preset: the same.**

Not a candidate. Nothing enters the crates.

## What is wrong, as far as it is measured

The charge does what it was written to do: it bounds the tree and sheds what does not pay. What fails is what comes after a shedding. The charge settles at about the income and then follows it down; living tips fall every decade; the doubled instrument keeps its limbs to 300 and still declines after 160. So the tree has no steady state: a tip that is shed is not replaced, and a tip that pays keeps extending at its row's rate until it no longer pays. Today's rules only describe a tree that is getting bigger.

That is the host's reading of the curves, not yet a measurement. The next step measures it. No rule is proposed until it is.

## The measurement (no behaviour changed; census probe only, kept as `probes/renewal.patch`)

1. **From the code** (`oak.rs`, `grow/unit.rs`, `grow/wake.rs`, `dormant.rs`, `grow/advance.rs`, the zones' rules): every way a living tip comes into being (a lateral on a node laid this year, a sleeping bud waking, a relay, a continuation), and for each age row: its life, what it moves on to, how many nodes a year it makes, whether its nodes bear laterals and sleeping buds. In particular: can a short shoot at the end of its row go on year after year making leaves in place, and for how long; where sleeping buds are held, what wakes them, and whether waking reads light or the loss of the wood beyond.
2. **Census,** oak seed 1001, on A (today's), P1, and P2 with `L_max` doubled, every 10 years to 300 or as far as each goes:
   - births of tips by source (as listed in 1) and by age row, in bands of path;
   - deaths of tips by cause (balance, survival draw, end of life) and by age row, in bands of path;
   - living tips by age row, with the mean yearly extension of each row;
   - sleeping buds held and woken;
   - for the tips the balance shed in a year: the lattice light at their places the year after, beside the light the surviving tips stand in. This says whether light returns to where the crown emptied.
3. **Arithmetic:** per decade and per variant, births over deaths for each age row; the share of deaths that fall inside the crown (more than 3 m of path short of the longest living path) against the share of births there; and, on P1 between 79 and 159, how many of the leaves lost belong to tips that were shed against tips that died of age or the survival draw and were not replaced.
4. **Questions for the host** the numbers raise. Do not answer them and propose no rule.

About 75 minutes. Result as `RENEWAL.md` beside RESULT.md. Return then.

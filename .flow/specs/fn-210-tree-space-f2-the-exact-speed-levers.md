# Tree space F2: the exact speed levers

## Goal & Context

fn-198's cost table (DESIGN-OPTIONS.md): 75 to 79% of grown phytomers are shed, and 92 to 97% of those sit in subtrees whose shedding is certain at the bud's birth from lifespans and delays alone. Growth is single-threaded. The oak's last-cycle light work is provably wasted. These levers make full fidelity cheap without changing any tree.

## Design (host, 2026-10-05)

- **Shedding decided before growth:** a subtree certain to be shed before the tree's age is not grown. Exact (hash-identical) for every species where nothing reads that subtree while it lives; where light, vigour or the balance read it, its contribution comes from the closed form (the same coarse coupling F1 uses), measured and judged.
- **Parallel axes:** independent axes and subtrees grow on all cores (lineage keys make order irrelevant); hash-identical.
- **Skip the last cycle's light work; gate the rough layout on species that read light; a compact phytomer:** hash-identical.

## Requirements

- **R1:** Each lever with its hash proof (or, for the oak's shed subtrees, the coarse-coupling measurement and stills).
- **R2:** Per-stage cost at 80 years, every species, before and after, on the owner's machine.
- **R3:** Workspace gate; Codex review.

## Boundaries

Depends on fn-206. No change to any tree beyond what R1 names.

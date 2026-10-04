# The tree space

The programme behind STRATEGY.md's track "The tree space" (owner, 2026-10-03): one growth engine from trunk to twig in which every tree species is a point in one continuous space, botanically accurate and fast enough for games, so that adding a species is calibrating values.

## The claims, and how each is measured

The programme aims at four claims that, to our knowledge, no generator makes together. None is made until it is measured and a prior-art search has been run.

| Claim | Measured by |
|---|---|
| Architecture from botany | Each species reads as itself beside its reference photographs, on stills the host screens and the owner judges; its architectural model (Hallé and Oldeman) is traceable to sources |
| One continuous space for every architecture | Coverage: how many of the 23 architectural models and how many species the space draws recognisably |
| Continuity by construction | Walks between any two points at a fixed seed change the tree by degree with no visible pop, on still strips and a per-step measure |
| Game speed | End-to-end time and memory per mature tree and per forest on the named machine, against today's pinned baseline |

## Stand on botany, invent the space

- **Reproduced, never invented:** plant science (physiological age, the reference axis, the architectural models, light interception, the pipe model). Each is reproduced against an oracle (an executable model run unchanged, or a published closed form or figure) before anything is built on it.
- **Invented:** the continuous space that holds every architecture, its continuity guarantees, and its speed.
- **Never copied:** external code or data. Oracles are run and compared against; Telperion's code is written from the published papers.

The 2026-10-03 probe rounds (fn-188, fn-190) are the reason: eighteen rounds built engines from prose with no oracle, and none drew a tree.

## Phases

Each phase is its own spec with its own gate, captured up front as a chain (each depends on the one before) and planned only when the run reaches it, with its predecessors' evidence. Breadth is one spec per species (AGENTS.md).

| Phase | Delivers | Gate | Stops (wrong path) when |
|---|---|---|---|
| A. Oracle and engine core | A physiological-age engine crate whose structures match Letort's GreenLab simulator (run unchanged) in distribution and GreenLab's closed-form counts exactly; the whole-tree reference chosen | Oracle tests pass; host-screened side-by-side sheet | The oracle cannot be run or matched after three recorded attempts |
| B. Continuity by construction | Lineage-keyed randomness, birth and death at vanishing size, discrete botany as settings | Fixed-seed walk tests with no jumps; A's oracle tests still pass | A jump cannot be removed without breaking the oracle |
| C. The beech | A mature open-grown beech from sourced architecture data | The host is confident the sheet reads as a beech beside the photographs, and Astra independently agrees on the same sheet | Two rejections (host or Astra) naming the same trait after a change aimed at it |
| D. Breadth | Spruce, oak and palm, one spec each; walks between them | As C, per species; the palm also meets today's palm | As C, per species |
| E. Feedback | Light and the carbon balance: bole, density, bounded size | No passed species looks worse, judged as C; walks stay continuous | A passed species regresses and cannot be restored |
| F. Production | The engine inside the one pipeline for the passed species; today's generator keeps the rest | Workspace gate, speed against the pinned baseline, Codex review, an open PR | Speed bar missed after measured optimisation |

Calibrating further species is the add-species pipeline's job (`docs/species-onboarding.md`), recalibrated for the new engine in its own spec once F lands.

Specs: A fn-191, B fn-192, C fn-193 (beech), D fn-194 (spruce), fn-195 (oak), fn-196 (date palm), E fn-197, F fn-198.

## How a species is judged

The host views every still first and fails the obvious ones itself. A species passes a phase when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it. The owner judges every passed species in the morning review; a rejection sends that species back without undoing the others, because every phase lives on its own chained branch.

## Running unattended (owner, 2026-10-03)

The phases are captured up front as chained specs (A to F), each depending on the one before; the owner marks them ready, which is the consent for the run. The driver is flow-next's own: `/loop /flow-next:flow --auto`, which drives the next ready spec whose dependency is met to an open PR, branching from its predecessor's pushed branch, and repeats.

- **Gates are evidence, not sign-off:** oracle and walk tests (A, B), the host-and-Astra visual pass (C to E), the workspace gate, speed and the Codex review (F).
- **The owner's verdict comes in the morning,** on every passed species and on F's PR; nothing merges to master without it.
- **The run stops** only on a backend outage, `NO_WORK`, or a genuine wrong path: the host can name no next design step the evidence supports, or the engine cannot express the botany. A trait that fails after a change aimed at it is not a stop by itself (owner, 2026-10-04): the host decides the design question the failure raises, records it, and continues. A stop writes what was tried, what failed and the decision needed.
- **Merging (owner, 2026-10-04):** a phase's PR whose checks pass and whose review passed (a host-reset review counter counts) may be merged by the run.
- **Never:** merge a PR with failing checks, copy external code or data, change the strategy, or pass a sheet the host would not stand behind.

# fn-34 gap analysis: what the beech and the birch ask of the generator

2026-09-14, after round 3. Three rounds of matched pairs against the Oregon
State photographs: round 1 at the fixed views (rejected), round 2 at the
matched camera (the instrument), round 3 after two value passes on both
tables. This file names every difference still visible in the round-3 whole
pairs and says whether a row can close it, an appearance spec owns it, or the
generator lacks the form. The owner's rule for the catalogue: a species that
exposes an unsupported form or organ becomes a generator spec for the
frontier tier, and the species is compared again once that spec lands.

## Reading the pairs

| Difference | Species | Rows can close it | Owner |
|---|---|---|---|
| Two stems from the base, one leaning | birch | no; one root node, one leader | gap: multi-stem trees |
| Long unbranched shoots hanging in a curtain to the ground | birch | no; the curtain mode exists but its droop is a constant and its shoot length is borrowed from the twig anatomy | gap: pendulous shoots |
| Crown outline lumpy and asymmetric, not a smooth oval | beech, birch | no; the envelope is a smooth shell every seed fills the same way | gap: crown outline irregularity |
| Foliage reads as one continuous mass with sky showing through in holes | beech | partly; density is at the node cap, the rest is how a leaf mass is lit | appearance (fn-29 follow-on, fn-32 method) |
| Bark and foliage bluer and darker than the photograph under overcast | beech | partly; rows moved it, the sky term does the rest | appearance |
| Trunk reads short and thick | beech | closed in round 3 (crown base, flare) | done |
| Crown base too high on the birch (0.20 against 0.08) | birch | no; the curtain is what reaches the ground | gap: pendulous shoots |
| Leaves in tufts rather than strings along hanging shoots | birch | no; a tuft is a clump at a station, a string is a shoot | gap: pendulous shoots |
| Width over height, fill | both | closed; within 0.05 of the photograph | done |

## The three gaps

### Multi-stem trees

A birch, a hazel, a coppiced oak and Yggdrasil's three roots all start from
more than one stem at the base. The generator grows one leader from the
origin and every downstream stage assumes one root node: the radius solve,
the flare, the profile's DBH rule that already flags "multiple structural
stems" as ambiguous, the identity pins. Stems as order-zero axes born at the root, with count, divergence and lean
as habit rows, each stem a trunk run to the surface and the base shared by
the pipe model: fn-38-multi-stem-trees.

### Pendulous shoots

Reading the code corrected the first draft of this entry: the generator
already has a curtain. A negative secondary rise flips a pendant mode in the
twig layer that the spruce and the birch use today. What it lacks is rows:
the droop is a constant (a 0.35 cap and a 0.5 slope), the mode is a switch
across zero rather than a continuous row, a pendulous shoot's length is
borrowed from the twig anatomy, and no leaf may point downward because the
canopy's orientation rails stop at zero. The spec makes the curtain rows the
blend can walk: fn-37-pendulous-shoots-as-rows.

### Crown outline irregularity

Every seed fills the envelope's smooth shell to the same outline, so a tree's
silhouette is an oval however the branches inside it vary. A photograph's
crown is lumpy: lobes where scaffold limbs end, gaps where they do not. A
per-seed, low-frequency perturbation of the envelope radius by height and
bearing (an amplitude and a wavelength row, keyed by the seed) is the
smallest term; the containment tests then hold against the perturbed shell
while the two-dimensional profile keeps driving retention: fn-39-crown-outline-irregularity. Growth-related
irregularity, limbs that die or are shaded out, belongs to fn-16 and fn-21
and is not this term.

- Fine hanging shoots take the trunk's bark colour, so the birch's curtain
  reads white where its twigs are dark brown: a young-shoot colour by radius
  is a row for fn-40, not a form gap.
- Crown outline irregularity landed as fn-39 (round 5): both tables state an
  amplitude, both silhouettes left the oval. The beech had to give up a shade
  of twig length for node headroom; the ceiling is still the owner's cost
  decision, not a form gap.

## What is not a gap

- Foliage mass lighting and the blue cast under overcast: fn-29's rows moved
  the bark; the leaf mass needs the sky term and the transmission to read as a
  canopy, which is appearance work in fn-32's method.
- Leaf size, blade shape, bark colour, crown base, flare, density up to the
  node cap: rows, and closed or bounded by the profile's gates.
- The node cap itself: the beech hit it at five laterals a station; raising
  it is a cost decision, not a form gap.

## Order

fn-37-pendulous-shoots-as-rows first (it closes three of the birch's rows and serves the most
species), then fn-38-multi-stem-trees, then fn-39-crown-outline-irregularity. fn-34 depends on all three. fn-34's next round
renders the pairs again when each lands; fn-34 closes on the owner's verdict
at that round.

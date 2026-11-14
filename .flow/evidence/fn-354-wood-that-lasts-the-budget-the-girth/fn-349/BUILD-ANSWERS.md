# fn-349: the host's answers to BUILD.md, 11 Nov 2026

Each is the host's design decision; none is the owner's ruling.

R5 is accepted: five of six rows within 1%, and the sixth explained by a last-bit difference between multiplying by 1 / H and dividing by H, shown by the six rows matching exactly under division. The relay-on-an-empty-bearer rule (it starts at that bearer's base) is accepted.

## 1. The "pays" test is a switch, and the rule is restated so it is not

The walk tests are right. As the host wrote it, a child lateral's whole light and whole charge enter its bearer's sums the moment it pays and leave the moment it does not, so the bearer's balance jumps by a finite step when a child crosses break-even. That breaks "every parameter changes the tree by degree".

The rule, restated: **a part earns its own light plus what its paying children have left over, and costs its own charge.**

    S(part) = own light + sum over child laterals of max(0, S(child)) - own charge       (the surplus)
    Q*      = own light + sum over child laterals of max(0, S(child))
    C*      = own charge
    balance = (Q* - C*) / (Q* + C*), and 0 where both are nothing

- A child at break-even hands its bearer nothing whichever side it is on, so nothing jumps.
- The sign of every balance is what it was under the first form: a part's balance is below zero under one exactly when it is under the other. What changes is the size: the denominator no longer carries the children's charge, so a failing part reads further below zero and is shed sooner.
- Continuations and relays are the bearer's own line, as before: their light and charge are "own".
- Because the size changes, the trees are drawn again before the host accepts it (below).

## 2. The steepness of the walk over the inverse height

Validation stays 0 to 1. The scale row's span, which is what the walk steps over, is the span real trees stand in: 0 to 0.1 a metre (no charge, to a potential height of 10 m). If the walk is still too steep over that span after answer 1, return with the numbers; do not loosen the test.

## 3. The two lattice tests

They keep checking what they promised: a growth that failed is spent, and a lattice past its nodes is refused by name. Reach the refusal through the lattice's own value (a finer cell on a small tree) and not through a taller tree, which the corrected charge no longer keeps small. If no value of the lattice reaches the refusal on a tree that fits the desk, return with what you found.

Separately, as a finding and not a fix here: a lattice just under the refusal limit lets each core take its own 1.87 GB and the run dies at 20 GB. Write it into BUILD.md under its own heading with the numbers; the host brings it to the owner.

## 4. `each_populated_response_moves_the_tree_through_its_consumer`

The response under test is the balance's hazard, so the fixture gives it a tree on which the balance acts: the test raises the inverse height of the species it builds (a short tree) until the balance sheds by the age it already runs at, for every preset it covers. It does not move to 60. The preset values are not changed for it.

## 5. Then draw the product's trees

On this branch's build, through `telperion-heavy`, sag on, leafless and leafed, beside today's (today's stills exist under `.worktrees/fn-319/.flow/evidence/fn-345-what-bounds-a-tree-vigour-falls-with/raw/stills` and `.../fn-346-the-carbon-balance-charges-living-wood/raw/stills`, variant `A`; copy what you need, draw what is missing from a build of this branch's base commit or ask):

- the oak at 40, 80, 120, 200 and 300 on seed 1001, and at 120, 200 and 300 on seeds 1007 and 1012, with the probe's SR stills beside them where they exist;
- the beech at 40 and 80, the birch, spruce, palm and ordinary at 40, and **laurelin and telperion** at their preset ages, today's beside the product's (the last two were never drawn under the corrected charge);
- the census to 300 on seeds 1001 and 1007: height, longest path, living tips and held phytomers every 20 years, to see the steady state holds under the restated rule;
- one-thread seconds and peak memory for the oak at 40, 120 and 300 on seed 1001, today's build and this one in one session.

Sheets as JPEGs in this spec's evidence, with a short `DRAWN.md`: the measures and your plain read per tree against today's and against the probe's SR.

Commit as you go. No gate, no review, no device, no merge in this dispatch. Stop by 08:30 Zurich whatever is done, tree committed and clean, and return. The host views every sheet; the device step follows on the host's word.

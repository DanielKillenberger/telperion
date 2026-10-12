# fn-264: the birch's ground limb and bare whips, isolated (worker, 2026-10-12)

The host asked for the cause of the thick white limb lying on the ground and the long bare whips in the birch's mid view. The birch alone was grown at 40 with one row group set back, at seeds 42 and 1, each one tree at a time behind the memory gate. The whole and mid stills came from the device, and the lying wood was measured on the CPU reference tree: nodes under 0.4 m and more than 0.8 m out from the stem's foot. A probe inside the engine read the light and the ages behind them. Stills and the scratch tools are in `raw/birch/` (ignored); the tools are not on the branch.

## What each row does

| Variant (seed 42 ; seed 1) | Thickest lying radius, m | Short shoots kept (age 14) | Ground limb and whips (viewed) |
|---|---|---|---|
| tip | 0.0278 ; 0.0230 | 43,244 (as a, which changes girth alone) ; 32,236 | thick white limb; long bare arching whips |
| (a) leaf girth 0 | 0.0224 ; 0.0182 | 43,244 ; 32,236 | limb thinner and lighter; whips remain |
| (b) shade hazard and size 0, leaf area kept | 0.0216 ; 0.0170 | 60,749 ; 47,874 | limb thin; fewer whips |
| (c) leaf area 0, responses kept | 0.0159 ; 0.0121 | 60,776 ; 47,880 | as before fn-264 |
| (d) balance hazard 0 | 0.0277 ; 0.0230 | 43,378 ; 32,297 | as the tip |
| (e) shade hazard alone 0 | ; 0.0290 | | thicker, more whips |
| (f) shade size alone 0 | ; 0.0155 | | as before fn-264 |
| (g) shade size 1.1 (the oak's mean) | 0.0265 ; 0.0219 | | limb white, whips remain |
| (h) shade size 0.55 | 0.0228 ; 0.0184 | | limb thin; a few bare arcs |
| before fn-264 | 0.0159 ; 0.0132 | | thin dark limb, no whips |

## The cause, checked against the code

1. **The size response, first.** A unit's size is (vigour / presence)^ψ, normalised so its age's mean unit stays whole (`allocation.rs:147-154`, read in `grow/cycle/passes.rs:102`). The mapping gives the birch ψ = 1.1 x (5 − 2.03) / 2.6 = 1.257, the strongest of any species (beech 0.19, spruce 0.23; the oak's ages 0.5-1.5). The birch's leaves stand on short shoots along long hanging shoots (`birch.rs`). Under a size response the well-lit outer long shoots take long units and the shaded short shoots take short ones, and the shade hazard kills shaded short shoots (a third fewer kept, b against tip). The long shoots therefore run on bare: the whips. Shade size alone at 0 removes both defects (f). They grade with ψ (g, h): even the oak's mean ψ of 1.1 gives them on the birch.
2. **Why the lying limb in particular.** It lies out from under the crown, so the lattice gives its buds near-open sky: the lying phytomers' light is 0.92 against the whole tree's 0.63 (seed 1; 0.90 against 0.54 at 42 with b). It is the best-lit wood of the tree, so its units grow largest and it keeps extending along the ground.
3. **Leaf girth, second.** A phytomer's own pipe is scaled by its light over the tree's pipe-weighted mean (`girth.rs:100-170`). The light it reads is the light its bud grew in when it was laid, stored for good (`presence.rs:332`), and read for every phytomer of every age, including old wood that has long borne no leaf. The lying limb's phytomers were laid over cycles 14-40 at a light of about 0.92 under a leaf mean of 1.40, a factor of about 1.29 on their own pipe. Setting leaf girth to 0 takes about 20% off the limb's radius (a). The shedding by balance plays no part (d is the tip).
4. **The limb itself predates fn-264.** The birch grown before fn-264 already lays a thin dark limb along the ground (0.0159 / 0.0132 m). The responses make it the tree's most favoured wood.

Both of the host's stop conditions are met in part. Leaf girth gives the lying limb girth from leaves the old wood no longer carries (item 3). The larger cause, though, is the size response on the birch's architecture (item 1), which neither condition names.

## For the host to decide

- **The size response on the birch.** Any ψ near the oak's makes its long shoots run bare. Options: hold the size response to its value at the oak's T or below for intolerant species (cap the factor at 1); or tie ψ to architecture, not tolerance, as apical control is. The stills: ψ 0.55 nearly clean, 1.1 not.
- **Leaf girth's light.** It reads the light at a phytomer's laying for the life of the wood. Either it reads only wood that still bears leaves (its leaf retention), an engine change of its own; or the birch takes leaf girth 0 until then.
- **Wood lying on the ground.** A limb sagged onto the ground predates this spec. A ground-contact rule (shed, or no growth below some height outside the stem) would be its own spec.

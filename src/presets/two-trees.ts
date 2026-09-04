import type { TreePreset } from "./preset";

/* ------------------------------------------------------------------ *
 * TELPERION AND LAURELIN
 *
 * The spec's acceptance test. Two named, deliberate trees out of one
 * algorithm, with nothing differing between them but the numbers in
 * this file. They are judged in clay - flat grey, no silver and no
 * gold - so every difference below has to be a difference in FORM or
 * it does not count.
 *
 * WHAT THE REFERENCES AGREE ON. Across Roger Garland's `Two Trees of
 * Valinor`, the Rings of Power establishing shot and Peter Xavier
 * Price's `The Two Trees` the same split holds: Laurelin is broad,
 * domed and spreading, carried on a stout low-forking trunk; Telperion
 * is narrower, finer and more upright, on a long clean trunk under a
 * lighter crown. Both are of enormous size, and every artist
 * establishes that the same way - something tiny at the foot. The
 * harness's 1.8 m figure is that something, which is why both presets
 * are stated at the scale of a landscape feature rather than at the
 * scale of an oak.
 *
 * WHERE THEY DISAGREE, and how it is settled here. The Rings of Power
 * and Price put Laurelin's mass in its trunk and limbs; Garland draws
 * the gold tree slender and its trunk visibly PLAITED, two strands
 * wound about each other, while his silver tree is smooth-skinned.
 * The silhouette follows the majority - Laurelin broad and heavy - and
 * the skin follows Garland: Laurelin gets four deep lobes winding
 * slowly, Telperion seven shallow ones winding fast. Deep-and-few
 * reads as rope at a distance; fine-and-many reads as a fluted column,
 * which is the finer of the two and belongs to the finer tree.
 *
 * HOW SIZE IS AUTHORED. The library is scale-free on purpose: every
 * length in it is a fraction of envelope height, so nothing makes a
 * 130 m tree proportionally stouter than a 24 m one. Physics is the
 * preset's business, and the physics is elastic similarity - a
 * self-supporting trunk's diameter goes as height^1.5, so as a
 * fraction of height it goes as height^0.5. From 0.020 at 24 m that
 * puts a 132 m tree at 0.047 and a 148 m tree at 0.050. Laurelin is
 * dialled above its own figure because it is carrying a crown wider
 * than it is tall and the bending moment is the load that decides a
 * trunk; Telperion sits at its figure, carrying almost nothing out to
 * the side.
 *
 * WHAT NOBODY HAS LOOKED AT. Nothing in this file has been judged by
 * eye. It is authored from the references and from the documented
 * range of every parameter, and the owner's gate is the first time it
 * is seen. The seeds are unjudged in particular: a seed varies the
 * detail and not the outcome, so 1 and 2 are two arbitrary draws and
 * not two chosen ones.
 * ------------------------------------------------------------------ */

/**
 * The elder tree. Narrow, upright, finely made: a long bare trunk
 * under a spire of a crown that carries its mass high, with a nervous
 * short-wavelength movement in the limbs and a hard wring of spiral
 * about the axis. Strong gravitropism is what keeps all that movement
 * pointing at the sky - the tree writhes without ever leaving vertical.
 */
export const TELPERION: TreePreset = {
  id: "telperion",
  name: "Telperion",
  note: "the elder: narrow, upright, finely made, wrung about its own axis",
  skeleton: {
    seed: 1,
    envelope: {
      // Slightly the taller of the two, and less than half as wide.
      // Height alone is not what makes it read as upright - the ratio
      // is: 148 m tall against a 71 m crown, where Laurelin's crown is
      // wider than its whole height.
      height: 148,
      // Over a third of the tree is bare trunk, against Laurelin's
      // quarter. That is the strongest upright cue in the references,
      // and it is also what a tree this size does: it sheds its lower
      // limbs.
      crownBase: 0.38,
      spread: 0.24,
      // Mass carried above the middle of the crown.
      fullness: 0.58,
      // Just past a straight-sided cone: a spire with no shoulders.
      shoulder: 1.5,
    },
    /* As many as the generator will use. Telperion's crown is a small
       volume beside Laurelin's, and the growth step is a fixed
       fraction of height, so this crown has room for far fewer nodes
       whatever it is offered - measured, 808 against Laurelin's 2442.
       Filling it is not a matter of asking for more attractors past
       this point; it saturates. Telperion is the finer tree in the
       place fineness is actually visible, which is the tips: its
       median twig comes out at 6.4% of its trunk radius against
       Laurelin's 8.0%, and that is `forkExponent`'s doing. */
    attractors: 1600,
    /* The step every tree was grown at before it was a dial: 3.26 m
       on this tree, and 808 nodes on 133 tips. Stated at the default
       so that Telperion is the tree the owner has already seen, and
       any change to it is a change someone made. Finer is deeper -
       the same crown at 0.003 is 14,100 nodes on 1,200 tips - and
       where this tree ships on that rail is the owner's to say in
       clay. */
    step: 0.022,
    bias: {
      // Nearly the maximum. Everything below bends this tree and this
      // is what will not let it lean away from vertical.
      gravitropism: 0.95,
      lean: 0.04,
      // A short bend length is the difference between a slow S and a
      // tight nervous wave. 0.34 of height is about fifteen growth
      // steps per bend, comfortably above the field's own sampling
      // floor, so it reads as a curve and not as a sawtooth.
      writheAmplitude: 0.11,
      writheWavelength: 0.34,
      // The wrung-out term, and Telperion's alone: two and a half
      // turns about the axis over the height.
      spiralRate: 2.6,
    },
    /* The recursion below colonization's tips, at rest: no orders yet,
       so this is still the tree the owner has seen, and the shipping
       depth is the owner's to state in clay. When it has orders they
       are alternate broadleaf twigs at the botanical defaults twigs.ts
       cites - a leader and one lateral per node, 45 degrees, the
       golden angle, and each order six tenths the length of the one
       above - held to this tree's own 26-degree stiffness. */
    twigs: {
      levels: 0,
      children: 2,
      angle: 45,
      divergence: 137.508,
      internode: 1,
      taper: 0.6,
    },
    // Stiff, at a little over half Laurelin's. A limb that commits to
    // a direction, which is what a narrow crown of long
    // straight-running limbs needs.
    growth: { maxTurnPerStep: 26 },
  },
  radii: {
    // Elastic similarity at 148 m, and no more: this tree carries
    // nothing out to the side.
    trunkRadius: 0.05,
    // Just above area-conserving. Forks shed close to their area, so
    // the run from trunk to twig is long and the twigs come out fine.
    forkExponent: 2.15,
    // The fine orders' own thinning, below the crossover only. 0.7 is
    // where this tree's leaf meets its twig at about 25 to 1 at eight
    // orders, measured; area conservation alone gave 2 to 1.
    twigTaper: 0.7,
    // The long bare trunk is the tree's longest unbranched run, so
    // this is most of what its silhouette does below the crown. High
    // enough that the trunk visibly narrows on the way up.
    lengthTaper: 0.75,
  },
  surface: {
    radialSegments: 12,
    // Many shallow lobes: a fluted column, the finer skin of the two.
    // Seven lobes raise the section to 28 vertices on their own.
    lobes: 7,
    lobeDepth: 0.11,
    // Winding fast, to match the centreline's own spiral. The two are
    // separate mechanisms and this is the one on the skin.
    twistRate: 2.4,
    // A modest flare. Telperion stands rather than buttresses.
    flareRadius: 2,
    flareFalloff: 0.022,
    flareDepth: 0.004,
    forkSocket: 0.5,
    forkSwell: 1.35,
  },
  canopy: {
    // Its twigs come out at 6.4% of the trunk's radius, so a tenth of
    // the trunk is the last long stretch of young wood on every limb -
    // a narrow tree clothed to some depth rather than tufted at the
    // very ends.
    shootRadius: 0.1,
    // Close-set. Fineness on this tree is visible in the tips, and a
    // fine tip carrying widely spaced leaves reads as a bare stick with
    // things stuck on it.
    spacing: 0.0045,
    // The golden angle, which is what almost every plant does.
    divergence: 137.508,
    clump: 6,
    clumpSpan: 0.28,
    // Held in close. A crown less than half as wide as Laurelin's does
    // not carry its foliage out to the side, and the upward term is
    // the same argument gravitropism makes about the limbs: this tree
    // points at the sky.
    outward: 0.42,
    upward: 0.55,
    // The least disorder of the two. Telperion is the finely made one
    // and the arrangement should survive being looked at.
    scatter: 14,
    // The element at the size it is authored. Leaf size is the
    // element's own business; this dial only says whether a tree
    // wants more or less of it, and Telperion wants the smaller.
    size: 1,
    sizeVariation: 0.28,
  },
};

/**
 * The younger tree. Broad, domed, spreading: a stout trunk that forks
 * low into a crown wider than the tree is tall, with heavy limbs that
 * hold their thickness far out and sweep in long slow arcs rather than
 * writhing. Everything about it is mass carried sideways.
 */
export const LAURELIN: TreePreset = {
  id: "laurelin",
  name: "Laurelin",
  note: "the younger: broad, domed, spreading, mass carried sideways",
  skeleton: {
    seed: 2,
    envelope: {
      height: 132,
      // Forks low. The crown starts at under a quarter of the height
      // and the dome comes down almost to the buttress.
      crownBase: 0.24,
      // 153 m across on a 132 m tree: wider than it is tall, which is
      // what the references show and what no ordinary tree does.
      spread: 0.58,
      // Widest below the middle of the crown - bottom-heavy and
      // spreading rather than lifted.
      fullness: 0.38,
      // Well past an ellipse: the shoulders square off into a dome.
      // This one number is most of the difference in silhouette.
      shoulder: 3.2,
      // fullness + shoulder together are the domed profile; spread is
      // only how far it reaches.
    },
    /* Fewer than Telperion is offered, spread through a crown volume
       many times larger, so they sit further apart and the limbs
       between them run further before they fork. The tree still ends
       up with three times the nodes - a crown this size simply holds
       more - and what the sparser scatter buys is that they are long
       heavy limbs rather than twigs. */
    attractors: 1060,
    /* The default step, for the reason Telperion states it: this is
       the tree that has been seen. 2.9 m here, and 2,442 nodes on 376
       tips; at 0.003 the same crown is 23,800 nodes on 880 tips, and
       a crown this size is where the rail costs most. */
    step: 0.022,
    bias: {
      // Lower than Telperion's on purpose: the upward pull is what
      // stops limbs going out and down, and going out is the whole
      // character of this tree.
      gravitropism: 0.55,
      lean: 0.06,
      // Little writhe, over a bend length nearly two and a half times
      // Telperion's: one long slow arc across the whole tree rather
      // than a nervous wave. Heavy limbs sweep, they do not twitch.
      writheAmplitude: 0.05,
      writheWavelength: 0.8,
      // Barely half a turn over the height. The plait on this tree is
      // in the skin, not in the centreline.
      spiralRate: 0.6,
    },
    /* Zero orders for the reason Telperion states it. The same
       resting twig as Telperion's: what differs between the two trees
       at twig scale is the field that bends them and the 46-degree
       looseness that lets a lateral leave at its full angle, not the
       rule that makes them. */
    twigs: {
      levels: 0,
      children: 2,
      angle: 45,
      divergence: 137.508,
      internode: 1,
      taper: 0.6,
    },
    // Loose and searching, so the crown reaches out to the far edge of
    // a very wide envelope instead of driving straight up through it.
    growth: { maxTurnPerStep: 46 },
  },
  radii: {
    // Above elastic similarity's 0.047 at this height, because a crown
    // wider than the tree is tall loads the trunk in bending and
    // bending is what sizes a trunk. Beside the 1.8 m figure this is
    // about 7 m through at the foot.
    trunkRadius: 0.055,
    // Well above area-conserving: a fork sheds less than its area and
    // the limbs stay stout a long way out. This is what makes the
    // dome read as built out of limbs rather than out of twigs.
    forkExponent: 2.7,
    // Low. The short bare trunk is a column that keeps its girth right
    // up to the first fork.
    lengthTaper: 0.35,
    // Same thinning exponent as Telperion: its own fork exponent is what
    // keeps Laurelin's twigs stouter (about 15 to 1 at eight orders).
    twigTaper: 0.7,
  },
  surface: {
    radialSegments: 12,
    // Few deep lobes: Garland's plaited gold trunk. Four strands cut a
    // quarter of the radius deep read as rope at a distance, where
    // Telperion's seven shallow ones read as flutes.
    lobes: 4,
    lobeDepth: 0.24,
    // Winding slowly, so the plait is legible on a trunk this thick
    // instead of blurring into chatter.
    twistRate: 0.8,
    // A heavy buttress. The references all put Laurelin's trunk into
    // the ground rather than on it.
    flareRadius: 3.1,
    flareFalloff: 0.022,
    flareDepth: 0.004,
    forkSocket: 0.5,
    forkSwell: 1.35,
  },
  canopy: {
    // A fork on this tree sheds less than its area, so the wood is
    // still stout where Telperion's has gone to twig. The threshold
    // rises with it or the canopy would find almost nothing to sit on.
    shootRadius: 0.14,
    // Wider set, on heavier wood, under bigger leaves.
    spacing: 0.0065,
    // Not the golden angle: the Lucas angle, the other divergence real
    // plants use. It closes the spiral in five turns rather than
    // eight, so leaves gather into blockier ranks - which is the
    // massed, domed reading, where Telperion's is the open one. Same
    // mechanism, a different number, which is the whole rule these two
    // trees are authored under.
    divergence: 99.502,
    // Heavy tufts at the ends of heavy limbs, over a longer stretch of
    // the shoot. This is most of what makes the dome read as solid.
    clump: 9,
    clumpSpan: 0.36,
    // Mass carried sideways, in the foliage as in everything else. The
    // low upward term is the same low gravitropism the limbs have.
    outward: 0.72,
    upward: 0.22,
    // Looser. A broad tree read from further back wants its disorder
    // where the eye can find it.
    scatter: 22,
    // Half again Telperion's. Broad and heavy at every scale it has
    // one, and that reads at the leaf as much as at the limb.
    size: 1.5,
    sizeVariation: 0.4,
  },
};

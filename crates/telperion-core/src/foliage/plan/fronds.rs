//! A frond crown in the plan. Fronds are placements at the stem apices, not
//! wood, so no run describes them: each frond, living or dead, is cut into a
//! few chords along its arched rachis, on the spiral, frame and scale the
//! placement hangs it at (`rosette::fronds`, `leaflet::leaflets`), and each
//! chord is described by one ribbon, the flat box its leaflets fan in.
//!
//! The ribbon is fitted, not guessed. Its segment runs along the chord, its
//! side lies square to it in the frond's plane and its thickness is taken
//! square to both. Every leaflet is drawn on the frond's own stream exactly
//! as placement draws it (`leaflet::drawn`), and each corner of the leaf's
//! box is bounded along those three directions. The leaflets' V about the
//! frond's plane, their droop and their scatter all set the thickness. A
//! margin covers what storing a leaf in twelve bytes moves it by.
use super::Descriptor;
use crate::{
    foliage::{
        leaflet::{self, Leaflet},
        rosette::{self, frame, Frond},
        CanopyParams, Element,
    },
    math::Vec3,
    rng::Rng,
    tree::Tree,
    Error, Result,
};

/// Chords a rachis is cut into.
const CHORDS: usize = 3;

/// What packing may move a leaf corner by: a centimetre, and a hundredth of
/// its distance from the attachment for the rotation's ten-bit codes.
const MARGIN: (f64, f64) = (0.01, 0.01);

/// Appends every rosette's fronds and returns the leaves they carry, before
/// any cull. `system` names each node's limb system, a frond taking its
/// apex's. Over the instance budget is an error, as placement's count is.
pub(super) fn describe(
    tree: &Tree,
    p: CanopyParams,
    element: &Element,
    seed: u32,
    system: &[u32],
    out: &mut Vec<Descriptor>,
) -> Result<u32> {
    if tree.nodes.len() < 2 || p.size == 0.0 {
        return Ok(0);
    }
    let apices = rosette::rosettes(tree);
    let per = rosette::leaflets(&p);
    let fronds = (p.rosette_fronds + rosette::skirt(&p)) as usize;
    let total = apices.len() * fronds * per;
    if total > p.max_instances || total > u32::MAX as usize {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    let chords = if per > 1 { CHORDS } else { 1 };
    out.try_reserve(apices.len() * fronds * chords)
        .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
    let corners = corners(element);
    for apex in &apices {
        let system = system[apex.apex];
        let birth = tree.nodes[apex.apex].identity.birth_order();
        for frond in rosette::fronds(apex, &p) {
            if per == 1 {
                // One blade at the station: the capsule of its reach.
                let reach = corners.iter().map(|c| c.length()).fold(0.0, f64::max);
                out.push(Descriptor {
                    endpoints: [frond.at; 2],
                    radii: [reach * frond.canopy.size * (1.0 + p.size_variation); 2],
                    count: 1,
                    system,
                    side: Vec3::ZERO,
                });
                continue;
            }
            let mut rng = rosette::stream(seed, birth, &frond);
            ribbons(&frond, per, &corners, &mut rng, system, out);
        }
    }
    Ok(total as u32)
}

/// The eight corners of the leaf element's box, in its own frame.
fn corners(element: &Element) -> Vec<Vec3> {
    let first = element.positions.first().copied().unwrap_or(Vec3::ZERO);
    let (lo, hi) = element
        .positions
        .iter()
        .fold((first, first), |(lo, hi), v| {
            (
                Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
                Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
            )
        });
    (0..8)
        .map(|i| {
            let pick = |bit: usize, lo: f64, hi: f64| if i & bit == 0 { lo } else { hi };
            Vec3::new(
                pick(1, lo.x, hi.x),
                pick(2, lo.y, hi.y),
                pick(4, lo.z, hi.z),
            )
        })
        .collect()
}

/// One frond's ribbons: on each chord, one for each row of leaflets.
fn ribbons(
    frond: &Frond,
    per: usize,
    corners: &[Vec3],
    rng: &mut Rng,
    system: u32,
    out: &mut Vec<Descriptor>,
) {
    let c = frond.canopy;
    let (lift, side) = frame(frond.heading);
    let point = |t: f64| leaflet::rachis(frond.at, frond.heading, lift, &c, t);
    let mut rows: [Row; 2 * CHORDS] = std::array::from_fn(|r| {
        let j = r / 2;
        let from = point(j as f64 / CHORDS as f64);
        let along = (point((j + 1) as f64 / CHORDS as f64) - from).normalized();
        Row {
            from,
            along,
            side,
            corners: Vec::new(),
            count: 0,
        }
    });
    for (i, leaf) in leaflet::leaflets(frond.at, frond.heading, c, per).enumerate() {
        // Leaflet `i` stands at `(i + 1) / per` along the rachis, on the
        // chord whose span of `1 / CHORDS` holds that point, in the row of
        // its side; the rows alternate as the leaflets do.
        let chord = ((i + 1) * CHORDS - 1) / per;
        let drawn = leaflet::drawn(leaf.axis, leaf.run, side, &c, rng);
        rows[2 * chord + i % 2].add(&leaf, drawn, corners);
    }
    out.extend(
        rows.iter()
            .filter(|r| r.count > 0)
            .map(|r| r.ribbon(system)),
    );
}

/// One row of one chord: the chord's start and direction, the frond's side,
/// and every corner of its leaflets' boxes, from the start, with the margin
/// each carries.
struct Row {
    from: Vec3,
    along: Vec3,
    side: Vec3,
    corners: Vec<(Vec3, f64)>,
    count: u32,
}

/// The rolls about the chord a row's plane is tried at: a row of leaflets
/// leaves the frond's plane in a V, each row tilted its own way.
const ROLLS: [f64; 13] = [
    -60., -50., -40., -30., -20., -10., 0., 10., 20., 30., 40., 50., 60.,
];

impl Row {
    /// Keeps every corner of one drawn leaflet's box.
    fn add(&mut self, leaf: &Leaflet, drawn: ([Vec3; 3], f64), corners: &[Vec3]) {
        let ([flank, axis, face], scale) = drawn;
        let k = scale * leaf.share.unwrap_or(1.0);
        for corner in corners {
            let q = (flank * corner.x + axis * corner.y + face * corner.z) * k;
            let slack = MARGIN.0 + MARGIN.1 * q.length();
            self.corners.push((leaf.at + q - self.from, slack));
        }
        self.count += 1;
    }
    /// The corners' spans along the chord, across it in a plane rolled by
    /// `roll` degrees from the frond's, and square to both.
    fn spans(&self, roll: f64) -> ([Vec3; 3], [[f64; 2]; 3]) {
        let side = self.side.rotate(self.along, roll.to_radians());
        let axes = [self.along, side, self.along.cross(side)];
        let mut span = [[f64::INFINITY, f64::NEG_INFINITY]; 3];
        for &(at, slack) in &self.corners {
            for (axis, range) in axes.iter().zip(&mut span) {
                let x = at.dot(*axis);
                range[0] = range[0].min(x - slack);
                range[1] = range[1].max(x + slack);
            }
        }
        (axes, span)
    }
    /// The thinnest ribbon over the tried rolls that holds every corner.
    fn ribbon(&self, system: u32) -> Descriptor {
        let (axes, span) = ROLLS
            .iter()
            .map(|&roll| self.spans(roll))
            .min_by(|a, b| (a.1[2][1] - a.1[2][0]).total_cmp(&(b.1[2][1] - b.1[2][0])))
            .expect("rolls are tried");
        let [along, side, across] = axes;
        let mid = |r: [f64; 2]| (r[0] + r[1]) / 2.0;
        let half = |r: [f64; 2]| (r[1] - r[0]) / 2.0;
        let centre = self.from + side * mid(span[1]) + across * mid(span[2]);
        Descriptor {
            endpoints: [centre + along * span[0][0], centre + along * span[0][1]],
            radii: [half(span[2]); 2],
            count: self.count,
            system,
            side: side * half(span[1]),
        }
    }
}

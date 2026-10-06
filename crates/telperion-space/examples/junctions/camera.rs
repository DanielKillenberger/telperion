//! The refinement rule and the cameras of the junction example: a chain
//! is resolved once its bearer is and its bud's extent bound, a sphere
//! about the bud, is in view and spans more than half a pixel at 960 by
//! 720 under a 45 degree vertical field.
use std::time::Instant;
use telperion_space::{Bounded, Expected, Request, Species, Vec3};

const HEIGHT: f64 = 720.0;
const ASPECT: f64 = 960.0 / 720.0;
const HALF_FIELD: f64 = 22.5 * std::f64::consts::PI / 180.0;
const FRAMES: usize = 120;

#[derive(Debug, Clone, Copy)]
pub struct Cam {
    pub position: Vec3,
    pub forward: Vec3,
}

impl Cam {
    /// Whether a sphere about `centre` of `radius` is in view and spans
    /// more than half a pixel.
    fn needs(&self, centre: Vec3, radius: f64) -> bool {
        let v = centre - self.position;
        let distance = v.length();
        if distance <= radius {
            return true;
        }
        let across = (radius / distance).asin();
        let off = (v.dot(self.forward) / distance).clamp(-1.0, 1.0).acos();
        let diagonal = (HALF_FIELD.tan() * (1.0 + ASPECT * ASPECT).sqrt()).atan();
        off - across < diagonal && across.tan() / HALF_FIELD.tan() * HEIGHT / 2.0 > 0.5
    }
}

/// The fly-in's cameras: from 40 m to 5 cm from the tip of the finest
/// chain of the highest order, geometrically over 120 frames, from the
/// side it stands out to.
pub fn fly_in(tree: &Bounded, _seed: u64) -> Vec<Cam> {
    let s = tree.structure();
    let top = (0..tree.chains()).map(|c| tree.order(c)).max().unwrap_or(0);
    let twig = (0..tree.chains())
        .filter(|&c| tree.order(c) == top && tree.phytomers(c) > 0)
        .min_by_key(|&c| tree.phytomers(c))
        .expect("a twig");
    let target = tree
        .axes(twig)
        .iter()
        .rev()
        .find_map(|&i| s.axes[i].phytomers.last())
        .expect("wood")
        .tip;
    let out = Vec3::new(target.x, target.y, 0.0)
        .unit()
        .unwrap_or(Vec3::new(1.0, 0.0, 0.0));
    let dir = (out + Vec3::new(0.0, 0.0, 0.3))
        .unit()
        .expect("a direction");
    (0..FRAMES)
        .map(|f| {
            let d = 40.0 * (0.05f64 / 40.0).powf(f as f64 / (FRAMES - 1) as f64);
            Cam {
                position: target + dir * d,
                forward: dir * -1.0,
            }
        })
        .collect()
}

/// Resolves every chain `cam` needs, bearers first, calling `done` for
/// each: the chains and phytomers resolved and the milliseconds taken.
pub fn refine(
    tree: &mut Bounded,
    cam: &Cam,
    frontier: &mut Vec<usize>,
    children: &[Vec<usize>],
    done: &mut dyn FnMut(&Bounded, usize),
) -> (usize, usize, f64) {
    let started = Instant::now();
    let (mut chains, mut phytomers) = (0, 0);
    loop {
        let (now, later): (Vec<usize>, Vec<usize>) = frontier.iter().partition(|&&c| {
            let (base, reach) = tree.bud(c).expect("bearer resolved");
            cam.needs(base, reach)
        });
        if now.is_empty() {
            break;
        }
        *frontier = later;
        for c in now {
            tree.resolve(c).expect("resolves");
            done(tree, c);
            chains += 1;
            phytomers += tree.phytomers(c);
            frontier.extend(&children[c]);
        }
    }
    (chains, phytomers, started.elapsed().as_secs_f64() * 1e3)
}

/// Each chain's chains borne.
pub fn children(tree: &Bounded) -> Vec<Vec<usize>> {
    let mut children = vec![Vec::new(); tree.chains()];
    for c in 1..tree.chains() {
        children[tree.parent(c).expect("borne")].push(c);
    }
    children
}

/// The fly-in on a fresh tree: the first frame, the worst frame after it
/// and the rest's total, and the share of phytomers resolved.
pub struct Flight {
    pub first: (usize, usize, f64),
    pub worst: (usize, usize, f64),
    pub rest: f64,
    pub share: f64,
}

pub fn fly(species: &Species, request: Request, expected: &Expected, path: &[Cam]) -> Flight {
    let mut tree = Bounded::with(species, request, expected).expect("settles");
    let children = children(&tree);
    tree.resolve(0).expect("the trunk resolves");
    let mut frontier = children[0].clone();
    let mut frames = Vec::with_capacity(path.len());
    for cam in path {
        frames.push(refine(
            &mut tree,
            cam,
            &mut frontier,
            &children,
            &mut |_, _| {},
        ));
    }
    let total: usize = (0..tree.chains()).map(|c| tree.phytomers(c)).sum();
    let resolved: usize = (0..tree.chains())
        .filter(|&c| tree.is_resolved(c))
        .map(|c| tree.phytomers(c))
        .sum();
    let worst = frames[1..]
        .iter()
        .copied()
        .max_by(|a, b| a.2.total_cmp(&b.2))
        .unwrap_or_default();
    Flight {
        first: frames[0],
        worst,
        rest: frames[1..].iter().map(|f| f.2).sum(),
        share: resolved as f64 / total.max(1) as f64,
    }
}

/// The chains and phytomers the hero view resolves on a resolved tree:
/// the whole tree framed from 1.3 times the distance its larger extent
/// fills the field at, level with its middle.
pub fn hero(tree: &Bounded) -> (usize, usize) {
    let s = tree.structure();
    let mut low = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut high = Vec3::new(-f64::MAX, -f64::MAX, -f64::MAX);
    for p in s.axes.iter().flat_map(|a| &a.phytomers) {
        low = Vec3::new(low.x.min(p.tip.x), low.y.min(p.tip.y), low.z.min(p.tip.z));
        high = Vec3::new(
            high.x.max(p.tip.x),
            high.y.max(p.tip.y),
            high.z.max(p.tip.z),
        );
    }
    let size = (high.z - low.z).max(high.x - low.x).max(high.y - low.y);
    let centre = (low + high) * 0.5;
    let distance = 1.3 * size / 2.0 / HALF_FIELD.tan();
    let cam = Cam {
        position: centre + Vec3::new(distance, 0.0, 0.0),
        forward: Vec3::new(-1.0, 0.0, 0.0),
    };
    let children = children(tree);
    let (mut chains, mut phytomers) = (1, tree.phytomers(0));
    let mut frontier = children[0].clone();
    while let Some(c) = frontier.pop() {
        let (base, reach) = tree.bud(c).expect("resolved");
        if cam.needs(base, reach) {
            chains += 1;
            phytomers += tree.phytomers(c);
            frontier.extend(&children[c]);
        }
    }
    (chains, phytomers)
}

/// A fixed camera path, for refining in the fly-in's order.
pub struct View {
    pub path: Vec<Cam>,
}

impl View {
    pub fn new(path: Vec<Cam>) -> Self {
        Self { path }
    }
}

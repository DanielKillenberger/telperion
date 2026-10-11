//! The volumes' hierarchy (DESIGN.md section 3): one node an axis whose
//! subtree holds leaf area this cycle. A node holds its own group's kernel
//! (the leaves its axis laid this cycle), its subtree's aggregate kernel
//! and a bound about the aggregate's centre that holds every kernel below
//! it. A bud reads the hierarchy from the root: a node whose bound no ray
//! meets adds nothing; one far for its reach stands as its aggregate; one
//! near is opened into its own group and its children; between, the two
//! are blended, so the light moves by degree as the node's moments do.
use super::{Kernel, Moments};
use crate::structure::Vec3;
use crate::transcendental::Transcendental;

/// The candidate's settings: a leaf's reach in its PA's leaf sizes, and
/// the refinement interval, in the aggregate's reaches from its centre,
/// across which a node goes from opened (`near`) to whole (`far`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pub leaf: f64,
    pub near: f64,
    pub far: f64,
}

impl Default for Candidate {
    fn default() -> Self {
        Self {
            leaf: 1.0,
            near: 1.5,
            far: 3.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    whole: Kernel,
    own: Option<Kernel>,
    bound: f64,
    children: (u32, u32),
}

/// A cycle's leaves as volumes, read by every bud of the next cycle.
pub(crate) struct Volumes {
    nodes: Vec<Node>,
    children: Vec<u32>,
    near: f64,
    far: f64,
}

impl Volumes {
    /// The volumes of `leaves` (each a point, its area, the axis that laid
    /// it and its own variance) on the axes `parents` links, each parent
    /// before its children.
    pub fn build(candidate: Candidate, leaves: &[(Vec3, f64, usize, f64)], parents: &[Option<usize>]) -> Self {
        let n = parents.len();
        let mut own = vec![Moments::default(); n];
        for &(at, area, axis, s2) in leaves {
            own[axis].add(&Moments::leaf(at, area, s2));
        }
        let mut sub = own.clone();
        for i in (0..n).rev() {
            if let Some(p) = parents[i] {
                debug_assert!(p < i, "a parent before its children");
                let s = sub[i];
                sub[p].add(&s);
            }
        }
        // A node an axis whose subtree's aggregate stands, parents first.
        let wholes: Vec<Option<Kernel>> = sub.iter().map(Moments::kernel).collect();
        let mut node = vec![u32::MAX; n];
        let mut axes = Vec::new();
        for i in 0..n {
            if wholes[i].is_some() {
                node[i] = axes.len() as u32;
                axes.push(i);
            }
        }
        let mut count = vec![0u32; axes.len() + 1];
        for &i in &axes {
            if let Some(p) = parents[i].filter(|&p| node[p] != u32::MAX) {
                count[node[p] as usize + 1] += 1;
            }
        }
        for k in 1..count.len() {
            count[k] += count[k - 1];
        }
        let mut fill = count.clone();
        let mut children = vec![0u32; *count.last().unwrap_or(&0) as usize];
        for &i in &axes {
            if let Some(p) = parents[i].filter(|&p| node[p] != u32::MAX) {
                let at = &mut fill[node[p] as usize];
                children[*at as usize] = node[i];
                *at += 1;
            }
        }
        let mut nodes: Vec<Node> = (axes.iter().enumerate())
            .map(|(k, &i)| Node {
                whole: wholes[i].expect("a node's aggregate"),
                own: own[i].kernel(),
                bound: 0.0,
                children: (count[k], count[k + 1]),
            })
            .collect();
        for k in (0..nodes.len()).rev() {
            let Node { whole, own, children: (from, to), .. } = nodes[k];
            let reach = |c: Vec3, r: f64| (c - whole.centre).length() + r;
            let mut bound = whole.reach;
            if let Some(o) = own {
                bound = bound.max(reach(o.centre, o.reach));
            }
            for &c in &children[from as usize..to as usize] {
                let c = &nodes[c as usize];
                bound = bound.max(reach(c.whole.centre, c.bound));
            }
            nodes[k].bound = bound;
        }
        Self {
            nodes,
            children,
            near: candidate.near,
            far: candidate.far,
        }
    }

    /// The nodes it holds.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// The light at `at` under `directions` (unit vectors and shares) and
    /// `extinction`, and the nodes the read visited.
    pub fn read(&self, at: Vec3, directions: &[(Vec3, f64)], extinction: f64) -> (f64, u32) {
        let mut depth = [0.0; 16];
        let mut visited = 0;
        if !self.nodes.is_empty() {
            self.depth(0, at, directions, 1.0, &mut depth, &mut visited);
        }
        (lit(directions, &depth, extinction), visited)
    }

    /// The light at `at` read from every axis's own group, no aggregate
    /// standing for any: the hierarchy's reference.
    pub fn exhaustive(&self, at: Vec3, directions: &[(Vec3, f64)], extinction: f64) -> f64 {
        let mut depth = [0.0; 16];
        for own in self.nodes.iter().filter_map(|n| n.own) {
            for (d, &(toward, _)) in depth.iter_mut().zip(directions) {
                *d += own.along(at, toward);
            }
        }
        lit(directions, &depth, extinction)
    }

    fn depth(&self, k: u32, at: Vec3, directions: &[(Vec3, f64)], weight: f64, depth: &mut [f64; 16], visited: &mut u32) {
        *visited += 1;
        let node = &self.nodes[k as usize];
        let centre = node.whole.centre;
        if !directions.iter().any(|d| meets(at, d.0, centre, node.bound)) {
            return;
        }
        let whole = blend((at - centre).length() / node.whole.reach, self.near, self.far);
        if whole > 0.0 {
            for (d, &(toward, _)) in depth.iter_mut().zip(directions) {
                *d += weight * whole * node.whole.along(at, toward);
            }
        }
        if whole >= 1.0 {
            return;
        }
        let open = weight * (1.0 - whole);
        if let Some(own) = node.own {
            for (d, &(toward, _)) in depth.iter_mut().zip(directions) {
                *d += open * own.along(at, toward);
            }
        }
        let (from, to) = node.children;
        for &c in &self.children[from as usize..to as usize] {
            self.depth(c, at, directions, open, depth, visited);
        }
    }
}

/// The light through each direction's depth of leaf area, held to 0 to 1.
fn lit(directions: &[(Vec3, f64)], depth: &[f64; 16], extinction: f64) -> f64 {
    let sum: f64 = (directions.iter().zip(depth))
        .map(|(&(_, share), &d)| share * (-extinction * d).exp_fixed())
        .sum();
    sum.clamp(0.0, 1.0)
}

/// How much a node stands whole at `r` of its reaches from its centre: 0
/// within `near`, 1 beyond `far`, a smoothstep between.
fn blend(r: f64, near: f64, far: f64) -> f64 {
    let t = ((r - near) / (far - near)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Whether the ray from `at` towards unit `toward` meets the ball of
/// `radius` about `centre`.
fn meets(at: Vec3, toward: Vec3, centre: Vec3, radius: f64) -> bool {
    let v = centre - at;
    let (far, along) = (v.dot(v), v.dot(toward));
    far <= radius * radius || (along >= 0.0 && far - along * along <= radius * radius)
}

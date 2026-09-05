//! Frozen surface algorithm, f64 arithmetic and owned f32/u32 outputs.
use std::ops::{Add, Sub};
#[derive(Clone, Copy, Default)]
struct V(f64, f64, f64);
impl Add for V {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        V(self.0 + b.0, self.1 + b.1, self.2 + b.2)
    }
}
impl Sub for V {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        V(self.0 - b.0, self.1 - b.1, self.2 - b.2)
    }
}
impl V {
    fn scale(self, s: f64) -> Self {
        V(self.0 * s, self.1 * s, self.2 * s)
    }
    fn dot(self, b: Self) -> f64 {
        self.0 * b.0 + self.1 * b.1 + self.2 * b.2
    }
    fn cross(self, b: Self) -> Self {
        V(
            self.1 * b.2 - self.2 * b.1,
            self.2 * b.0 - self.0 * b.2,
            self.0 * b.1 - self.1 * b.0,
        )
    }
    fn norm(self) -> Self {
        let len = self.dot(self).sqrt();
        self.scale(1.0 / if len == 0.0 { 1.0 } else { len })
    }
    fn axis(self) -> Self {
        let (x, y, z) = (self.0.abs(), self.1.abs(), self.2.abs());
        if x <= y && x <= z {
            V(1.0, 0.0, 0.0)
        } else if y <= z {
            V(0.0, 1.0, 0.0)
        } else {
            V(0.0, 0.0, 1.0)
        }
    }
    fn rotate(self, from: Self, to: Self) -> Self {
        let mut w = from.dot(to) + 1.0;
        let mut q = if w < 1e-8 {
            w = 0.0;
            if from.0.abs() > from.2.abs() {
                V(-from.1, from.0, 0.0)
            } else {
                V(0.0, -from.2, from.1)
            }
        } else {
            from.cross(to)
        };
        let inv = 1.0 / (q.dot(q) + w * w).sqrt();
        q = q.scale(inv);
        w *= inv;
        let t = q.cross(self).scale(2.0);
        // Match THREE.Vector3.applyQuaternion's operation order.
        V(
            self.0 + w * t.0 + q.1 * t.2 - q.2 * t.1,
            self.1 + w * t.1 + q.2 * t.0 - q.0 * t.2,
            self.2 + w * t.2 + q.0 * t.1 - q.1 * t.0,
        )
    }
}
#[derive(Clone, Copy)]
struct Node {
    p: V,
    parent: i32,
    r: f64,
    start: f64,
}
struct Path {
    nodes: Vec<usize>,
    trunk: bool,
}
fn paths(nodes: &[Node]) -> Vec<Path> {
    let n = nodes.len();
    if n < 2 {
        return vec![];
    }
    let mut stands = vec![0; n];
    let mut children = vec![vec![]; n];
    for i in 1..n {
        if nodes[i].parent < 0 {
            stands[i] = i;
            continue;
        }
        let at = stands[nodes[i].parent as usize];
        let d = nodes[i].p - nodes[at].p;
        if d.dot(d).sqrt() > 1e-9 {
            stands[i] = i;
            children[at].push(i);
        } else {
            stands[i] = at;
        }
    }
    let leader = |at: usize| {
        let kids = &children[at];
        let mut best = kids[0];
        for &k in &kids[1..] {
            if nodes[k].start > nodes[best].start {
                best = k;
            }
        }
        best
    };
    let mut seeds = vec![];
    if !children[0].is_empty() {
        seeds.push((0, leader(0)));
    }
    for &k in &children[0] {
        if Some(k) != seeds.first().map(|s| s.1) {
            seeds.push((0, k));
        }
    }
    let mut result = vec![];
    let mut s = 0;
    while s < seeds.len() {
        let (attach, first) = seeds[s];
        let mut run = vec![attach, first];
        let mut at = first;
        while !children[at].is_empty() {
            let next = leader(at);
            for &k in &children[at] {
                if k != next {
                    seeds.push((at, k));
                }
            }
            run.push(next);
            at = next;
        }
        result.push(Path {
            nodes: run,
            trunk: s == 0,
        });
        s += 1;
    }
    result
}
#[derive(Clone, Copy)]
struct Sample {
    p: V,
    r: f64,
    d: f64,
}
fn frames(samples: &[Sample]) -> Vec<(V, V)> {
    let n = samples.len();
    let mut segments: Vec<V> = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        let step = samples[i + 1].p - samples[i].p;
        segments.push(if step.dot(step) > 0.0 {
            step.norm()
        } else {
            segments.last().copied().unwrap_or(V(0.0, 1.0, 0.0))
        });
    }
    let mut tangents = Vec::with_capacity(n);
    for i in 0..n {
        tangents.push(if i == 0 {
            segments[0]
        } else if i == n - 1 {
            segments[n - 2]
        } else {
            let mean = segments[i - 1] + segments[i];
            if mean.dot(mean) > 1e-9 {
                mean.norm()
            } else {
                segments[i]
            }
        });
    }
    let mut normal = tangents[0].axis().cross(tangents[0]).norm();
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let t = tangents[i];
        if i > 0 {
            normal = normal.rotate(tangents[i - 1], t);
        }
        normal = normal + t.scale(-normal.dot(t));
        if normal.dot(normal) <= 1e-9 {
            normal = t.axis().cross(t);
        }
        normal = normal.norm();
        result.push((normal, t.cross(normal).norm()));
    }
    result
}
#[derive(Default)]
pub struct Mesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
}
fn held(v: f64, fallback: f64) -> f64 {
    if v.is_finite() {
        v
    } else {
        fallback
    }
}
fn vertex(out: &mut Vec<f32>, p: V) {
    out.extend([p.0 as f32, p.1 as f32, p.2 as f32]);
}
/// Trusted experiment ABI: [count,height,radial,lobes,depth,twist,flare,falloff,
/// burial,socket,swell, then count*(parent,x,y,z,radius,startRadius)].
pub fn build(input: &[f64]) -> Mesh {
    assert!(input.len() >= 11);
    let n = input[0] as usize;
    assert_eq!(input.len(), 11 + n * 6);
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        let j = 11 + i * 6;
        let parent = input[j] as i32;
        assert!(parent < 0 || (parent as usize) < i);
        nodes.push(Node {
            parent,
            p: V(input[j + 1], input[j + 2], input[j + 3]),
            r: input[j + 4],
            start: input[j + 5],
        });
    }
    let paths = paths(&nodes);
    if paths.is_empty() {
        return Mesh::default();
    }
    let height = held(input[1], 24.0).max(1e-6);
    let lobes = held(input[3], 5.0).round().clamp(0.0, 16.0);
    let segments = held(input[2], 12.0)
        .round()
        .max(3.0)
        .max(lobes * 4.0)
        .min(64.0) as usize;
    let depth = held(input[4], 0.16).clamp(0.0, 0.9);
    let twist = held(input[5], 1.5).clamp(-64.0, 64.0);
    let flare_radius = held(input[6], 2.1).clamp(1.0, 8.0);
    let falloff = held(input[7], 0.022).clamp(1e-4, 1.0) * height;
    let burial = held(input[8], 0.004).clamp(0.0, 1.0) * height;
    let socket = held(input[9], 0.5).clamp(0.0, 0.9);
    let swell = held(input[10], 1.35).clamp(1.0, 4.0);
    let mut distance = vec![0.0; n];
    for i in 1..n {
        let p = nodes[i].parent;
        if p >= 0 {
            let delta = nodes[p as usize].p - nodes[i].p;
            distance[i] = distance[p as usize] + delta.dot(delta).sqrt();
        }
    }
    let flare = |y: f64| 1.0 + (flare_radius - 1.0) * (-y.max(0.0) / falloff).exp();
    let rings: usize = paths
        .iter()
        .map(|p| p.nodes.len() + usize::from(p.trunk && burial > 0.0))
        .sum();
    let mut mesh = Mesh {
        positions: Vec::with_capacity((rings * segments + paths.len() * 2) * 3),
        indices: Vec::with_capacity(rings * segments * 6),
    };
    for path in paths {
        let mut samples = Vec::with_capacity(path.nodes.len() + 1);
        if path.trunk {
            let root = nodes[path.nodes[0]];
            if burial > 0.0 {
                samples.push(Sample {
                    p: V(root.p.0, root.p.1 - burial, root.p.2),
                    r: root.r * flare(root.p.1),
                    d: 0.0,
                });
            }
            for &i in &path.nodes {
                samples.push(Sample {
                    p: nodes[i].p,
                    r: nodes[i].r * flare(nodes[i].p.1),
                    d: distance[i],
                });
            }
        } else {
            let attach = path.nodes[0];
            let first = path.nodes[1];
            let pr = nodes[attach].r;
            let away = (nodes[first].p - nodes[attach].p).norm();
            let inscribed = pr * (1.0 - depth) * (std::f64::consts::PI / segments as f64).cos();
            let sink = (socket * pr).min(0.9 * inscribed);
            let contained = (inscribed * inscribed - sink * sink).max(0.0).sqrt() / (1.0 + depth);
            samples.push(Sample {
                p: nodes[attach].p + away.scale(-sink),
                r: (nodes[first].start * swell).min(contained) * flare(nodes[attach].p.1),
                d: distance[attach],
            });
            for &i in &path.nodes[1..] {
                let swelling =
                    1.0 + (swell - 1.0) * (-(distance[i] - distance[attach]) / pr.max(1e-9)).exp();
                samples.push(Sample {
                    p: nodes[i].p,
                    r: nodes[i].r * swelling * flare(nodes[i].p.1),
                    d: distance[i],
                });
            }
        }
        let frame = frames(&samples);
        let base = (mesh.positions.len() / 3) as u32;
        let seg = segments as u32;
        for (i, s) in samples.iter().enumerate() {
            let (normal, binormal) = frame[i];
            let phase = std::f64::consts::TAU * twist * (s.d / height);
            for k in 0..segments {
                let angle = (k as f64 / segments as f64) * std::f64::consts::TAU;
                let profile = if lobes == 0.0 {
                    1.0
                } else {
                    1.0 + depth * (lobes * (angle + phase)).cos()
                };
                let width = s.r * profile;
                vertex(
                    &mut mesh.positions,
                    s.p + (normal.scale(angle.cos()) + binormal.scale(angle.sin())).scale(width),
                );
            }
        }
        for i in 0..samples.len() - 1 {
            let lower = base + i as u32 * seg;
            let upper = lower + seg;
            for k in 0..seg {
                let next = (k + 1) % seg;
                mesh.indices.extend([
                    lower + k,
                    lower + next,
                    upper + k,
                    lower + next,
                    upper + next,
                    upper + k,
                ]);
            }
        }
        let bottom = (mesh.positions.len() / 3) as u32;
        vertex(&mut mesh.positions, samples[0].p);
        let top = bottom + 1;
        vertex(&mut mesh.positions, samples.last().unwrap().p);
        let top_ring = base + (samples.len() as u32 - 1) * seg;
        for k in 0..seg {
            let next = (k + 1) % seg;
            mesh.indices.extend([
                bottom,
                base + next,
                base + k,
                top,
                top_ring + k,
                top_ring + next,
            ]);
        }
    }
    mesh
}
#[cfg(target_arch = "wasm32")]
mod abi {
    use super::*;
    use std::cell::RefCell;
    thread_local! {static INPUT:RefCell<Vec<f64>>=const{RefCell::new(Vec::new())};static OUTPUT:RefCell<Mesh>=RefCell::new(Mesh::default());}
    #[no_mangle]
    pub extern "C" fn input_resize(len: usize) -> *mut f64 {
        INPUT.with(|v| {
            let mut v = v.borrow_mut();
            v.resize(len, 0.0);
            v.as_mut_ptr()
        })
    }
    #[no_mangle]
    pub extern "C" fn compute() {
        INPUT.with(|i| OUTPUT.with(|o| *o.borrow_mut() = build(&i.borrow())));
    }
    #[no_mangle]
    pub extern "C" fn positions_ptr() -> *const f32 {
        OUTPUT.with(|v| v.borrow().positions.as_ptr())
    }
    #[no_mangle]
    pub extern "C" fn positions_len() -> usize {
        OUTPUT.with(|v| v.borrow().positions.len())
    }
    #[no_mangle]
    pub extern "C" fn indices_ptr() -> *const u32 {
        OUTPUT.with(|v| v.borrow().indices.as_ptr())
    }
    #[no_mangle]
    pub extern "C" fn indices_len() -> usize {
        OUTPUT.with(|v| v.borrow().indices.len())
    }
}

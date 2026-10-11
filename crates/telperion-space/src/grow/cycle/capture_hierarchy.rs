//! fn-256 task 2 (DESIGN.md sections 2, 3 and 4): one cycle's balance and
//! allotment captured for the device, over the whole tree. The tree grows
//! through the one cycle to the captured cycle; each laid axis's place and
//! pencil and the living apexes are then taken, `open`, `retire`,
//! `balance_one` and `allot_one` run as `step` runs them, and what they
//! leave is taken: each axis's children in the order the sums take them,
//! the open axes and their sums, each pencil's memory, kept share and
//! size, the apexes shed and the living after. Reals are their bits; a
//! real the CPU holds as an f32's f64 is that f32. An experiment entry
//! point, wired into nothing.
use super::super::sketch::{Pencil, Sketch};
use super::super::{Grower, Options, Request, start};
use super::capture::Capture;
use crate::allocation::TIP;
use crate::error::{Result, refuse};
use crate::species::Species;
use crate::structure::Origin;

const NONE: u32 = u32::MAX;

/// Grows `species` through the one cycle and captures `cycle`'s balance
/// and allotment, with the axes laid at each cycle's start.
pub fn grow_capture_hierarchy_at(
    species: &Species,
    request: Request,
    cycle: u32,
) -> Result<Capture> {
    if cycle == 0 || cycle > request.age {
        return refuse("cycle", "a captured cycle is one the tree grows");
    }
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::default());
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let (mut advanced, mut laid) = (Vec::new(), Vec::new());
    for c in 1..cycle {
        laid.push(g.laid_pencils().len() as u32);
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
    }
    laid.push(g.laid_pencils().len() as u32);
    let mut c = Capture { arrays: Vec::new() };
    let n = g.laid_pencils().len();
    c.u32s(
        "hier_counts",
        &[cycle, request.age, n as u32, g.axes.len() as u32],
    );
    c.u32s("hier_laid", &laid);
    g.capture_places(&mut c, n);
    g.capture_pencils(&mut c, "in");
    g.capture_living(&mut c, "in");
    g.capture_rows(&mut c, cycle);
    g.open(cycle);
    g.retire(cycle);
    g.balance_one(cycle);
    g.allot_one();
    g.capture_sums(&mut c, n);
    g.capture_pencils(&mut c, "out");
    g.capture_living(&mut c, "out");
    Ok(c)
}

/// A real the CPU holds as an f32's f64: that f32.
fn f(x: f64) -> f32 {
    assert_eq!(f64::from(x as f32), x, "a real that is no f32");
    x as f32
}

impl Grower<'_> {
    fn laid_pencils(&self) -> &[Pencil] {
        &self.sketch.as_ref().expect("sketching").pencils
    }

    /// Each laid axis's bearer and node (`NONE` for the seed's bearer and
    /// for a continuation's or relay's node, which stands at the tip), PA
    /// and lineage.
    fn capture_places(&self, c: &mut Capture, n: usize) {
        let (mut parent, mut node) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for a in &self.axes[..n] {
            let (p, j) = match a.origin {
                Origin::Seed => (NONE, NONE),
                Origin::Lateral { parent, node, .. } => (parent as u32, node as u32),
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                    (parent as u32, NONE)
                }
            };
            assert!(j == NONE || (j as usize) < TIP, "a node below the tip");
            parent.push(p);
            node.push(j);
        }
        c.u32s("hier_parent", &parent);
        c.u32s("hier_node", &node);
        let axes = &self.axes[..n];
        c.u32s(
            "hier_pa",
            &axes.iter().map(|a| a.pa as u32).collect::<Vec<_>>(),
        );
        c.u64s(
            "hier_lineage",
            &axes.iter().map(|a| a.lineage).collect::<Vec<_>>(),
        );
        let ends: Vec<u32> = axes
            .iter()
            .map(|a| a.apex_end.map_or(0, |e| e + 1))
            .collect();
        c.u32s("hier_in_apex_end", &ends);
    }

    /// Each laid axis's pencil as the balance and allotment read and
    /// write it, under `prefix`.
    fn capture_pencils(&self, c: &mut Capture, prefix: &str) {
        let p = self.laid_pencils();
        let name = |n: &str| format!("hier_{prefix}_{n}");
        let bits =
            |g: &dyn Fn(&Pencil) -> f64| -> Vec<u64> { p.iter().map(|x| g(x).to_bits()).collect() };
        let small =
            |g: &dyn Fn(&Pencil) -> f64| -> Vec<f32> { p.iter().map(|x| f(g(x))).collect() };
        c.u64s(&name("wood"), &bits(&|x| x.wood));
        c.u64s(&name("light"), &bits(&|x| x.light));
        c.f32s(&name("base_scale"), &small(&|x| x.base_scale));
        c.f32s(&name("running"), &small(&|x| x.running));
        c.f32s(&name("kept"), &small(&|x| x.kept));
        c.f32s(&name("memory"), &small(&|x| x.memory));
        c.f32s(&name("size"), &small(&|x| x.size));
        if prefix == "out" {
            let n = p.len();
            let ends: Vec<u32> = self.axes[..n]
                .iter()
                .map(|a| a.apex_end.map_or(0, |e| e + 1))
                .collect();
            c.u32s("hier_out_apex_end", &ends);
        }
    }

    /// The living apexes in order: each axis and the bits of its time.
    fn capture_living(&self, c: &mut Capture, prefix: &str) {
        let axis: Vec<u32> = self.live.iter().map(|a| a.axis as u32).collect();
        let spent: Vec<u64> = self.live.iter().map(|a| a.spent.to_bits()).collect();
        c.u32s(&format!("hier_{prefix}_live_axis"), &axis);
        c.u64s(&format!("hier_{prefix}_live_spent"), &spent);
    }

    /// Each PA's row as the cycle reads it, every value the f32 the CPU
    /// takes: upkeep, balance hazard, tolerance, the shedding window at the
    /// cycle, apical control and ψ.
    fn capture_rows(&self, c: &mut Capture, cycle: u32) {
        let mut rows = Vec::new();
        for (pa, s) in self.species.states.iter().enumerate() {
            let window = self.rules.windows.window((pa, cycle, None), 1.0);
            rows.extend([
                s.upkeep as f32,
                s.balance_hazard as f32,
                s.tolerance as f32,
                window,
                s.apical_control as f32,
                s.shade_size as f32,
            ]);
        }
        c.f32s("hier_rows", &rows);
    }

    /// What the passes leave in the working set: each axis's children in
    /// order (a start per axis, then the children), the open axes, and
    /// each axis's sums (meaningful at the open ones).
    fn capture_sums(&self, c: &mut Capture, n: usize) {
        let w = self.working.as_ref().expect("a working set");
        let (mut start, mut kids) = (vec![0u32], Vec::new());
        for i in 0..n {
            kids.extend(w.children(i));
            start.push(kids.len() as u32);
        }
        c.u32s("hier_kid_start", &start);
        c.u32s("hier_kids", &kids);
        c.u32s(
            "hier_open",
            &w.list.iter().map(|&i| i as u32).collect::<Vec<_>>(),
        );
        c.f32s("hier_light", &w.light[..n]);
        c.f32s("hier_cost", &w.cost[..n]);
        c.u32s(
            "hier_living",
            &w.living[..n]
                .iter()
                .map(|&l| u32::from(l))
                .collect::<Vec<_>>(),
        );
        c.f32s("hier_sub", &w.sub[..n]);
        c.f32s("hier_incoming", &w.incoming[..n]);
        c.f32s("hier_bud", &w.bud[..n]);
    }
}

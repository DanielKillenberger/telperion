//! fn-256 task 2 (DESIGN.md section 2, step 8): one cycle's light
//! captured for the device. The tree grows through the one cycle to the
//! captured cycle, grows and lays it (`step`, `sketch`), then takes the
//! leaves and the living apexes' tips as `read_light` takes them, and the
//! CPU's box, quanta, densities, lattice and reads. Positions and areas
//! are the f32s the lattice reads; the sweep's per-direction reals are
//! the CPU's own f64s, by their bits. An experiment entry point, wired
//! into nothing.
use super::super::sketch::Sketch;
use super::super::{start, Grower, Options, Request};
use super::capture::Capture;
use crate::error::{refuse, Result};
use crate::light::{Field, CELL};
use crate::species::Species;

/// The light read in `cycle` of a tree of `request.age`: the cycles
/// before it grown whole, then its growth and layout, then its light.
pub fn grow_capture_light_at(species: &Species, request: Request, cycle: u32) -> Result<Capture> {
    if cycle == 0 || cycle >= request.age {
        return refuse("cycle", "a lit cycle is one before the tree's age");
    }
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::default());
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut advanced = Vec::new();
    for c in 1..cycle {
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
    }
    assert!(
        g.couple.is_none() && g.fine.is_none(),
        "the whole tree, lit"
    );
    advanced.clear();
    advanced.extend(g.live.iter().map(|a| a.axis));
    g.step(cycle, &mut |_| {})?;
    g.sketch(cycle, &advanced)?;
    let mut c = Capture { arrays: Vec::new() };
    c.u32s("light_cycle", &[cycle]);
    g.capture_light_inputs(&mut c);
    g.capture_light_outputs(&mut c)?;
    Ok(c)
}

impl Grower<'_> {
    /// The living apexes' tips, in the live order `shade` reads them.
    fn lit_tips(&self) -> Vec<crate::structure::Vec3> {
        self.live.iter().map(|a| self.tip(a.axis)).collect()
    }

    /// The leaves (x, y, z, area) and tips (x, y, z) as the lattice
    /// reads them, in f32; each lit direction's `x/z`, `y/z`, its path
    /// `CELL/z` and its share, and the extinction, by their f64 bits.
    fn capture_light_inputs(&self, c: &mut Capture) {
        let light = self.sketch.as_ref().expect("sketching").light;
        let f = |x: f64| x as f32;
        let leaves: Vec<f32> = (self.leaves.iter())
            .flat_map(|(p, a)| [f(p.x), f(p.y), f(p.z), f(*a)])
            .collect();
        let tips: Vec<f32> = (self.lit_tips().iter())
            .flat_map(|p| [f(p.x), f(p.y), f(p.z)])
            .collect();
        let dirs: Vec<u64> = (light.directions().into_iter())
            .filter(|d| d.1 > 0.0)
            .flat_map(|(t, s)| [t.x / t.z, t.y / t.z, CELL / t.z, s].map(f64::to_bits))
            .collect();
        c.f32s("light_leaves", &leaves);
        c.f32s("light_tips", &tips);
        c.u64s("light_directions", &dirs);
        c.u64s("light_extinction", &[light.extinction.to_bits()]);
    }

    /// The CPU's box (low as i32 bits, then size), each node's quanta,
    /// densities and light, and each tip's read, as `read_light` makes
    /// them.
    fn capture_light_outputs(&self, c: &mut Capture) -> Result<()> {
        let light = self.sketch.as_ref().expect("sketching").light;
        let tips = self.lit_tips();
        let (field, quanta) = Field::quantised_with(&light, &self.leaves, &tips, self.threads)?;
        let (grid, density, lit) = field.parts();
        let reads: Vec<f32> = tips.iter().map(|&t| field.at(t) as f32).collect();
        c.u32s("cpu_grid_low", &grid.low.map(|l| l as u32));
        c.u32s("cpu_grid_size", &grid.size);
        c.u32s("cpu_quanta", &quanta);
        c.f32s("cpu_density", density);
        c.f32s("cpu_lattice", lit);
        c.f32s("cpu_reads", &reads);
        Ok(())
    }
}

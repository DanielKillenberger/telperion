//! The fine roots' rough layout on a second executor (fn-211 task 10;
//! CONTRACT-A.md 2.4): each round of a cycle's layout (the axes that
//! grew, then the new ones a generation at a time) is framed on the CPU
//! and its axes' units laid on the device, every value crossing a Q32 of
//! contract A. A job is what `draw` reads of one axis; what comes back is
//! what it writes.
use super::{Executor, Feedback};
use crate::error::{Error, Result};
use crate::fixed::{q32, ONE32};
use crate::geometry::{cone, Layer};
use crate::grow::draw::{bent, draw_bent, Sketching};
use crate::grow::sketch::Pencil;
use crate::grow::{bud, Grower};
use crate::lineage::Key;
use crate::presence::{Draws, Unit};
use crate::species::Species;
use crate::structure::{Axis, Origin, Phytomer, Vec3};

/// A PA's layout constants, Q32 (`elevation` an amount, Q32 turns), and
/// whether it wanders and keeps a curvature.
#[derive(Debug, Clone, Copy)]
pub struct LayPa {
    pub internode: i64,
    pub tropism: i64,
    pub elevation: i64,
    pub wander: i64,
    pub bend_length: i64,
    pub wanders: bool,
    pub bends: bool,
}

impl LayPa {
    /// Every PA's, and the cone about straight down as a sine, Q32.
    pub fn table(species: &Species) -> (Vec<Self>, i64) {
        let pa = |s: &crate::species::PaState| Self {
            internode: q32(s.internode),
            tropism: q32(s.form.tropism),
            elevation: crate::geometry::amount(s.form.elevation),
            wander: q32(s.form.wander),
            bend_length: q32(s.form.bend_length),
            wanders: s.form.wander > 0.0,
            bends: s.form.bend_length > 0.0,
        };
        (species.states.iter().map(pa).collect(), cone())
    }
}

/// One axis's units to lay: its PA, whether it is the trunk, its pull
/// towards the vertical at its age, its walk (`Layer::words`) and the
/// pulls it carries, its base scale, presence, rank and wood; per unit
/// `[share, survive, persist, size, phytomers]`, per phytomer its key,
/// its node's draw and its radius. Q32 throughout.
#[derive(Debug, Clone, Default)]
pub struct LayJob {
    pub pa: u32,
    pub trunk: bool,
    pub bend: i64,
    pub layer: [i64; 14],
    pub lifts: Vec<[i64; 4]>,
    pub base_scale: i64,
    pub running: i64,
    pub rank: i64,
    pub wood: i64,
    pub units: Vec<[i64; 5]>,
    pub nodes: Vec<(u64, i64, i64)>,
}

/// What laying a job made: its status (0 laid, 1 a point beyond the
/// domain, 2 the trunk below the ground at `z`, 3 a scale outside Q32),
/// its walk, rank, presence and wood, and per phytomer `[scale, rank,
/// tip, heading, side]`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Laid {
    pub status: u32,
    pub z: i64,
    pub layer: [i64; 14],
    pub rank: i64,
    pub running: i64,
    pub wood: i64,
    pub phytomers: Vec<[i64; 11]>,
}

fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

fn q3(v: Vec3) -> [i64; 3] {
    [q32(v.x), q32(v.y), q32(v.z)]
}

/// Axis `axis`'s job at `cycle`.
fn job((axis, pencil, draws): (&Axis, &Pencil, &Draws), species: &Species, cycle: u32) -> LayJob {
    let (layer, lifts) = pencil.layer.words();
    let mut at = pencil.laid;
    let units = (pencil.units..draws.units.len()).map(|k| {
        let u = draws.units[k];
        let unit = |p: &Phytomer| (p.cycle - axis.birth - 1) as usize == k;
        let n = axis.phytomers[at..].iter().take_while(|p| unit(p)).count();
        at += n;
        let size = draws.shade(k)[0];
        [u.share, u.survive, u.persist, size]
            .map(q32)
            .into_iter()
            .chain([n as i64])
    });
    let units: Vec<[i64; 5]> = units
        .map(|u| u.collect::<Vec<_>>().try_into().expect("five"))
        .collect();
    let nodes = (pencil.laid..at)
        .map(|j| {
            let p = &axis.phytomers[j];
            (p.key, q32(draws.nodes[j]), q32(p.radius))
        })
        .collect();
    LayJob {
        pa: axis.pa as u32,
        trunk: pencil.trunk,
        bend: q32(bent(axis, draws, species, cycle)),
        layer,
        lifts,
        base_scale: q32(pencil.base_scale),
        running: q32(pencil.running),
        rank: q32(pencil.rank),
        wood: q32(pencil.wood),
        units,
        nodes,
    }
}

/// A laid job written into its axis and pencil: what `draw` writes.
fn apply(
    (axis, pencil, draws): (&mut Axis, &mut Pencil, &Draws),
    laid: &Laid,
) -> std::result::Result<(), crate::geometry::Stop> {
    match laid.status {
        0 => {}
        1 => return Err(crate::geometry::Stop::Beyond),
        2 => return Err(crate::geometry::Stop::Below(-real(laid.z))),
        _ => panic!("a scale inside Q32"),
    }
    let v3 = |w: &[i64]| Vec3::new(real(w[0]), real(w[1]), real(w[2]));
    let mut j = pencil.laid;
    for k in pencil.units..draws.units.len() {
        let size = draws.shade(k)[0];
        let birth = axis.birth;
        while j < axis.phytomers.len() && (axis.phytomers[j].cycle - birth - 1) as usize == k {
            let (p, o) = (&mut axis.phytomers[j], &laid.phytomers[j - pencil.laid]);
            (p.scale, p.rank, p.size) = (real(o[0]), real(o[1]), size);
            (p.tip, p.heading, p.side) = (v3(&o[2..5]), v3(&o[5..8]), v3(&o[8..11]));
            j += 1;
        }
    }
    let lifts = pencil.layer.words().1;
    pencil.layer = Layer::of_words(&laid.layer, &lifts);
    (pencil.rank, pencil.running) = (real(laid.rank), real(laid.running));
    pencil.wood = real(laid.wood);
    (pencil.units, pencil.laid) = (draws.units.len(), j);
    axis.rank = pencil.rank;
    Ok(())
}

/// The CPU's own laying of jobs: each materialised as an axis and pencil
/// and laid by `draw`, its writes read back. The reference a device's
/// laying is held to.
pub(super) fn lay_cpu(species: &Species, jobs: &[LayJob]) -> Vec<Laid> {
    jobs.iter().map(|j| lay_one(species, j)).collect()
}

fn lay_one(species: &Species, job: &LayJob) -> Laid {
    let mut axis = bud(Key(0), job.pa as usize, 0, Origin::Seed);
    let mut draws = Draws::default();
    let mut nodes = job.nodes.iter();
    for (k, u) in job.units.iter().enumerate() {
        draws.units.push(Unit {
            share: real(u[0]),
            survive: real(u[1]),
            persist: real(u[2]),
        });
        draws.shade.push([real(u[3]), 1.0]);
        for &(key, node, radius) in nodes.by_ref().take(u[4] as usize) {
            draws.nodes.push(real(node));
            axis.phytomers.push(Phytomer {
                cycle: k as u32 + 1,
                tip: Vec3::default(),
                heading: Vec3::default(),
                side: Vec3::default(),
                radius: real(radius),
                scale: 0.0,
                key,
                rank: 0.0,
                size: 1.0,
                light: 1.0,
            });
        }
    }
    let mut pencil = crate::grow::subtree::bare(0.0);
    pencil.layer = Layer::of_words(&job.layer, &job.lifts);
    pencil.trunk = job.trunk;
    (pencil.base_scale, pencil.running) = (real(job.base_scale), real(job.running));
    (pencil.rank, pencil.wood) = (real(job.rank), real(job.wood));
    let drawn = draw_bent(
        (&mut axis, &mut pencil),
        &draws,
        species,
        (0, real(job.bend)),
    );
    let (status, z) = match drawn {
        Ok(()) => (0, 0),
        Err(Error::BelowGround { height, .. }) => (2, -q32(height)),
        Err(_) => (1, 0),
    };
    let row = |p: &Phytomer| {
        let w = [p.scale, p.rank].map(q32);
        let v = [q3(p.tip), q3(p.heading), q3(p.side)].concat();
        [
            w[0], w[1], v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8],
        ]
    };
    Laid {
        status,
        z,
        layer: pencil.layer.words().0,
        rank: q32(pencil.rank),
        running: q32(pencil.running),
        wood: q32(pencil.wood),
        phytomers: axis.phytomers.iter().map(row).collect(),
    }
}

/// An item's layout this cycle: going, with its next round's axes, or
/// done.
enum Round {
    Going(Box<Sketching>, Vec<usize>),
    Done(Result<()>),
}

impl Grower<'_> {
    fn jobs(&self, s: &mut Sketching, todo: &[usize], cycle: u32) -> Vec<LayJob> {
        let pencils = s.pencils(todo);
        let at = todo.iter().zip(pencils);
        at.map(|(&i, p)| job((&self.axes[i], p, &self.draws[i]), self.species, cycle))
            .collect()
    }

    fn laid(&mut self, s: &mut Sketching, todo: &[usize], laid: &[Laid]) -> Vec<(usize, Error)> {
        let pencils = s.pencils(todo);
        let axes = crate::cores::picked(&mut self.axes, todo.iter().copied());
        let all = todo.iter().zip(axes.into_iter().zip(pencils)).zip(laid);
        let mut failed = Vec::new();
        for ((&i, (axis, pencil)), laid) in all {
            if let Err(stop) = apply((axis, pencil, &self.draws[i]), laid) {
                failed.push((i, stop.error(i, axis.pa, axis.birth, axis.base.z)));
            }
        }
        failed
    }
}

impl Executor<'_> {
    /// Every item's rough layout this cycle, each round's axes laid on
    /// `device` together; the first item's error, in their order.
    pub(super) fn sketch_on(
        &mut self,
        cycle: u32,
        device: &mut dyn Feedback,
        advanced: Vec<Vec<usize>>,
    ) -> Result<()> {
        let species = self.coarse.species;
        let mut rounds = self.each(advanced, &|item, advanced| match item
            .g
            .sketch_begin(cycle, &advanced)
        {
            Some(s) => {
                let todo = s.grew();
                Round::Going(Box::new(s), todo)
            }
            None => Round::Done(Ok(())),
        });
        while rounds.iter().any(|r| matches!(r, Round::Going(..))) {
            let built = self.each(rounds, &|item, mut round| {
                let jobs = match &mut round {
                    Round::Going(s, todo) => item.g.jobs(s, todo, cycle),
                    Round::Done(_) => Vec::new(),
                };
                (round, jobs)
            });
            let (going, jobs): (Vec<Round>, Vec<Vec<LayJob>>) = built.into_iter().unzip();
            let counts: Vec<usize> = jobs.iter().map(Vec::len).collect();
            let mut laid = device.lay(species, &jobs.concat()).into_iter();
            let parts: Vec<Vec<Laid>> = (counts.iter())
                .map(|&n| laid.by_ref().take(n).collect())
                .collect();
            rounds = self.each(
                going.into_iter().zip(parts).collect(),
                &|item, (round, laid)| {
                    let Round::Going(mut s, todo) = round else {
                        return round;
                    };
                    let failures = item.g.laid(&mut s, &todo, &laid);
                    if let Err(e) = s.failed(failures) {
                        return Round::Done(Err(e));
                    }
                    match item.g.sketch_round(&mut s) {
                        Some(next) => Round::Going(s, next),
                        None => Round::Done(item.g.sketch_end(*s)),
                    }
                },
            );
        }
        for round in rounds {
            if let Round::Done(Err(e)) = round {
                return Err(e);
            }
        }
        Ok(())
    }
}

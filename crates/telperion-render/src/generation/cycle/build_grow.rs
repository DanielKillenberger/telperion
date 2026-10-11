//! A build grown to its end: begun at the sizes the last named, and
//! begun again from the seed with them raised wherever a count passed a
//! capacity or a bound (DESIGN.md section 5), which leaves the same tree.
use super::build::{Build, Ended, Kit, Tables};
use super::build_light::{refusals, Lit};
use super::build_plan::Bounds;
use super::census::{self as c, Flag};
use super::plan_lay::DEPOSIT;
use super::state::Caps;
use crate::{Gpu, Result};
use telperion_space::Capture;

/// What a build is sized for: the state's tables, the light's points,
/// tips and lattice nodes, and the loops' bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Sizes {
    pub caps: Caps,
    pub points: u32,
    pub tips: u32,
    pub nodes: u32,
    pub bounds: Bounds,
}

impl Sizes {
    /// The smallest a build starts from.
    pub(super) fn least() -> Self {
        let caps = Caps { axes: 4096, units: 4096, phyts: 16384, sleepers: 4096, next: 4096, woken: 1024 };
        Self { caps, points: 16384, tips: 4096, nodes: 4096, bounds: Bounds::default() }
    }
}

fn twice(have: u32, count: u32) -> u32 {
    have.max(count).saturating_mul(2)
}

/// `s` raised for every flag in `flags`, from `census`; refused where a
/// flag names a fault or the budget.
fn raised(mut s: Sizes, flags: &[(Flag, u32)], census: &[u32]) -> Result<Sizes> {
    let refused = |what: String| super::super::io::error(what);
    for &(flag, value) in flags {
        let caps = &mut s.caps;
        let b = &mut s.bounds;
        match flag {
            Flag::Axes => caps.axes = twice(caps.axes, census[c::AXES]).next_multiple_of(64),
            Flag::Units => caps.units = twice(caps.units, census[c::UNITS]),
            Flag::Phyts => caps.phyts = twice(caps.phyts, census[c::PHYTS]),
            Flag::Sleepers => caps.sleepers = twice(caps.sleepers, census[c::SLEEPERS]),
            Flag::Next | Flag::UnitRanges => caps.next = twice(caps.next, census[c::NEXT]).next_multiple_of(64),
            Flag::Woken => caps.woken = twice(caps.woken, census[c::WOKEN]),
            Flag::DrawRanges => caps.axes = twice(caps.axes, census[c::AXES]).next_multiple_of(64),
            Flag::HierGens => b.hier.gens = twice(b.hier.gens, census[c::GENS]),
            Flag::HierPasses => b.hier.passes *= 2,
            Flag::HierLevels => b.hier.levels = (census[c::LEVELS] + 4).min(63),
            Flag::LayGenerations if b.lay.generations < 15 => b.lay.generations = (2 * b.lay.generations).min(15),
            Flag::DepositRanges => b.lay.deposits = (census[c::PHYTS] - census[c::PHYTS0]).div_ceil(DEPOSIT) + 1,
            Flag::Sweeps => b.lay.sweeps *= 2,
            Flag::Points => {
                let tips = census[c::NEXT];
                (s.points, s.tips) = (twice(s.points, census[c::PHYTS] - census[c::PHYTS0] + tips), twice(s.tips, tips));
            }
            Flag::LightErrors if value == 4 => s.nodes = twice(s.nodes, census[c::GRID + 6]),
            Flag::LightErrors => return Err(refused(format!("the light refused: {:?}", refusals(value)))),
            _ => return Err(refused(format!("the build stopped: {flag:?} ({value:#x})"))),
        }
    }
    Ok(s)
}

/// `upto` cycles grown from `sizes`, begun again until no count passes
/// what it was sized for: the build, its sizes and how many times it
/// began again.
pub(super) fn grow(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (mut sizes, upto): (Sizes, u32)) -> Result<(Build, Sizes, u32)> {
    for again in 0..64 {
        let lit = Lit { leaves: sizes.points - sizes.tips.min(sizes.points), tips: sizes.tips };
        let mut b = Build::new(gpu, t, (tables, sizes.caps), (lit, sizes.nodes))?;
        match b.run(gpu, kit, t, (sizes.bounds, upto))? {
            Ended::Grown => return Ok((b, sizes, again)),
            Ended::Passed(flags, census) => sizes = raised(sizes, &flags, &census)?,
        }
    }
    Err(super::super::io::error("a build that never fitted".into()))
}

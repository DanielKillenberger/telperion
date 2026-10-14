//! The fronds a rosette's crown has made by the year it is drawn in, and
//! the share of its whole size each stands at: emerging this year, or made
//! small while the crown was establishing (fn-270 R5b, fn-275 R2).
use super::{
    rosette::{skirt, MAX_FRONDS},
    CanopyParams,
};
use crate::tree::{Tree, Year};
use telperion_space::unfold;

/// How many fronds a crown has made by the year `year` it is drawn in, its
/// living count over a frond's life a year (fn-270 R5b, host): its whole
/// years', and this year's as they emerge,
/// evenly over its stem's elongation from budburst; and the share of its
/// whole size frond `serial` stands at, a frond of this year's expanding
/// from its emergence. A tree no engine grew makes none.
#[derive(Debug, Clone, Copy)]
pub struct Crown {
    year: Option<Year>,
    /// The living crown's fronds and the years one lives: the crown makes
    /// its count over that life (host, fn-270 R5b), none at no life.
    fronds: f64,
    life: f64,
    /// The fronds a crown draws, living and dead, before any is emerging.
    kept: f64,
    /// The years the crown takes to make a mature frond, and the share of
    /// one its first frond is made at (fn-275 R2): none at no years.
    establishment: f64,
    early: f64,
}

impl Crown {
    pub(crate) fn of(tree: &Tree, p: &CanopyParams) -> Self {
        Self::new(tree.year, p)
    }

    /// The crown of a tree drawn in `year` (`executor::year`), for an
    /// executor that holds no tree.
    pub fn new(year: Option<Year>, p: &CanopyParams) -> Self {
        let kept = f64::from(p.rosette_fronds + skirt(p));
        Self {
            year,
            fronds: f64::from(p.rosette_fronds),
            life: p.frond_life,
            kept,
            establishment: p.frond_establishment,
            early: p.frond_early_share,
        }
    }

    /// The rows the device draws the crown by (`stations.wgsl`'s
    /// `rosette`): the fronds made, the years a frond lives, the whole
    /// years before the last (negative where no engine grew the tree) and
    /// the days into it; its stem's budburst, elongation and expansion,
    /// and the fronds it draws (`slots`); the establishment's years and
    /// early share, none where no engine grew the tree.
    pub fn rows(&self) -> [[f64; 4]; 3] {
        let slots = self.slots() as f64;
        match (self.clock(), self.year) {
            (Some((whole, tau)), Some(y)) => [
                [self.made(), self.life, whole, tau],
                [y.budburst, y.elongation, y.expansion, slots],
                [self.establishment, self.early, 0.0, 0.0],
            ],
            _ => [[0.0, self.life, -1.0, 0.0], [0.0, 0.0, 0.0, slots], [0.0; 4]],
        }
    }

    /// The fronds the crown draws, newest first: every one until those
    /// grown above it fill the living crown and the skirt (`fronds`), the
    /// last fading over its last place. A crown its tree grows draws only
    /// the fronds it has made, none before its first.
    pub fn slots(&self) -> usize {
        let newest = self.made().ceil() as i64 - 1;
        let born = match self.year.is_some() && self.life > 0.0 {
            true => (newest + 1).max(0) as usize,
            false => usize::MAX,
        };
        let (mut above, mut n) = (0.0, 0);
        // cap: the kept fronds and every one a crown can make in a year
        // past them; each one drawn adds its share.
        while above < self.kept && n < born && n < self.kept as usize + MAX_FRONDS as usize {
            above += self.unfolded((newest - n as i64) as f64);
            n += 1;
        }
        n
    }

    /// The years grown before the last, and how far into it.
    fn clock(&self) -> Option<(f64, f64)> {
        let y = self.year?;
        let whole = y.age.ceil().max(1.0) - 1.0;
        Some((whole, (y.age - whole) * unfold::YEAR))
    }

    /// Fronds made: whole years' and this year's emerged, the crown's
    /// count a frond's life.
    pub fn made(&self) -> f64 {
        let (Some((whole, tau)), Some(y)) = (self.clock(), self.year) else {
            return 0.0;
        };
        if self.life <= 0.0 {
            return 0.0;
        }
        let emerged = match y.elongation > 0.0 {
            true => ((tau - y.budburst) / y.elongation).clamp(0.0, 1.0),
            false => f64::from(u8::from(tau > y.budburst)),
        };
        self.fronds * (whole + emerged) / self.life
    }

    /// The crown's age in years when it made frond `serial`: a ratio of
    /// whole numbers (its count over the crown's a life), so every
    /// executor places it in the same year.
    fn made_at(&self, serial: f64) -> f64 {
        (serial + 1.0) * self.life / self.fronds
    }

    /// The share of its whole size frond `serial` stands at: whole once
    /// made in a year before this one, and one this year's from its
    /// emergence by the expansion course.
    pub fn unfolded(&self, serial: f64) -> f64 {
        let (Some((whole, tau)), Some(y)) = (self.clock(), self.year) else {
            return 1.0;
        };
        if self.life <= 0.0 || self.fronds <= 0.0 {
            return 1.0;
        }
        let made = self.made_at(serial);
        if made <= whole {
            return 1.0;
        }
        // Still emerging at the year's end: made next year.
        if made > whole + 1.0 {
            return 0.0;
        }
        let emerges = y.budburst + y.elongation * (made - whole).min(1.0);
        unfold::run(tau - emerges, y.expansion)
    }

    /// The share of a mature frond's size frond `serial` was made at
    /// (fn-275 R2): the crown's age when it emerged, along the logistic
    /// course from the early share to 1 over the establishment years, and
    /// kept for the frond's life, so no year turns a frond's size. Whole
    /// with no establishment, and on a crown no engine grew.
    pub fn established(&self, serial: f64) -> f64 {
        let Some(y) = self.year else {
            return 1.0;
        };
        if self.establishment <= 0.0 || self.life <= 0.0 || self.fronds <= 0.0 {
            return 1.0;
        }
        // Its year's fronds emerge evenly over the stem's elongation from
        // budburst (`unfolded`).
        let made = self.made_at(serial);
        let year = made.ceil() - 1.0;
        let born = year + (y.budburst + y.elongation * (made - year)) / unfold::YEAR;
        establishment(self.early, self.establishment, born)
    }
}

/// The share of a mature size an organ made at the crown's age `t` years
/// is made at (fn-275 R2, R2a): the logistic course from `early` to whole
/// over `years`. Whole with no establishment.
pub fn establishment(early: f64, years: f64, t: f64) -> f64 {
    if years <= 0.0 {
        return 1.0;
    }
    1.0 - (1.0 - early) * (1.0 - unfold::course(t / years))
}

/// The years of its rise a stem laid at the crown's age `t` has waited on
/// its crown's establishment over `years` (fn-275 R2a): the integral from
/// 0 to `t` of the course's remainder, `years / 2` once established.
pub fn waited(years: f64, t: f64) -> f64 {
    if years <= 0.0 {
        return 0.0;
    }
    let x = (t / years).clamp(0.0, 1.0);
    years * (x - unfold::integral(x))
}

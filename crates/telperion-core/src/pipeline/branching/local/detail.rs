//! The fine detail a direct build gives up to finish inside its node budget.
//! It is counted in sixteenths of a twig generation. Whole generations are
//! kept as branches from the scaffold outward; inside the next one, a share of
//! its laterals still branch and the rest end as twigs, drawn per bud from the
//! bud's own key. Below one generation, the share is of the first
//! generation's twigs, and the rest are not grown. Every level is a whole tree:
//! each axis it grows reaches its twig ends.
use crate::rng::Rng;

/// Steps of detail inside one twig generation.
pub(in crate::pipeline::branching) const STEPS: u16 = 16;
const SALT: u32 = 0x5f0d_c3a1;

/// Sixteenths of a twig generation kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::pipeline::branching) struct Detail(pub u16);

/// What the detail makes of one lateral bud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Fate {
    /// Grown as the rows grow it.
    Grown,
    /// A twig, whatever the rows would have grown.
    Twig,
    /// Not grown.
    Dropped,
}

impl Detail {
    /// The fate of a lateral bud of `generation` (one or more) keyed `key`.
    pub(super) fn fate(self, generation: usize, key: u32, seed: u32) -> Fate {
        let whole = usize::from(self.0 / STEPS);
        let part = f64::from(self.0 % STEPS) / f64::from(STEPS);
        let kept = || Rng::new(key ^ seed ^ SALT).next_f64() < part;
        match generation {
            g if g < whole => Fate::Grown,
            g if g == whole => {
                if kept() {
                    Fate::Grown
                } else {
                    Fate::Twig
                }
            }
            1 if whole == 0 => {
                if kept() {
                    Fate::Twig
                } else {
                    Fate::Dropped
                }
            }
            _ => Fate::Twig,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_level_is_the_generation_row() {
        // Four whole generations: the fourth generation's laterals are twigs
        // and every shallower one grows, as `generations` four would have it.
        let d = Detail(4 * STEPS);
        for key in 0..64 {
            assert_eq!(d.fate(3, key, 7), Fate::Grown);
            assert_eq!(d.fate(4, key, 7), Fate::Twig);
            assert_eq!(d.fate(5, key, 7), Fate::Twig);
        }
        for key in 0..64 {
            assert_eq!(Detail(0).fate(1, key, 7), Fate::Dropped);
        }
    }

    #[test]
    fn more_detail_never_takes_a_bud_back() {
        // A bud's fate only moves toward growth as the level rises, so a
        // larger budget adds wood to the tree a smaller one drew.
        let rank = |f: Fate| match f {
            Fate::Dropped => 0,
            Fate::Twig => 1,
            Fate::Grown => 2,
        };
        for key in 0..512 {
            for generation in 1..=3 {
                let fates: Vec<_> = (0..=3 * STEPS)
                    .map(|l| rank(Detail(l).fate(generation, key, 11)))
                    .collect();
                assert!(fates.windows(2).all(|w| w[0] <= w[1]), "key {key}");
            }
        }
    }

    #[test]
    fn a_part_keeps_its_share_of_buds() {
        let grown = (0..4096)
            .filter(|&k| Detail(STEPS + 4).fate(1, k, 3) == Fate::Grown)
            .count();
        assert!((900..1150).contains(&grown), "{grown} of 4096 at a quarter");
    }
}

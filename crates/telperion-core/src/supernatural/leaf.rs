//! The leaf the grower bears while the group sets its size (fn-354 R5),
//! and the largest size whose leaf the species' edge admits (fn-351
//! review, host): past it the refusal names the user's setting, never the
//! engine row the multiple lands on.
use crate::{Error, Family, Result};

impl Family {
    /// The square of the group's leaf size over the natural size, which
    /// every age's leaf area is multiplied by; none while the group is off,
    /// sets no size, or the natural size is none.
    pub(super) fn leaf_multiple(&self) -> Option<f64> {
        let (g, natural) = (&self.supernatural, self.canopy.size);
        let size = g.canopy.size.filter(|_| g.enabled && natural > 0.0)?;
        Some((size / natural).powi(2))
    }

    /// Refuses, at `supernatural.canopy.size`, a size whose drawn leaf
    /// passes the species' leaf-area edge (`telperion_space::SETTINGS`),
    /// with the drawn area, the edge and the largest size the family's
    /// own rows admit. A species whose natural leaf the edge already
    /// refuses is left to the engine's own refusal, as written.
    pub(crate) fn drawn_leaf(&self) -> Result<()> {
        let (Some(multiple), Some(species)) = (self.leaf_multiple(), &self.space.species) else {
            return Ok(());
        };
        let edge = telperion_space::setting("leaf_area").expect("the engine's leaf-area row");
        let natural = species.states.iter().map(|s| s.leaf_area);
        if !natural.clone().all(|a| edge.admits(a)) {
            return Ok(());
        }
        let Some((k, drawn)) = (natural.clone().map(|a| a * multiple))
            .enumerate()
            .find(|(_, a)| !edge.admits(*a))
        else {
            return Ok(());
        };
        let most = natural
            .filter(|a| *a > 0.0)
            .map(|a| self.canopy.size * (edge.high / a).sqrt())
            .fold(f64::INFINITY, f64::min);
        let size = self.supernatural.canopy.size.unwrap_or_default();
        Err(Error::InvalidValue {
            field: "supernatural.canopy.size",
            value: format!(
                "{size}: the leaf drawn at it bears {drawn} m² at age {k}, past the species' \
                 edge of {} m²; this family admits a size of at most {most}",
                edge.high
            ),
        })
    }
}

#[cfg(all(test, feature = "geometry"))]
mod tests;

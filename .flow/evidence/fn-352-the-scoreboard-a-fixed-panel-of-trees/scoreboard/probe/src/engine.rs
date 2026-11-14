//! Measures read off the engine's grown structure (z up, engine metres,
//! times the scale the pipeline draws it at): the same species and request
//! the pipeline grows (`executor::growth`), grown a second time because the
//! pipeline keeps its structure to itself (FRICTION.md).
use telperion_core::{pipeline::executor, Family};
use telperion_space::{grow, Origin, Structure, Vec3};

pub struct Engine {
    /// Axes that grew in the last cycle: each one current-year shoot.
    new_shoots: usize,
    /// Phytomers grown in the last cycle.
    new_phytomers: usize,
    /// The lowest junction on the trunk path whose lateral still grows
    /// (anywhere in its system), any size and 5 cm or more at its base.
    lowest_live: Option<f64>,
    lowest_live5: Option<f64>,
    axes: usize,
    phytomers: usize,
    /// fn-354: phytomers whose radius is under the drawing's finest wood
    /// (`pipeline/space.rs` FINEST, 0.2 mm), where it ends an axis.
    fine: usize,
    scale: f64,
}

impl Engine {
    pub fn json(&self) -> String {
        format!(
            "\"new_shoots\":{},\"new_phytomers\":{},\"lowest_live\":{},\"lowest_live5\":{},\
             \"axes\":{},\"phytomers\":{},\"under_finest\":{},\"scale\":{}",
            self.new_shoots,
            self.new_phytomers,
            crate::json(self.lowest_live),
            crate::json(self.lowest_live5),
            self.axes,
            self.phytomers,
            self.fine,
            self.scale
        )
    }
}

pub fn measures(f: &Family) -> Result<Engine, String> {
    let g = executor::growth(f).map_err(|e| e.to_string())?;
    let s = grow(&g.species, g.request).map_err(|e| e.to_string())?;
    // The engine grows at natural size; the conversion draws it at the
    // supernatural scale (`pipeline/space.rs`; `Growth::scale` is gone).
    Ok(read(&s, f.supernatural.scale()))
}

type At = (usize, usize);

/// Each phytomer's child axes, where they join it (fn-341's graph).
fn kids(s: &Structure) -> Vec<Vec<Vec<usize>>> {
    let mut kids: Vec<Vec<Vec<usize>>> = s
        .axes
        .iter()
        .map(|a| vec![Vec::new(); a.phytomers.len()])
        .collect();
    for (i, a) in s.axes.iter().enumerate() {
        let Some(p) = a.origin.parent() else { continue };
        let last = s.axes[p].phytomers.len().saturating_sub(1);
        let node = match a.origin {
            Origin::Lateral { node, .. } => node,
            Origin::Relay { node, .. } => a.joined().map_or(node, |j| j.node),
            _ => last,
        };
        if let Some(k) = kids[p].get_mut(node.min(last)) {
            k.push(i);
        }
    }
    kids
}

fn read(s: &Structure, scale: f64) -> Engine {
    let age = s.age;
    let grew: Vec<bool> = s
        .axes
        .iter()
        .map(|a| a.phytomers.iter().any(|p| p.cycle == age))
        .collect();
    let new_phytomers = s
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .filter(|p| p.cycle == age)
        .count();
    // An axis lives where it or anything it bears grew this cycle.
    let mut live = grew.clone();
    for i in (1..s.axes.len()).rev() {
        if let (true, Some(p)) = (live[i], s.axes[i].origin.parent()) {
            live[p] = true;
        }
    }
    let kids = kids(s);
    let r = |(a, j): At| s.axes[a].phytomers[j].radius;
    let tip = |(a, j): At| s.axes[a].phytomers[j].tip;
    let next = |(a, j): At| -> Vec<At> {
        let mut n = Vec::new();
        if j + 1 < s.axes[a].phytomers.len() {
            n.push((a, j + 1));
        }
        for &c in &kids[a][j] {
            if !s.axes[c].phytomers.is_empty() {
                n.push((c, 0));
            }
        }
        n
    };
    let z0 = s.axes.first().map_or(0.0, |a| a.base.z);
    let z = |v: Vec3| (v.z - z0) * scale;
    let (mut lowest, mut lowest5): (Option<f64>, Option<f64>) = (None, None);
    let low = |slot: &mut Option<f64>, h: f64| *slot = Some(slot.map_or(h, |l: f64| l.min(h)));
    if s.axes.first().is_some_and(|a| !a.phytomers.is_empty()) {
        let mut at = (0usize, 0usize);
        loop {
            let mut n = next(at);
            if n.is_empty() {
                break;
            }
            n.sort_by(|x, y| r(*y).total_cmp(&r(*x)));
            let h = z(tip(at));
            for &(c, _) in &n[1..] {
                if c != at.0 && live[c] {
                    low(&mut lowest, h);
                    if 2.0 * r((c, 0)) * scale >= 0.05 {
                        low(&mut lowest5, h);
                    }
                }
            }
            at = n[0];
        }
    }
    Engine {
        new_shoots: grew.iter().filter(|g| **g).count(),
        new_phytomers,
        lowest_live: lowest,
        lowest_live5: lowest5,
        axes: s.axes.len(),
        phytomers: s.axes.iter().map(|a| a.phytomers.len()).sum(),
        fine: (s.axes.iter().flat_map(|a| &a.phytomers))
            .filter(|p| p.radius * scale < 2e-4)
            .count(),
        scale,
    }
}

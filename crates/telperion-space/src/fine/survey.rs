//! The cut's predictor against a tree grown whole (fn-209 F1): where a
//! species reads light its fine roots cannot be expanded yet (F2b), so
//! its buds are classified by the light-neutral predictor on the tree the
//! engine grows whole, lit, and each fine root's realized kept phytomers
//! are read off that tree.
use crate::bounded::Expected;
use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::grow::{run, Grown, Options, Request};
use crate::species::Species;
use crate::structure::{Origin, Structure};

/// A tree grown whole, surveyed at a cut: each fine root's predicted and
/// realized kept phytomers, the phytomers above the cut, and the fine
/// root each axis stands in (none: above the cut).
#[derive(Debug, Clone, Default)]
pub struct Surveyed {
    pub roots: Vec<(f64, usize)>,
    pub above: usize,
    pub root_of: Vec<Option<usize>>,
}

/// Grows `species` whole and surveys its fine roots at `cut`: every bud
/// made on a kept axis above the cut whose expected kept phytomers are at
/// most `cut`, shed ones included as empty roots.
pub fn survey_lit(
    species: &Species,
    request: Request,
    expected: &Expected,
    cut: f64,
) -> Result<Surveyed> {
    let options = Options {
        settle: true,
        ..Options::default()
    };
    let grown = run(species, request, &mut |_| {}, true, options)?;
    Ok(survey(&grown, &expected.rows(), cut))
}

/// `survey_lit` on the tree laid in full by today's final lay, returned
/// beside its survey (fn-211.4: bud light per fine root).
pub fn survey_laid(
    species: &Species,
    request: Request,
    expected: &Expected,
    cut: f64,
) -> Result<(Structure, Surveyed)> {
    let options = Options {
        records: true,
        ..Options::default()
    };
    let grown = run(species, request, &mut |_| {}, true, options)?;
    let surveyed = survey(&grown, &expected.rows(), cut);
    Ok((grown.structure, surveyed))
}

fn survey(grown: &Grown, rows: &Rows, cut: f64) -> Surveyed {
    let axes = &grown.structure.axes;
    let mut root_of: Vec<Option<usize>> = vec![None; axes.len()];
    let mut by_axis = vec![None; axes.len()];
    for (r, record) in grown.records.iter().enumerate() {
        if let Some(i) = record.axis {
            by_axis[i] = Some(r);
        }
    }
    let mut out = Surveyed::default();
    for (i, axis) in axes.iter().enumerate() {
        let inherited = axis.origin.parent().and_then(|p| root_of[p]);
        root_of[i] = inherited.or_else(|| {
            let r = by_axis[i].filter(|_| matches!(axis.origin, Origin::Lateral { .. }))?;
            let record = &grown.records[r];
            let predicted = rows.lateral(record.pa, record.birth).count;
            (predicted <= cut).then(|| {
                out.roots.push((predicted, 0));
                out.roots.len() - 1
            })
        });
        match root_of[i] {
            Some(k) => out.roots[k].1 += axis.phytomers.len(),
            None => out.above += axis.phytomers.len(),
        }
    }
    // Buds shed whole, on a bearer above the cut: empty roots.
    for record in grown.records.iter().filter(|r| r.axis.is_none()) {
        let Origin::Lateral { parent, .. } = record.origin else {
            continue;
        };
        let predicted = rows.lateral(record.pa, record.birth).count;
        if root_of[parent].is_none() && predicted <= cut {
            out.roots.push((predicted, 0));
        }
    }
    out.root_of = root_of;
    out
}

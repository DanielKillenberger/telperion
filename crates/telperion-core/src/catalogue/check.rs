//! The generated scalar check: each row a site judges, against its bounds, in
//! the rank the site always judged it at.
use super::{Bounds, Check, Checked, Refusal, Site};
use crate::{Error, Result};

/// The most rows one group holds; a test keeps every group under it.
pub(super) const GROUP_LIMIT: usize = 128;

/// Refuses the first row `site` judges in `s` whose value is off its bounds.
/// An unset option is not judged.
pub fn check<S>(rows: &[Checked<S>], s: &S, site: Site) -> Result<()> {
    let mut judged = [None; GROUP_LIMIT];
    for (slot, row) in judged.iter_mut().zip(rows) {
        if let (Some(c), Some(value)) = (row.check, (row.get)(s).number()) {
            *slot = (c.site == site).then_some((c, value, row.bounds));
        }
    }
    first_refusal(&mut judged[..rows.len()])
}

/// The refusal of the lowest-ranked value off its bounds, one copy for every
/// group.
fn first_refusal(judged: &mut [Option<(Check, f64, Bounds)>]) -> Result<()> {
    judged.sort_unstable_by_key(|j| j.map(|(c, ..)| c.rank));
    let Some((check, value, _)) = judged
        .iter()
        .flatten()
        .find(|(_, value, bounds)| !bounds.admits(*value))
    else {
        return Ok(());
    };
    Err(match check.refusal {
        Refusal::Value(field) => Error::InvalidValue {
            field,
            value: value.to_string(),
        },
        Refusal::Input(name) => Error::InvalidInput(name),
    })
}

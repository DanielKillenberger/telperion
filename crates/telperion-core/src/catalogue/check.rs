//! The generated scalar check: each row a site judges, against its bounds, in
//! the rank the site always judged it at.
use super::{Refusal, Row, Site};
use crate::{Error, Result};

/// The most rows one group holds; a test keeps every group under it.
pub(super) const GROUP_LIMIT: usize = 128;

/// Refuses the first row `site` judges in `s` whose value is off its bounds.
/// An unset option is not judged.
pub fn check<S>(rows: &[Row<S>], s: &S, site: Site) -> Result<()> {
    let mut judged = [None::<&Row<S>>; GROUP_LIMIT];
    let mut count = 0;
    for row in rows {
        if row.info.check.is_some_and(|c| c.site == site) {
            judged[count] = Some(row);
            count += 1;
        }
    }
    let judged = &mut judged[..count];
    judged.sort_unstable_by_key(|row| row.and_then(|r| r.info.check).map(|c| c.rank));
    for row in judged.iter().flatten() {
        let Some(value) = (row.get)(s).number() else {
            continue;
        };
        if row.info.bounds.admits(value) {
            continue;
        }
        return Err(match row.info.check.map(|c| c.refusal) {
            Some(Refusal::Value(field)) => Error::InvalidValue {
                field,
                value: value.to_string(),
            },
            Some(Refusal::Input(name)) => Error::InvalidInput(name),
            None => unreachable!("a judged row has a check"),
        });
    }
    Ok(())
}

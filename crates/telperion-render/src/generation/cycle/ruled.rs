//! The ruled standard of agreement with the CPU (DESIGN.md section 9,
//! fn-254 R4 check 4): every axis, unit and phytomer matched by lineage;
//! identity integers exact; tick counts a float decides (a relay's sleep
//! and the like) apart on few rows by few ticks; every float within its
//! field's tolerance of its field's scale.
//!
//! The tolerances come from the plain builds at every preset's default
//! age (EXECUTOR.md, "Plain against the CPU's cycle, by lineage"), with
//! a margin of at least five:
//! - a direction's component, measured at most 8.7e-3 of its unit scale
//!   (Laurelin 80's heading): 5e-2;
//! - every other float, measured at most 1.9e-3 m of tips on a crown
//!   tens of metres across, and 9.5e-5 of a scale near one: 5e-3 of the
//!   field's largest magnitude;
//! - tick counts, measured apart on 249 of the oak at 40's 52,248 axes
//!   (0.48%) by at most 2 ticks: 1% of the rows, 4 ticks.
use super::grown::Kind;
use super::grown_compare::ByLineage;

/// The integer fields counted in ticks of a time a float decides.
const TICKS: [&str; 9] = ["spent", "below", "outlived", "aged", "sleep", "grown_ticks", "unit share", "unit size", "live spent"];
/// The float fields that are a direction's components.
const DIRECTIONS: [&str; 6] = ["heading", "side", "running", "across", "curve", "normal"];

const DIRECTION_TOLERANCE: f64 = 5e-2;
pub(super) const FLOAT_TOLERANCE: f64 = 5e-3;
const TICK_ROWS: f64 = 0.01;
const TICK_APART: f64 = 4.0;

/// Whether a tick count holds: `rows` of `of` apart, by at most `most`.
pub(super) fn ticks_hold(rows: usize, of: usize, most: u32) -> bool {
    rows as f64 <= TICK_ROWS * of as f64 && f64::from(most) <= TICK_APART
}

/// What breaks the standard, one line a finding; none where it holds.
pub(super) fn findings(by: &ByLineage) -> Vec<String> {
    let mut out = Vec::new();
    if by.unmatched != (0, 0) {
        out.push(format!("axes unmatched: device {}, cpu {}", by.unmatched.0, by.unmatched.1));
    }
    for m in by.fields.iter().filter(|m| m.differ > 0) {
        let ticks = TICKS.iter().any(|t| m.name == *t || m.name.ends_with(&format!(" {t}")));
        let ok = match m.kind {
            None => false,
            Some(Kind::U32 | Kind::U64) if ticks => ticks_hold(m.differ, m.rows, m.apart as u32),
            Some(Kind::U32 | Kind::U64) => false,
            Some(Kind::F32 | Kind::F64) => {
                // A direction's component is held against the unit it is
                // a part of, its own scale near zero saying nothing.
                let (tolerance, scale) = match DIRECTIONS.iter().any(|d| m.name.contains(d)) {
                    true => (DIRECTION_TOLERANCE, m.scale.max(1.0)),
                    false => (FLOAT_TOLERANCE, m.scale.max(f64::MIN_POSITIVE)),
                };
                m.apart <= tolerance * scale
            }
        };
        if !ok {
            out.push(format!("{}: {} of {} rows apart, by {:.3e} of scale {:.3e}", m.name, m.differ, m.rows, m.apart, m.scale));
        }
    }
    out
}

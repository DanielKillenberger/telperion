//! Curve composition and Chapman-Richards fit, in code.
//!
//! Every number is computed from the selected points and the manifest's
//! composition choices. Jev never reaches this module: no call, no file, no
//! network. The arithmetic is fn-30's, with one rule changed by fn-58 R5: an
//! age beyond a table's last row is unavailable and never extrapolated.
//!
//! The objective is fn-30's, the summed squared log error of the generator's
//! envelope fraction against the reference height at the manifest's three
//! reference ages, over the same grid, first minimum in grid order.

use serde::{Deserialize, Serialize};

const M_PER_FT: f64 = 0.3048;
const M_PER_IN: f64 = 0.0254;
const CM_PER_IN: f64 = 2.54;
const CM_PER_M: f64 = 100.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "error", rename_all = "kebab-case")]
pub enum CurveError {
    TooFewPoints {
        points: usize,
    },
    BeyondLastRow {
        age_years: f64,
        last_row_age_years: f64,
    },
    BelowFirstRow {
        age_years: f64,
        first_row_age_years: f64,
    },
    Invalid {
        reason: String,
    },
}

impl std::fmt::Display for CurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewPoints { points: n } => {
                write!(f, "fewer than two age-indexed points ({n})")
            }
            Self::BeyondLastRow { age_years: a, .. } => write!(f, "age {a} is beyond the last row"),
            Self::BelowFirstRow { age_years: a, .. } => write!(f, "age {a} is below the first row"),
            Self::Invalid { reason } => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for CurveError {}

fn invalid(reason: impl Into<String>) -> CurveError {
    let reason = reason.into();
    CurveError::Invalid { reason }
}

fn beyond_last_row(age_years: f64, last_row_age_years: f64) -> CurveError {
    CurveError::BeyondLastRow {
        age_years,
        last_row_age_years,
    }
}

/// A finite value above zero, which every reference and asymptote must be.
fn is_positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    M,
    Cm,
    Ft,
    In,
}

/// Conversion to metres, in code.
pub fn to_metres(value: f64, unit: Unit) -> f64 {
    match unit {
        Unit::M => value,
        Unit::Cm => value / CM_PER_M,
        Unit::Ft => value * M_PER_FT,
        Unit::In => value * M_PER_IN,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Row {
    pub age_years: f64,
    pub value: f64,
}

/// An age-indexed table selected from a source, one dimension, one unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub source: String,
    pub unit: Unit,
    pub rows: Vec<Row>,
}

impl Table {
    /// Two or more rows, ascending and distinct in age, every value finite.
    pub fn validate(&self) -> Result<(), CurveError> {
        let points = self.rows.len();
        if points < 2 {
            return Err(CurveError::TooFewPoints { points });
        }
        let mut previous: Option<f64> = None;
        for row in &self.rows {
            let finite = row.age_years.is_finite() && row.value.is_finite();
            if !finite || row.age_years < 0.0 {
                return Err(invalid("a table row is not a finite, non-negative age"));
            }
            if previous.is_some_and(|p| row.age_years <= p) {
                return Err(invalid("table rows are not ascending, distinct ages"));
            }
            previous = Some(row.age_years);
        }
        Ok(())
    }
}

/// What a composition does below its first row or its anchor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BelowFirstRow {
    LinearFromZero,
    Unavailable,
}

/// Below the first anchor: linear from zero up to it, or unavailable.
fn below_anchor_m(
    policy: BelowFirstRow,
    age_years: f64,
    anchor_age_years: f64,
    anchor_value_m: f64,
) -> Result<f64, CurveError> {
    match policy {
        BelowFirstRow::LinearFromZero => Ok(anchor_value_m * age_years / anchor_age_years),
        BelowFirstRow::Unavailable => Err(CurveError::BelowFirstRow {
            age_years,
            first_row_age_years: anchor_age_years,
        }),
    }
}

/// Linear interpolation between table rows, in metres. Beyond the last row is
/// `BeyondLastRow`; a value is never extrapolated.
pub fn interpolate(
    table: &Table,
    age_years: f64,
    below_first_row: BelowFirstRow,
) -> Result<f64, CurveError> {
    table.validate()?;
    let rows = &table.rows;
    let (first, last) = (&rows[0], &rows[rows.len() - 1]);
    if age_years > last.age_years {
        return Err(beyond_last_row(age_years, last.age_years));
    }
    if age_years < first.age_years {
        let anchor = to_metres(first.value, table.unit);
        return below_anchor_m(below_first_row, age_years, first.age_years, anchor);
    }
    let reached = |row: &Row| row.age_years <= age_years;
    let index = rows.iter().rposition(reached).unwrap_or(0);
    let low = &rows[index];
    if low.age_years == age_years || index + 1 == rows.len() {
        return Ok(to_metres(low.value, table.unit));
    }
    let high = &rows[index + 1];
    let span = (age_years - low.age_years) / (high.age_years - low.age_years);
    Ok(to_metres(
        low.value + (high.value - low.value) * span,
        table.unit,
    ))
}

/// Gould, Harrington and Devine 2011 large-tree diameter growth: the 10-year
/// change in squared DBH, in square inches, at a site index in feet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GouldCoefficients {
    pub intercept: f64,
    pub ln_dbh: f64,
    pub dbh2: f64,
    pub bal: f64,
    pub ba: f64,
    pub ln_si: f64,
}

/// The open-grown case sets basal area and basal area in larger trees to zero.
fn gould_dds_in2(dbh_in: f64, c: &GouldCoefficients, site_index_ft: f64) -> f64 {
    (c.intercept + c.ln_dbh * dbh_in.ln() + c.dbh2 * dbh_in * dbh_in + c.ln_si * site_index_ft.ln())
        .exp()
}

/// One decade of open-grown diameter growth at `dbh_cm`, in centimetres.
pub fn gould_decade_growth_cm(dbh_cm: f64, c: &GouldCoefficients, site_index_m: f64) -> f64 {
    let dbh_in = dbh_cm / CM_PER_IN;
    let grown = (dbh_in * dbh_in + gould_dds_in2(dbh_in, c, site_index_m / M_PER_FT)).sqrt();
    (grown - dbh_in) * CM_PER_IN
}

/// Integration from the anchor in whole years, one tenth of the decade change
/// in squared DBH per year, read at the nearest integer year.
fn gould_integrate_cm(
    anchor_age_years: f64,
    anchor_dbh_cm: f64,
    c: &GouldCoefficients,
    site_index_m: f64,
    age_years: f64,
) -> f64 {
    let site_index_ft = site_index_m / M_PER_FT;
    let steps = (age_years.round() - anchor_age_years).round().max(0.0) as u64;
    let mut dbh_in = anchor_dbh_cm / CM_PER_IN;
    for _ in 0..steps {
        dbh_in = (dbh_in * dbh_in + gould_dds_in2(dbh_in, c, site_index_ft) / 10.0).sqrt();
    }
    dbh_in * CM_PER_IN
}

/// How a dimension's reference curve is composed, read from the manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "method", rename_all = "kebab-case")]
pub enum Composition {
    /// Interpolate the table directly.
    Table {
        table: Table,
        below_first_row: BelowFirstRow,
    },
    /// Multiply an interpolated stand-grown table by a factor.
    ScaledTable {
        table: Table,
        factor: f64,
        below_first_row: BelowFirstRow,
    },
    /// Anchor on a table row, then integrate the open-grown diameter growth.
    GouldIntegration {
        anchor_age_years: f64,
        anchor_dbh_cm: f64,
        coefficients: GouldCoefficients,
        site_index_m: f64,
        max_age_years: f64,
        below_anchor: BelowFirstRow,
    },
}

/// Value of the composed reference at an age, in metres.
pub fn reference_at(c: &Composition, age_years: f64) -> Result<f64, CurveError> {
    if !age_years.is_finite() || age_years < 0.0 {
        return Err(invalid(format!("age {age_years} is not a finite year")));
    }
    match c {
        Composition::Table {
            table,
            below_first_row,
        } => interpolate(table, age_years, *below_first_row),
        Composition::ScaledTable {
            table,
            factor,
            below_first_row,
        } => {
            if !factor.is_finite() {
                return Err(invalid("the scale factor is not finite"));
            }
            Ok(factor * interpolate(table, age_years, *below_first_row)?)
        }
        Composition::GouldIntegration {
            anchor_age_years: anchor_age,
            anchor_dbh_cm: anchor_cm,
            coefficients,
            site_index_m: site_index,
            max_age_years: max_age,
            below_anchor,
        } => {
            let anchor = [*anchor_age, *anchor_cm, *site_index];
            if anchor.iter().any(|value| !is_positive(*value)) {
                return Err(invalid("the Gould anchor and site index must be positive"));
            }
            if max_age <= anchor_age {
                return Err(CurveError::TooFewPoints { points: 1 });
            }
            if age_years > *max_age {
                return Err(beyond_last_row(age_years, *max_age));
            }
            if age_years < *anchor_age {
                let metres = anchor_cm / CM_PER_M;
                return below_anchor_m(*below_anchor, age_years, *anchor_age, metres);
            }
            let cm = gould_integrate_cm(
                *anchor_age,
                *anchor_cm,
                coefficients,
                *site_index,
                age_years,
            );
            Ok(cm / CM_PER_M)
        }
    }
}

/// The generator's Chapman-Richards fraction at the default work budget.
/// Curve fitting does not currently author a family's workBudget.
pub fn fraction(rate: f64, shape: f64, year: f64) -> f64 {
    let f = (1.0 - (-rate * year).exp()).powf(shape);
    if f >= 1.0 - 0.5 / telperion_core::ranges::DEFAULT_WORK_BUDGET as f64 {
        1.0
    } else {
        f
    }
}

/// First integer year whose fraction rounds to the whole lifetime work.
pub fn mature_age(rate: f64, shape: f64) -> u64 {
    let (mut low, mut high) = (0u64, 1_000_000u64);
    while low < high {
        let mid = low + (high - low) / 2;
        if fraction(rate, shape, mid as f64) == 1.0 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

/// The preset's authored envelope height and the generator's measured mature
/// trunk DBH are the two asymptotes; the tolerance comes from the manifest.
/// Fits the default work budget only: the manifest's curve inputs carry no
/// preset family or workBudget, and the result authors only rate and shape.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FitInput {
    pub envelope_height_m: f64,
    pub mature_dbh_m: f64,
    pub reference_ages_years: [f64; 3],
    pub height: Composition,
    pub dbh: Composition,
    pub tolerance_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgeRow {
    pub age_years: f64,
    pub reference_height_m: Option<f64>,
    pub reference_dbh_m: Option<f64>,
    pub model_height_m: f64,
    pub model_dbh_m: f64,
    pub height_error_percent: Option<f64>,
    pub dbh_error_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToleranceMiss {
    pub field: String,
    pub age_years: f64,
    pub measured: f64,
    pub reference: f64,
    pub error_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unavailable {
    pub field: String,
    pub age_years: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FitReport {
    pub rate: f64,
    pub shape: f64,
    pub derived_mature_age_years: u64,
    pub objective: f64,
    pub rows: Vec<AgeRow>,
    pub misses: Vec<ToleranceMiss>,
    pub unavailable: Vec<Unavailable>,
}

/// The reference value at each of the three ages, and every age without one.
type References = ([Option<f64>; 3], Vec<Unavailable>);

/// Reference values at the three ages. An age the composition cannot answer
/// carries no value and is recorded as unavailable.
fn references(c: &Composition, field: &str, ages: &[f64; 3]) -> References {
    let mut values = [None; 3];
    let mut unavailable = Vec::new();
    for (slot, &age_years) in values.iter_mut().zip(ages) {
        let reason = match reference_at(c, age_years) {
            Ok(value) if is_positive(value) => {
                *slot = Some(value);
                continue;
            }
            Ok(_) => "the reference value is not positive".to_string(),
            Err(error) => error.to_string(),
        };
        let field = field.to_string();
        unavailable.push(Unavailable {
            field,
            age_years,
            reason,
        });
    }
    (values, unavailable)
}

/// Grid search over the validated trait ranges, keeping the first minimum in
/// grid order: shape outer in steps of 0.1, rate inner in steps of 0.001.
fn search(envelope_height_m: f64, ages: &[f64; 3], heights: &[Option<f64>; 3]) -> (f64, f64, f64) {
    let mut best: Option<(f64, f64, f64)> = None;
    for shape_index in 10..=80u32 {
        let shape = f64::from(shape_index) / 10.0;
        for rate_index in 1..=399u32 {
            let rate = f64::from(rate_index) / 1000.0;
            let mut objective = 0.0;
            for (&age_years, reference) in ages.iter().zip(heights) {
                let Some(reference) = reference else { continue };
                let model = (envelope_height_m * fraction(rate, shape, age_years)).max(1e-3);
                objective += (model / reference).ln().powi(2);
            }
            if best.is_none_or(|(lowest, _, _)| objective < lowest) {
                best = Some((objective, rate, shape));
            }
        }
    }
    best.expect("the grid holds at least one point")
}

/// Fit the growth traits to the composed height reference, then compare both
/// dimensions at the three reference ages. Height and diameter are compared
/// independently; a miss beyond tolerance is reported, never accepted. A height
/// reference with fewer than two age-indexed points is `TooFewPoints`, which
/// the caller files as missing-curve; a diameter reference with fewer than two
/// is skipped and recorded as unavailable.
pub fn fit(input: &FitInput) -> Result<FitReport, CurveError> {
    if !is_positive(input.envelope_height_m) || !is_positive(input.mature_dbh_m) {
        return Err(invalid("the envelope and mature DBH must be positive"));
    }
    if !input.tolerance_percent.is_finite() || input.tolerance_percent < 0.0 {
        return Err(invalid("the tolerance must be a finite percentage"));
    }
    let ages = &input.reference_ages_years;
    let (heights, mut unavailable) = references(&input.height, "height_m", ages);
    let available = heights.iter().flatten().count();
    if available < 2 {
        return Err(CurveError::TooFewPoints { points: available });
    }
    let (dbhs, dbh_unavailable) = references(&input.dbh, "dbh_m", ages);
    unavailable.extend(dbh_unavailable);

    let (objective, rate, shape) = search(input.envelope_height_m, ages, &heights);
    let (mut rows, mut misses) = (Vec::with_capacity(ages.len()), Vec::new());
    for (index, &age_years) in ages.iter().enumerate() {
        let f = fraction(rate, shape, age_years);
        let model_height_m = input.envelope_height_m * f;
        let model_dbh_m = input.mature_dbh_m * f;
        let dimensions = [
            ("height_m", model_height_m, heights[index]),
            ("dbh_m", model_dbh_m, dbhs[index]),
        ];
        let mut errors = [None, None];
        for (slot, (field, measured, reference)) in errors.iter_mut().zip(dimensions) {
            let Some(reference) = reference else { continue };
            let error_percent = 100.0 * (measured / reference - 1.0);
            *slot = Some(error_percent);
            if error_percent.abs() > input.tolerance_percent {
                let field = field.to_string();
                misses.push(ToleranceMiss {
                    field,
                    age_years,
                    measured,
                    reference,
                    error_percent,
                });
            }
        }
        rows.push(AgeRow {
            age_years,
            reference_height_m: heights[index],
            reference_dbh_m: dbhs[index],
            model_height_m,
            model_dbh_m,
            height_error_percent: errors[0],
            dbh_error_percent: errors[1],
        });
    }
    Ok(FitReport {
        rate,
        shape,
        derived_mature_age_years: mature_age(rate, shape),
        objective,
        rows,
        misses,
        unavailable,
    })
}

use super::*;
const DENOMINATOR: u64 = 1_000_000_000;
const TICKS_PER_YEAR: f64 = 12.0 * DENOMINATOR as f64;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Age {
    pub month: u64,
    pub remainder: u64,
}
impl Age {
    pub fn from_years(years: f64) -> Result<Self> {
        let ticks = checked_ticks("age", years)?;
        Ok(Self::from_ticks(ticks))
    }
    pub fn advanced(self, years: f64) -> Result<Self> {
        let ticks = checked_ticks("advance", years)?;
        let total = self.month * DENOMINATOR + self.remainder + ticks;
        if total > (MAX_AGE * TICKS_PER_YEAR) as u64 {
            return Err(Error::InvalidValue {
                field: "age",
                value: (total as f64 / TICKS_PER_YEAR).to_string(),
            });
        }
        Ok(Self::from_ticks(total))
    }
    fn from_ticks(ticks: u64) -> Self {
        Self {
            month: ticks / DENOMINATOR,
            remainder: ticks % DENOMINATOR,
        }
    }
    pub fn years(self) -> f64 {
        self.month as f64 / 12.0 + self.remainder as f64 / TICKS_PER_YEAR
    }
}
fn checked_ticks(field: &'static str, years: f64) -> Result<u64> {
    if !years.is_finite() || !(0.0..=MAX_AGE).contains(&years) {
        return Err(Error::InvalidValue {
            field,
            value: years.to_string(),
        });
    }
    Ok((years * TICKS_PER_YEAR).round() as u64)
}

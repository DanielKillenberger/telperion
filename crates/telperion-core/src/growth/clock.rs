use super::*;
// Retain the original billionth-of-a-month API resolution.
const DENOMINATOR: u64 = 12_000_000_000;
const TICKS_PER_YEAR: f64 = DENOMINATOR as f64;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Age {
    pub slice: u64,
    pub remainder: u64,
}
impl Age {
    pub fn from_years(years: f64) -> Result<Self> {
        let ticks = checked_ticks("age", years)?;
        Ok(Self::from_ticks(ticks))
    }
    pub fn advanced(self, years: f64) -> Result<Self> {
        let ticks = checked_ticks("advance", years)?;
        let total = self.slice * DENOMINATOR + self.remainder + ticks;
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
            slice: ticks / DENOMINATOR,
            remainder: ticks % DENOMINATOR,
        }
    }
    pub fn ticks(self) -> u64 {
        self.slice * DENOMINATOR + self.remainder
    }
    pub fn years(self) -> f64 {
        self.slice as f64 + self.remainder as f64 / TICKS_PER_YEAR
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_boundary_keeps_the_original_submonth_resolution() {
        let almost = Age::from_years(1.0 - 1.0 / 12_000_000_000.0).unwrap();
        assert_eq!(almost.slice, 0, "a sub-year advance ran a growth slice");
        assert_eq!(almost.remainder, 11_999_999_999);
        let whole = almost.advanced(1.0 / 12_000_000_000.0).unwrap();
        assert_eq!(whole.slice, 1);
        assert_eq!(whole.remainder, 0);
        let mut split = Age::default();
        for part in [0.01, 0.02, 0.07, 0.1, 0.3, 0.5] {
            split = split.advanced(part).unwrap();
        }
        assert_eq!(split, whole);
    }
}

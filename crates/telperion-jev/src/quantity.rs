//! One number-and-unit grammar (fn-131): the extractor finds its candidate
//! sentences and spans with it, select parses a chosen span with it, and
//! quality reads a stated age off a sentence with it. Code owns every number.

use std::sync::OnceLock;

use regex::Regex;

const NUMBER: &str = r"\d[\d,]*(?:\.\d+)?";
const DASH: &str = r"\s*(?:to|-|–|—)\s*";

/// A number or a range of two, then a length, age or rate unit. A unit may
/// sit glued to its number ("6–10m"); "in" is an inch only as "in." or
/// spelled out, never the preposition.
pub fn unit_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(&format!(
            r"(?i)({NUMBER})(?:{DASH}({NUMBER}))?\s*(ft|feet|foot|in\.|inches|inch|m\b|metres?|meters?|centimetres?|centimeters?|cm|millimetres?|millimeters?|mm|years?|yr|rings/in)"
        ))
        .expect("unit regex")
    })
}

/// A stated age or age span: "in 15 to 20 years", "at 20 years",
/// "10-year-old", "20 years old".
fn age_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(&format!(
            r"(?i)(?:\b(?:in|at|after|by|within|over)\s+(?:about\s+|around\s+|some\s+)?({NUMBER})(?:{DASH}({NUMBER}))?\s*(?:years?|yrs?)\b|\b({NUMBER})(?:{DASH}({NUMBER}))?[\s-]*years?[\s-]*old\b)"
        ))
        .expect("age regex")
    })
}

/// A number as a source writes it: "1,200" is a thousand two hundred, "7,2"
/// a decimal comma.
pub fn parse_number(text: &str) -> Option<f64> {
    let mut groups = text.split(',');
    let head = groups.next()?;
    let rest: Vec<&str> = groups.collect();
    let thousands = !rest.is_empty()
        && head.len() <= 3
        && rest
            .iter()
            .all(|g| g.split('.').next().is_some_and(|d| d.len() == 3));
    let plain = if thousands {
        text.replace(',', "")
    } else {
        text.replace(',', ".")
    };
    plain.parse().ok()
}

/// Metres per unit of a length unit; None for an age or a rate unit.
fn metres_per(unit: &str) -> Option<f64> {
    let unit = unit.trim_end_matches('.').to_ascii_lowercase();
    Some(match unit.as_str() {
        "ft" | "feet" | "foot" => 0.3048,
        "in" | "inch" | "inches" => 0.0254,
        "cm" => 0.01,
        "mm" => 0.001,
        u if u.starts_with("centimet") => 0.01,
        u if u.starts_with("millimet") => 0.001,
        u if u == "m" || u.starts_with("metre") || u.starts_with("meter") => 1.0,
        _ => return None,
    })
}

/// The first length in `text` as `[low, high]` metres, with its unit as
/// written ("in." as "in"): `50 to 90 ft` is [15.24, 27.432]; one number is
/// both ends.
pub fn first_length(text: &str) -> Option<([f64; 2], String)> {
    unit_re().captures_iter(text).find_map(|c| {
        let unit = c.get(3)?.as_str();
        let factor = metres_per(unit)?;
        let low = parse_number(c.get(1)?.as_str())?;
        let high = c
            .get(2)
            .and_then(|m| parse_number(m.as_str()))
            .unwrap_or(low);
        let unit = unit.trim_end_matches('.').to_ascii_lowercase();
        Some(([low * factor, high * factor], unit))
    })
}

/// The ages `text` states a size at, in years: "reaching 5 m in 15 to 20
/// years" is [15, 20]. Empty when it states none.
pub fn stated_ages(text: &str) -> Vec<f64> {
    let Some(c) = age_re().captures(text) else {
        return Vec::new();
    };
    let (low, high) = match c.get(1) {
        Some(low) => (low, c.get(2)),
        None => match c.get(3) {
            Some(low) => (low, c.get(4)),
            None => return Vec::new(),
        },
    };
    let mut ages: Vec<f64> = [Some(low), high]
        .into_iter()
        .flatten()
        .filter_map(|m| parse_number(m.as_str()))
        .collect();
    ages.dedup();
    ages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_length_parses_glued_millimetres_grouped_and_dashed() {
        let cases = [
            ("6\u{2013}10m (20\u{2013}33ft)", [6.0, 10.0], "m"),
            ("12 mm long", [0.012, 0.012], "mm"),
            ("1,200 feet", [365.76, 365.76], "feet"),
            ("DG 7,2 cm", [0.072, 0.072], "cm"),
            ("6\u{2014}10 m", [6.0, 10.0], "m"),
            ("30 centimetres (12 inches)", [0.3, 0.3], "centimetres"),
            ("24 to 40 in. in DBH", [0.6096, 1.016], "in"),
        ];
        for (text, range, unit) in cases {
            let (got, u) = first_length(text).unwrap_or_else(|| panic!("{text}"));
            assert!(
                (got[0] - range[0]).abs() < 1e-9 && (got[1] - range[1]).abs() < 1e-9,
                "{text}: {got:?}"
            );
            assert_eq!(u, unit, "{text}");
        }
        // "in" the preposition is no inch: the age is no length either.
        assert!(first_length("grows in 20 years").is_none());
        assert!(first_length("about 500 years").is_none());
    }

    #[test]
    fn an_age_is_read_only_where_the_sentence_states_one() {
        let rate = "Date palms have a moderate growth rate of 30-45 cm (1 to 1.5 feet) a year, reaching 5 m (20 feet) in 15 to 20 years depending on the cultivar.";
        assert_eq!(stated_ages(rate), vec![15.0, 20.0]);
        assert_eq!(stated_ages("reaches 6 m at 20 years"), vec![20.0]);
        assert_eq!(stated_ages("a 10-year-old tree 4 m tall"), vec![10.0]);
        assert!(stated_ages("may live 500 years").is_empty());
    }
}

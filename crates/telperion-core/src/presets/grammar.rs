//! The value file's grammar, shared by the build script that compiles the
//! shipped presets and by the reader and writer the tools use. A line is a
//! blank, a comment whose first mark is `#`, or one row: its wire path, `=`,
//! and a value, `true`, `false` or a decimal number. A path appears once.
//! Only the grammar is judged here; the catalogue judges the rows.

/// How a value is written, which decides the rows it may set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// A number with no point and no exponent.
    Whole,
    /// A number with a point or an exponent.
    Real,
    /// `true` or `false`.
    Switch,
}

/// One row of a value file, as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line<'a> {
    /// Its line number, from one.
    pub at: usize,
    pub path: &'a str,
    pub form: Form,
    pub text: &'a str,
}
impl Line<'_> {
    /// The value as the row stores it: a switch is one or zero.
    pub fn number(&self) -> f64 {
        match self.text {
            "true" => 1.0,
            "false" => 0.0,
            text => text.parse().expect("the grammar admitted a number"),
        }
    }
}

/// The rows of `text`, or the first line the grammar refuses, as
/// `<line>: <why>`.
pub fn parse(text: &str) -> Result<Vec<Line<'_>>, String> {
    let mut rows: Vec<Line> = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let at = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (path, value) = line
            .split_once('=')
            .ok_or(format!("{at}: `{line}` is not `<path> = <value>`"))?;
        let (path, text) = (path.trim(), value.trim());
        if !is_path(path) {
            return Err(format!("{at}: `{path}` is not a wire path"));
        }
        let form = form(text).ok_or(format!("{at}: `{text}` is not a value"))?;
        if let Some(first) = rows.iter().find(|r| r.path == path) {
            return Err(format!("{at}: {path} is already set on line {}", first.at));
        }
        rows.push(Line {
            at,
            path,
            form,
            text,
        });
    }
    Ok(rows)
}

/// `/` and then words of letters and digits, separated by `/`.
fn is_path(path: &str) -> bool {
    path.strip_prefix('/').is_some_and(|rest| {
        rest.split('/')
            .all(|w| !w.is_empty() && w.bytes().all(|b| b.is_ascii_alphanumeric()))
    })
}

fn form(text: &str) -> Option<Form> {
    if text == "true" || text == "false" {
        return Some(Form::Switch);
    }
    let (mantissa, exponent) = match text.split_once(['e', 'E']) {
        Some((m, e)) => (m, Some(e.strip_prefix(['-', '+']).unwrap_or(e))),
        None => (text, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let unsigned = mantissa.strip_prefix('-').unwrap_or(mantissa);
    let (whole, fraction) = match unsigned.split_once('.') {
        Some((w, f)) => (w, Some(f)),
        None => (unsigned, None),
    };
    let number = digits(whole) && fraction.is_none_or(digits) && exponent.is_none_or(digits);
    match (number, fraction.is_some() || exponent.is_some()) {
        (false, _) => None,
        (true, true) => Some(Form::Real),
        (true, false) => Some(Form::Whole),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_file_is_read_as_its_rows() {
        let text = "# a comment\n\n/age = 120\n  /skeleton/bias/supernatural/enabled = true\n\
                    /skeleton/envelope/height = -2.5e-3\n";
        let rows = parse(text).unwrap();
        let seen: Vec<_> = rows.iter().map(|r| (r.at, r.path, r.form)).collect();
        assert_eq!(
            seen,
            [
                (3, "/age", Form::Whole),
                (4, "/skeleton/bias/supernatural/enabled", Form::Switch),
                (5, "/skeleton/envelope/height", Form::Real),
            ]
        );
        assert_eq!(rows[2].number(), -2.5e-3);
    }

    #[test]
    fn a_line_off_the_grammar_is_refused_with_its_line() {
        for (text, why) in [
            ("/age 120", "1: `/age 120` is not `<path> = <value>`"),
            ("age = 1", "1: `age` is not a wire path"),
            ("/a//b = 1", "1: `/a//b` is not a wire path"),
            ("/age = 1.", "1: `1.` is not a value"),
            ("/age = 0x10", "1: `0x10` is not a value"),
            ("/age = none", "1: `none` is not a value"),
            ("/age = 1\n\n/age = 2", "3: /age is already set on line 1"),
        ] {
            assert_eq!(parse(text).unwrap_err(), why, "{text}");
        }
    }
}

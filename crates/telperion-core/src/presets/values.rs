//! A family as a preset value file and back: the writer the species runner's
//! Accept stage calls, and the reader that checks what it wrote. A value file
//! holds only the rows that differ from the default family, one `<path> =
//! <value>` line each (`grammar.rs`).
use super::grammar;
use super::table::{Refused, Value};
use crate::catalogue::{self, Entry, Kind};
use crate::Family;

/// The family `text` describes: the default family with each row set, every
/// row judged as the build judges a shipped preset's. A refusal names its
/// line.
pub fn read(text: &str) -> Result<Family, String> {
    let mut f = Family::default();
    for line in grammar::parse(text)? {
        let at = line.at;
        let path = line.path;
        let why = match Value::row(path, line.form, line.number()) {
            Ok(value) => {
                value.apply(&mut f);
                continue;
            }
            Err(Refused::Unknown) => "is not a catalogue row",
            Err(Refused::Kind) => "holds another kind of value",
            Err(Refused::Bounds) => "is off its bounds",
        };
        return Err(format!("{at}: {path} {why}"));
    }
    Ok(f)
}

/// `family` as a value file, written over `source`, the file as it stands
/// (empty for a new preset). Each row `source` sets keeps its line, its
/// comments and its text where the value is unchanged, and takes the new
/// value where it moved; a row back at its default loses its line; a row
/// newly off the default is added at the end under `note`. An option the
/// default family sets and `family` leaves unset has no line to say so and
/// is refused.
pub fn write(family: &Family, source: &str, note: &str) -> Result<String, String> {
    let defaults = Family::default();
    let set: Vec<Entry> = catalogue::entries()
        .filter(|e| e.get(family).number() != e.get(&defaults).number())
        .collect();
    if let Some(e) = set.iter().find(|e| e.get(family).number().is_none()) {
        return Err(format!("{} is unset where the default sets it", e.path()));
    }
    let rows = grammar::parse(source)?;
    let mut out = String::new();
    for (i, line) in source.lines().enumerate() {
        let Some(row) = rows.iter().find(|r| r.at == i + 1) else {
            out += line;
            out += "\n";
            continue;
        };
        let Some(e) = set.iter().find(|e| e.path() == row.path) else {
            continue;
        };
        let now = e.get(family).number();
        let text = match now == Some(row.number()) {
            true => row.text.to_string(),
            false => value(*e, family),
        };
        let (head, tail) = line.split_once('=').expect("a row line holds `=`");
        out += &format!("{head}={}\n", tail.replacen(row.text, &text, 1));
    }
    let added: Vec<String> = set
        .iter()
        .filter(|e| !rows.iter().any(|r| r.path == e.path()))
        .map(|e| format!("{} = {}\n", e.path(), value(*e, family)))
        .collect();
    if !added.is_empty() {
        if !out.is_empty() && !out.ends_with("\n\n") {
            out += "\n";
        }
        if !note.is_empty() {
            out += &format!("# {note}\n");
        }
        out += &added.concat();
    }
    Ok(out)
}

/// The row's value in `family` as the grammar writes it: a switch as a word,
/// a count as a whole number, a real as the shortest decimal that reads back
/// as the same double.
fn value(e: Entry, family: &Family) -> String {
    let scalar = e.get(family);
    let n = scalar.number().expect("a set row");
    match scalar.kind() {
        Kind::Switch => (n != 0.0).to_string(),
        Kind::Real | Kind::OptionalReal => format!("{n:?}"),
        Kind::Count | Kind::Size | Kind::OptionalSize => format!("{n}"),
    }
}

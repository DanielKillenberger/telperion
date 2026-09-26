//! The browser's parameter metadata, rendered from the catalogue: the
//! `Family` type the package declares, each row under its meaning, and the
//! table the harness draws its controls from.
use super::{entries, reference::meaning, Dial, Entry, Growth, Kind};
use crate::Family;
use std::fmt::Write;

/// `src/browser/parameters.generated.ts`, as the catalogue states it today.
pub fn browser() -> String {
    let defaults = Family::default();
    let rows: Vec<Entry> = entries().collect();
    let mut out = String::from(HEADER);
    out.push_str("\n/** A family: every wire row, under the meaning the catalogue gives it. */\n");
    out.push_str("export interface Family ");
    group(&mut out, &rows, &defaults, "", 0);
    out.push_str("\n\n");
    out.push_str(PARAMETER);
    out.push_str("export const PARAMETERS: readonly Parameter[] = [\n");
    for entry in rows {
        parameter(&mut out, entry, &defaults);
    }
    out.push_str("];\n");
    out
}

const HEADER: &str =
    "// Generated from the parameter catalogue (crates/telperion-core/src/catalogue.rs)
// by `the_browser_reads_the_catalogue` in crates/telperion-core/tests/parameter_reference.rs;
// run it with TELPERION_WRITE_REFERENCE=1 to rewrite this file. Do not edit.
";

const PARAMETER: &str = "/** One catalogue row, as a control reads it. */
export interface Parameter {
  /** The row's JSON pointer on the wire. */
  path: string;
  kind: \"real\" | \"count\" | \"switch\";
  /** An unset value is admitted and means \"none stated\". */
  optional: boolean;
  meaning: string;
  unit: string;
  /** The admitted values: `lowOpen` refuses `low` itself, `zero` admits zero besides. */
  low: number;
  high: number;
  lowOpen: boolean;
  zero: boolean;
  /** Where the row lies dormant; empty where it always acts. */
  applies: string;
  /** Who reads it: the direct build, only the growth path, or no production stage. */
  reach: \"mature\" | \"growth\" | \"deprecated\";
  /** The tuning dial's window and small step, where the row offers one. */
  dial?: { window: [number, number]; step: number };
}

";

/// The object at `prefix`: its own rows, then each nested group in the order
/// its first row is declared.
fn group(out: &mut String, rows: &[Entry], defaults: &Family, prefix: &str, depth: usize) {
    let pad = "  ".repeat(depth + 1);
    out.push_str("{\n");
    let mut nested: Vec<&str> = Vec::new();
    for entry in rows {
        let Some(rest) = entry
            .path()
            .strip_prefix(prefix)
            .and_then(|r| r.strip_prefix('/'))
        else {
            continue;
        };
        match rest.split_once('/') {
            None => field(out, entry, defaults, rest, &pad),
            Some((name, _)) if !nested.contains(&name) => nested.push(name),
            Some(_) => {}
        }
    }
    for name in nested {
        let _ = write!(out, "{pad}{name}: ");
        group(out, rows, defaults, &format!("{prefix}/{name}"), depth + 1);
        out.push_str(";\n");
    }
    let _ = write!(out, "{}}}", "  ".repeat(depth));
}

fn field(out: &mut String, entry: &Entry, defaults: &Family, name: &str, pad: &str) {
    let info = entry.info();
    let deprecated = if info.deprecated { " @deprecated" } else { "" };
    let doc = meaning(info.meaning).replace("*/", "*\\/");
    let kind = entry.get(defaults).kind();
    let (optional, ty) = match kind {
        Kind::Switch => ("", "boolean"),
        Kind::OptionalReal | Kind::OptionalSize => ("?", "number"),
        Kind::Real | Kind::Count | Kind::Size => ("", "number"),
    };
    let _ = writeln!(
        out,
        "{pad}/** {doc}{deprecated} */\n{pad}{name}{optional}: {ty};"
    );
}

fn parameter(out: &mut String, entry: Entry, defaults: &Family) {
    let info = entry.info();
    let value = entry.get(defaults).kind();
    let kind = match value {
        Kind::Switch => "switch",
        k if k.whole() => "count",
        _ => "real",
    };
    let optional = matches!(value, Kind::OptionalReal | Kind::OptionalSize);
    let reach = if info.deprecated {
        "deprecated"
    } else if info.growth == Growth::Only {
        "growth"
    } else {
        "mature"
    };
    let b = info.bounds;
    let _ = write!(
        out,
        "  {{ path: {}, kind: \"{kind}\", optional: {optional}, meaning: {}, unit: {}, \
         low: {}, high: {}, lowOpen: {}, zero: {}, applies: {}, reach: \"{reach}\"",
        text(entry.path()),
        text(&meaning(info.meaning)),
        text(info.unit),
        number(b.low),
        number(b.high),
        b.low_open,
        b.zero,
        text(info.applies),
    );
    if let Dial::Tuned(t) = info.dial {
        let [low, high] = t.window.unwrap_or([b.low, b.high]);
        let _ = write!(
            out,
            ", dial: {{ window: [{}, {}], step: {} }}",
            number(low),
            number(high),
            number(t.small)
        );
    }
    out.push_str(" },\n");
}

/// A TypeScript string literal.
fn text(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn number(v: f64) -> String {
    match v {
        f64::INFINITY => "Infinity".into(),
        f64::NEG_INFINITY => "-Infinity".into(),
        v => format!("{v}"),
    }
}

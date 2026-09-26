//! The parameter reference, rendered from the catalogue, and the catalogue's
//! revision: a digest of that reference, so anything the catalogue states
//! moves it.
use super::{entries, Blend, Bounds, Check, Dial, Entry, Growth, Refusal, Stage};
use crate::Family;
use std::fmt::Write;

/// `docs/parameters.md`, as the catalogue states it today.
pub fn reference() -> String {
    let defaults = Family::default();
    let mut out = String::from(HEADER);
    let mut group = None;
    for entry in entries() {
        let at = entry.path().rsplit_once('/').map_or("", |(g, _)| g);
        if group != Some(at) {
            group = Some(at);
            let title = if at.is_empty() { "/ (the family)" } else { at };
            let _ = write!(out, "\n## `{title}`\n");
        }
        row(&mut out, entry, &defaults);
    }
    out
}

/// A digest of the reference, as sixteen hex digits.
pub fn revision() -> String {
    let digest = reference().bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{digest:016x}")
}

const HEADER: &str = "# Parameter reference

Generated from the parameter catalogue (`crates/telperion-core/src/catalogue.rs`)
by `every_row_is_in_the_reference` in `crates/telperion-core/tests/parameter_reference.rs`;
run it with `TELPERION_WRITE_REFERENCE=1` to rewrite this file. Every wire row
is declared once, beside the code that reads it; edit the declaration, never this file.

Each row lists its type, unit, bounds and the ordinary family's default; the
stages that read it on the direct build (grow, plan, expand, cull, draw) and
how the growth path reads it; where it is refused and by what name; where it
lies dormant; its couplings and conflicting bounds; how a walk between two
families moves it; and the tuning dial it offers. A deprecated row is read by
no production stage and is kept on the wire for compatibility.
";

fn row(out: &mut String, entry: Entry, defaults: &Family) {
    let info = entry.info();
    let value = entry.get(defaults);
    let flag = if info.deprecated { " (deprecated)" } else { "" };
    let _ = write!(
        out,
        "\n### `{}`{flag}\n\n{}\n\n",
        entry.path(),
        meaning(info.meaning)
    );
    let default = value.number().map_or("unset".to_owned(), |v| v.to_string());
    let unit = if info.unit.is_empty() { "-" } else { info.unit };
    let _ = writeln!(
        out,
        "- {}, {unit}, {}, default {default}",
        value.kind().name(),
        bounds(info.bounds),
    );
    let _ = writeln!(
        out,
        "- read by {}; growth path: {}",
        stages(info.reads),
        growth(info.growth)
    );
    let _ = writeln!(out, "- checked: {}", check(info.check));
    if !info.applies.is_empty() {
        let _ = writeln!(out, "- dormant: {}", info.applies);
    }
    if !info.note.is_empty() {
        let _ = writeln!(out, "- note: {}", info.note);
    }
    let _ = writeln!(
        out,
        "- blend: {}; dial: {}",
        blend(info.blend),
        dial(info.dial)
    );
}

/// A doc comment as one paragraph.
pub(super) fn meaning(doc: &str) -> String {
    doc.lines().map(str::trim).collect::<Vec<_>>().join(" ")
}

fn number(v: f64) -> String {
    match v {
        f64::INFINITY => "∞".into(),
        f64::NEG_INFINITY => "-∞".into(),
        v => format!("{v}"),
    }
}

fn bounds(b: Bounds) -> String {
    if b == Bounds::FINITE {
        return "any finite value".into();
    }
    let open = if b.low_open { "(" } else { "[" };
    let range = format!("{open}{}, {}]", number(b.low), number(b.high));
    if b.zero {
        format!("0 or {range}")
    } else {
        range
    }
}

fn stages(reads: &[Stage]) -> String {
    let names: Vec<_> = reads
        .iter()
        .map(|s| match s {
            Stage::Grow => "grow",
            Stage::Plan => "plan",
            Stage::Expand => "expand",
            Stage::Cull => "cull",
            Stage::Draw => "draw",
        })
        .collect();
    names.join(", ")
}

fn growth(g: Growth) -> String {
    match g {
        Growth::Same => "as the direct build".into(),
        Growth::Only => "only the growth path reads it".into(),
        Growth::Ignored => "not read".into(),
        Growth::Differs(how) => how.into(),
    }
}

fn check(c: Option<Check>) -> String {
    let Some(c) = c else {
        return "by a named check or not at all (see note)".into();
    };
    let refusal = match c.refusal {
        Refusal::Value(name) => format!("invalid value `{name}`"),
        Refusal::Input(name) => format!("invalid input `{name}`"),
    };
    format!("{:?} site, rank {}, as {refusal}", c.site, c.rank)
}

fn blend(b: Blend) -> &'static str {
    match b {
        Blend::Linear => "linear",
        Blend::Weighted => "linear, clamped to its ends",
        Blend::Degrees => "degrees along the shorter arc",
        Blend::Count | Blend::Many => "rounded to the nearest",
        Blend::Down => "rounded down",
        Blend::Up => "rounded up",
        Blend::Density => "as the density it spaces",
        Blend::Kept => "the first family's",
        Blend::Coupled => "with the rows it is coupled to",
    }
}

fn dial(d: Dial) -> String {
    match d {
        Dial::Derived => "none".into(),
        Dial::Excluded(why) => format!("none: {why}"),
        Dial::Tuned(t) => {
            let window = t
                .window
                .map_or("its bounds".into(), |[a, b]| format!("[{a}, {b}]"));
            format!(
                "`{}` ({}), window {window}, steps {} and {}",
                t.id, t.ask, t.small, t.substantial
            )
        }
    }
}

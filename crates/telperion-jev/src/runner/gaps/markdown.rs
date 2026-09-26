//! `gaps.md`: one readable line per gap, grouped by class; the whole record
//! is `gaps.json`.
use super::{Gap, Kind};

/// Evidence items a gaps.md line shows, and the characters each keeps: a
/// person reads the file, and the beech's reached 1.3 MB (fn-157).
const SHOWN: usize = 3;
const CUT: usize = 240;

/// A gap's evidence as one readable line; the whole stays in gaps.json.
fn said(gap: &Gap) -> String {
    let cut = |e: &String| match e.char_indices().nth(CUT) {
        Some((at, _)) => format!("{}...", &e[..at]),
        None => e.clone(),
    };
    let mut line: Vec<String> = gap.evidence.iter().take(SHOWN).map(cut).collect();
    let rest = gap.evidence.len().saturating_sub(SHOWN);
    if rest > 0 {
        line.push(format!("{rest} more in gaps.json"));
    }
    line.join("; ")
}

pub fn markdown(gaps: &[Gap]) -> String {
    let mut out = String::from(
        "# Gaps\n\nEvery trait still failing after tuning, classed by the runner from what the run \
         recorded. The host reviews each line and writes any spec; the runner mints none.\n",
    );
    for (kind, title) in [
        (Kind::Identity, "Identity: the species waits on these"),
        (Kind::Global, "Global: backlog"),
        (Kind::Reachable, "Reachable: a live dial moves it"),
        (Kind::Unsourced, "Unsourced: no agreeing sources settled it"),
        (
            Kind::References,
            "References: no photograph to compare against",
        ),
    ] {
        out.push_str(&format!("\n## {title}\n\n"));
        let listed: Vec<&Gap> = gaps.iter().filter(|g| g.kind == kind).collect();
        if listed.is_empty() {
            out.push_str("None.\n");
        }
        for gap in listed {
            let specs = match gap.specs.is_empty() {
                true => String::new(),
                false => format!(" ({})", gap.specs.join(", ")),
            };
            out.push_str(&format!("- **{}**{specs}: {}\n", gap.trait_id, said(gap)));
        }
    }
    out
}

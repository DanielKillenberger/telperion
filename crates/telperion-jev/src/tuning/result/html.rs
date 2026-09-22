//! The end result as one page: the current tree's stills at a glance, the
//! outcome, and a card per gap. Self-contained, no script, images by path.
use super::{EndResult, GapEntry, Still};
use std::path::Path;

const STYLE: &str = "body{font:15px/1.45 system-ui,sans-serif;color:#23241f;background:#f3f1ec;margin:0;padding:24px 16px}\
.wrap{max-width:1400px;margin:0 auto}h1{font-size:1.6rem;margin:0 0 4px}h2{font-size:1.15rem;margin:28px 0 8px}\
.muted{color:#6b6d63}table{border-collapse:collapse;font-variant-numeric:tabular-nums}td{padding:3px 12px 3px 0;vertical-align:top}\
.stills{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:12px}\
.stills figure{margin:0}.stills img{width:100%;display:block;border:1px solid #d9d6cd;background:#000}\
figcaption{font-size:.8rem;color:#6b6d63;margin-top:4px}\
.gap{border:1px solid #d9d6cd;background:#faf9f6;padding:12px 14px;margin:10px 0}\
.gap h3{margin:0 0 6px;font-size:1rem}.pill{display:inline-block;font-size:.72rem;letter-spacing:.06em;text-transform:uppercase;\
padding:2px 8px;border-radius:2px;background:#e3ecd8;color:#2f4a1a;margin-left:8px}\
.pill.stalled{background:#f1e2cf;color:#6b3f10}.pill.handed{background:#dfe6f2;color:#233b66}.pill.passing{background:#d9efd7;color:#1d5a24}\
ul{margin:4px 0 8px;padding-left:1.2em}pre{background:#eeece6;padding:10px;overflow-x:auto;font-size:.8rem}\
.check{border-left:3px solid #8b8578;padding:6px 10px;background:#efede7;font-size:.9rem}";

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A still's `src`: relative to the run directory when it lies inside it,
/// otherwise the absolute path, so the page opens from disk either way.
fn src(still: &Still, run_dir: &Path) -> String {
    let path = Path::new(&still.path);
    match path.strip_prefix(run_dir) {
        Ok(rel) => rel.display().to_string(),
        Err(_) => format!("file://{}", path.display()),
    }
}

fn stills(items: &[Still], run_dir: &Path) -> String {
    let mut s = String::from("<div class=\"stills\">");
    for st in items.iter().filter(|s| !s.path.ends_with("-twin.png")) {
        s.push_str(&format!(
            "<figure><img src=\"{}\" alt=\"{} seed {}\"><figcaption>{} · seed {} · {}</figcaption></figure>",
            esc(&src(st, run_dir)), esc(&st.view), st.seed, esc(&st.view), st.seed,
            esc(&st.sha256[..12.min(st.sha256.len())])
        ));
    }
    s.push_str("</div>");
    s
}

fn pill(status: &str) -> String {
    let class = if status.starts_with("stalled") {
        "stalled"
    } else if status.starts_with("handed") {
        "handed"
    } else if status.starts_with("passing") {
        "passing"
    } else {
        ""
    };
    format!("<span class=\"pill {class}\">{}</span>", esc(status))
}

fn gap(g: &GapEntry) -> String {
    let mut s = format!(
        "<section class=\"gap\"><h3>{}. {}{}</h3><p>{}</p><table><tr><td>Latest route</td><td>{}</td></tr>",
        g.rank, esc(&g.id), pill(&g.status), esc(&g.priority),
        esc(g.latest_route.as_deref().unwrap_or("none"))
    );
    if let Some(spec) = &g.existing_spec {
        s.push_str(&format!("<tr><td>Existing spec</td><td>{}</td></tr>", esc(spec)));
    }
    let feasible = g.attempts.iter().filter(|a| a.feasible).count();
    s.push_str(&format!(
        "<tr><td>Attempts graded on it</td><td>{} evaluated, {} feasible</td></tr></table>",
        g.attempts.len(), feasible
    ));
    if !g.reviewer_words.is_empty() {
        s.push_str("<p class=\"muted\">Reviewer's words, most recent first</p><ul>");
        for w in &g.reviewer_words {
            s.push_str(&format!("<li>{}</li>", esc(w)));
        }
        s.push_str("</ul>");
    }
    s.push_str(&format!("<p class=\"check\">Check: {}</p></section>", esc(&g.check)));
    s
}

/// The page. `run_dir` is where it will be written, so stills inside it are
/// linked relatively.
pub fn page(r: &EndResult, run_dir: &Path) -> String {
    let o = &r.outcome;
    let mut s = format!(
        "<!doctype html><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
<title>Run result: {} seed {}</title><style>{STYLE}</style><div class=\"wrap\">\
<h1>Run result: {} at seed {}</h1><p class=\"muted\">{}</p>",
        esc(&o.preset), o.seed, esc(&o.preset), o.seed, esc(&r.meaning)
    );
    if let Some(c) = &o.current {
        s.push_str(&format!(
            "<h2>Current tree</h2><p>Trial <code>{}</code>, round {}, {}{}.</p>{}",
            esc(&c.key[..12.min(c.key.len())]), c.round, esc(&c.label),
            c.score_telemetry.map(|v| format!(", score telemetry {v:.4}")).unwrap_or_default(),
            stills(&c.stills, run_dir)
        ));
    } else {
        s.push_str("<h2>Current tree</h2><p>None: no candidate was ever current.</p>");
    }
    s.push_str(&format!(
        "<h2>Outcome</h2><table><tr><td>Stopped</td><td>{}</td></tr><tr><td>Bootstrap</td><td>{}</td></tr>\
<tr><td>Machine ready</td><td>{}</td></tr><tr><td>Owner acceptance</td><td>{}</td></tr>\
<tr><td>Adoptions kept / rolled back</td><td>{} / {}</td></tr>",
        esc(&o.stopped), o.bootstrap, o.machine_ready, esc(&o.owner_acceptance),
        o.adoptions_kept, o.adoptions_rolled_back
    ));
    if let Some(b) = o.budget.as_object() {
        for key in ["rounds", "evaluations", "images", "visual_passes"] {
            let cap = b.get(&format!("max_{key}")).map(|v| v.to_string()).unwrap_or_else(|| "?".into());
            let used = b.get(key).map(|v| v.to_string()).unwrap_or_else(|| "?".into());
            s.push_str(&format!("<tr><td>{}</td><td>{used} of {cap}</td></tr>", key.replace('_', " ")));
        }
    }
    s.push_str("</table>");
    s.push_str(&format!("<h2>Gaps ({})</h2><p class=\"muted\">{}</p>", r.gaps.len(), esc(&r.gaps_note)));
    if r.gaps.is_empty() {
        s.push_str("<p>No approved priorities: nothing was asked of this run.</p>");
    }
    for g in &r.gaps {
        s.push_str(&gap(g));
    }
    if let Some(c) = &o.current {
        s.push_str(&format!(
            "<h2>Current overlay</h2><pre>{}</pre>",
            esc(&serde_json::to_string_pretty(&c.overrides).unwrap_or_default())
        ));
    }
    s.push_str("</div>");
    s
}

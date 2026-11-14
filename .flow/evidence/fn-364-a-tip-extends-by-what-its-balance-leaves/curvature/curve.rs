// fn-364 CURVATURE (tooling): the outer 5 m of the most and least tortuous
// living tips' paths, axis by axis. Shared by both branches' runners; each
// supplies `stretch`.
use telperion_space::{Axis, Origin, Structure, Vec3};

const OUTER: f64 = 5.0;

/// One phytomer on a tip's path: axis, index.
type Step = (usize, usize);

/// The path from the tip of axis `i` down to the collar, tip first: each
/// phytomer whose node the path passes, and the bearer's node where an
/// axis leaves it.
fn path(s: &Structure, i: usize) -> Vec<Step> {
    let mut out = Vec::new();
    let (mut a, mut top) = (i, s.axes[i].phytomers.len());
    loop {
        // A bearer whose wood was let go holds fewer phytomers than the
        // node its axis stands at.
        for k in (0..top.min(s.axes[a].phytomers.len())).rev() {
            out.push((a, k));
        }
        match s.axes[a].origin {
            Origin::Seed => break,
            Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node } => {
                (a, top) = (parent, node + 1);
            }
            Origin::Continuation { parent } => {
                top = s.axes[parent].phytomers.len();
                a = parent;
            }
        }
    }
    out
}

fn base_of(s: &Structure, (a, k): Step) -> Vec3 {
    if k == 0 {
        s.axes[a].base
    } else {
        s.axes[a].phytomers[k - 1].tip
    }
}

fn seg(s: &Structure, st: Step) -> f64 {
    (s.axes[st.0].phytomers[st.1].tip - base_of(s, st)).length()
}

fn angle(a: Vec3, b: Vec3) -> f64 {
    let (la, lb) = (a.length(), b.length());
    if la < 1e-12 || lb < 1e-12 {
        return 0.0;
    }
    (a.dot(b) / (la * lb)).clamp(-1.0, 1.0).acos().to_degrees()
}

fn kind(a: &Axis) -> &'static str {
    match a.origin {
        Origin::Seed => "seed",
        Origin::Lateral { woken: true, .. } => "woken",
        Origin::Lateral { .. } => "lateral",
        Origin::Continuation { .. } => "continuation",
        Origin::Relay { .. } => "relay",
    }
}

/// The steps of the outer 5 m of `p` and the laid length they take.
fn outer(s: &Structure, p: &[Step]) -> (Vec<Step>, f64) {
    let (mut out, mut gone) = (Vec::new(), 0.0);
    for &st in p {
        if gone >= OUTER {
            break;
        }
        gone += seg(s, st);
        out.push(st);
    }
    (out, gone)
}

pub fn report(s: &Structure, stretch: &dyn Fn(&Structure, Step) -> Option<f64>, label: &str) {
    let living: Vec<usize> = (0..s.axes.len())
        .filter(|&i| s.axes[i].apex_end.is_none() && !s.axes[i].phytomers.is_empty())
        .collect();
    // Each living tip: its tortuosity over the outer 5 m, and its full path.
    let mut tips: Vec<(f64, f64, usize)> = Vec::new();
    for &i in &living {
        let p = path(s, i);
        let full: f64 = p.iter().map(|&st| seg(s, st)).sum();
        let (o, laid) = outer(s, &p);
        if laid < OUTER {
            continue;
        }
        let tip = s.axes[i].phytomers.last().unwrap().tip;
        let back = base_of(s, *o.last().unwrap());
        let chord = (tip - back).length();
        tips.push((laid / chord.max(1e-9), full, i));
    }
    let mut most = tips.clone();
    most.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut least: Vec<_> = tips.iter().filter(|t| t.1 >= 15.0).cloned().collect();
    least.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!("CURVE {label}: {} living tips, {} with 5 m of path", living.len(), tips.len());
    for (set, list) in [("most tortuous", &most), ("least tortuous (path >= 15 m)", &least)] {
        println!("\n### {label}, {set}\n");
        for &(t, full, i) in list.iter().take(5) {
            one(s, stretch, i, t, full);
        }
    }
}

fn one(s: &Structure, stretch: &dyn Fn(&Structure, Step) -> Option<f64>, i: usize, t: f64, full: f64) {
    let p = path(s, i);
    let (o, laid) = outer(s, &p);
    println!("Tip of axis {i}: tortuosity {t:.2} over the outer {laid:.2} m, path {full:.1} m from the collar\n");
    println!("| axis | kind | PA | birth | phytomers on path | laid m | stretch mean / min | turning inside, deg | junction, deg |");
    println!("|---|---|---|---|---|---|---|---|---|");
    let (mut inside_sum, mut junction_sum, mut axes, mut short) = (0.0, 0.0, 0, 0.0);
    let mut k = 0;
    while k < o.len() {
        let a = o[k].0;
        let mut run = Vec::new();
        while k < o.len() && o[k].0 == a {
            run.push(o[k]);
            k += 1;
        }
        // Tip first: put base to tip for the turning.
        run.reverse();
        let ax = &s.axes[a];
        let heads: Vec<Vec3> = run.iter().map(|&(_, j)| ax.phytomers[j].heading).collect();
        let inside: f64 = heads.windows(2).map(|w| angle(w[0], w[1])).sum();
        let laid: f64 = run.iter().map(|&st| seg(s, st)).sum();
        let st: Vec<f64> = run.iter().filter_map(|&x| stretch(s, x)).collect();
        let (mean, min) = if st.is_empty() {
            ("-".to_string(), "-".to_string())
        } else {
            (
                format!("{:.2}", st.iter().sum::<f64>() / st.len() as f64),
                format!("{:.2}", st.iter().cloned().fold(f64::MAX, f64::min)),
            )
        };
        short += run.iter().filter(|&&x| stretch(s, x).is_some_and(|v| v < 0.25)).map(|&x| seg(s, x)).sum::<f64>();
        // The junction: this axis's first phytomer on the path against the
        // bearer's heading where it leaves it, only where the path starts
        // this axis at its base.
        let junction = match (run.first().map(|x| x.1), ax.origin.parent()) {
            (Some(0), Some(par)) => {
                let node = match ax.origin {
                    Origin::Lateral { node, .. } | Origin::Relay { node, .. } => node,
                    _ => s.axes[par].phytomers.len().saturating_sub(1),
                };
                s.axes[par].phytomers.get(node).map(|q| angle(q.heading, ax.phytomers[0].heading))
            }
            _ => None,
        };
        if let Some(j) = junction {
            junction_sum += j;
        }
        inside_sum += inside;
        axes += 1;
        println!(
            "| {a} | {} | {} | {} | {} | {laid:.2} | {mean} / {min} | {inside:.0} | {} |",
            kind(ax),
            ax.pa,
            ax.birth,
            run.len(),
            junction.map_or("- (path enters mid-axis)".into(), |j| format!("{j:.0}"))
        );
    }
    println!(
        "\nTotals: turning inside axes {inside_sum:.0} deg, at junctions {junction_sum:.0} deg; {:.0} deg per metre; {:.2} axes per metre; laid at stretch under 0.25: {:.0}% of the {laid:.2} m.\n",
        (inside_sum + junction_sum) / laid,
        axes as f64 / laid,
        100.0 * short / laid
    );
}

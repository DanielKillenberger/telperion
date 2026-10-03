//! The R1 pass bands as the spec fixes them ("R1 pass bands"), judged on a
//! tree's scorecard. A band with no value (an unmeasurable reading) fails.
use serde_json::{json, Value};

fn f(v: &Value, path: &[&str]) -> Option<f64> {
    let mut x = v;
    for k in path {
        x = match k.parse::<usize>() {
            Ok(i) => x.get(i)?,
            Err(_) => x.get(*k)?,
        };
    }
    x.as_f64()
}

type Band = (&'static str, Option<f64>, bool);

fn within(v: Option<f64>, lo: f64, hi: f64) -> bool {
    v.is_some_and(|x| x >= lo && x <= hi)
}

pub fn judge(id: &str, s: &Value, fine: &Value) -> Value {
    let bole = f(s, &["t1_bole", "lowest_substantial"]);
    let any = f(fine, &["lowest_lateral_axis"]);
    let div = f(s, &["t1_bole", "leaders2"]);
    let majors = f(s, &["t2_major", "root04"]);
    let sec = f(s, &["t4_secondaries", "per_major_median"]);
    let rest = f(s, &["t4_secondaries", "len_over_rest_p25_p50_p75", "1"]);
    let shell = f(s, &["t5_terminal", "shell_over_interior"]);
    let up = f(s, &["t5_terminal", "upper_over_lower"]);
    let o1 = f(s, &["t6_ratio_by_order", "1", "1"]);
    let latm = f(fine, &["laterals_per_m"]);
    let gens = f(fine, &["generations_p50_p90_max", "0"]);
    let elev = f(fine, &["elevation_median"]);
        let ge = |v: Option<f64>, lo: f64| v.is_some_and(|x| x >= lo);
    let le = |v: Option<f64>, hi: f64| v.is_some_and(|x| x <= hi);
    let bands: Vec<Band> = match id {
        "european-beech" => vec![
            ("clear bole 0.18-0.30", bole, within(bole, 0.18, 0.30)),
            ("division 0.24-0.56", div, within(div, 0.24, 0.56)),
            ("majors 4-7", majors, within(majors, 4.0, 7.0)),
            ("secondaries 4-10", sec, within(sec, 4.0, 10.0)),
            ("len/rest 0.3-0.8", rest, within(rest, 0.3, 0.8)),
            ("shell/interior >= 1.3", shell, ge(shell, 1.3)),
            ("upper/lower >= 1.1", up, ge(up, 1.1)),
            ("o1 ratio 0.35-0.60", o1, within(o1, 0.35, 0.60)),
            ("laterals/m >= 1.3", latm, ge(latm, 1.3)),
            ("generations p50 <= 3", gens, le(gens, 3.0)),
        ],
        "oregon-white-oak" => {
            let gap = bole.zip(div).map(|(b, d)| d - b);
            vec![
                ("clear bole 0.14-0.32", bole, within(bole, 0.14, 0.32)),
                ("division 0.15-0.35, minus bole <= 0.10", div, within(div, 0.15, 0.35) && le(gap, 0.10)),
                ("majors 4-7", majors, within(majors, 4.0, 7.0)),
                ("secondaries 3-10", sec, within(sec, 3.0, 10.0)),
                ("len/rest 0.4-1.0", rest, within(rest, 0.4, 1.0)),
                ("shell/interior >= 1.1", shell, ge(shell, 1.1)),
                ("upper/lower 0.9-1.5", up, within(up, 0.9, 1.5)),
                ("o1 ratio 0.40-0.70", o1, within(o1, 0.40, 0.70)),
                ("laterals/m >= 1.3", latm, ge(latm, 1.3)),
                ("generations p50 <= 3", gens, le(gens, 3.0)),
            ]
        }
        "norway-spruce" => vec![
            ("lowest lateral <= 0.10", any, le(any, 0.10)),
            ("no division below 0.85", div, div.is_none_or(|d| d >= 0.85)),
            ("majors 0-1", majors, within(majors, 0.0, 1.0)),
            ("shell/interior >= 1.2", shell, ge(shell, 1.2)),
            ("upper/lower 0.7-1.6", up, within(up, 0.7, 1.6)),
            ("o1 ratio 0.08-0.25", o1, within(o1, 0.08, 0.25)),
            ("laterals/m >= 1.3", latm, ge(latm, 1.3)),
            ("generations p50 <= 3", gens, le(gens, 3.0)),
            ("elevation -5..30", elev, within(elev, -5.0, 30.0)),
        ],
        _ => vec![],
    };
    let met = bands.iter().filter(|b| b.2).count();
    json!({
        "met": met, "of": bands.len(), "share": if bands.is_empty() { 0.0 } else { met as f64 / bands.len() as f64 },
        "bands": bands.iter().map(|(n, v, ok)| json!({"band": n, "value": v, "pass": ok})).collect::<Vec<_>>(),
    })
}

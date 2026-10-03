//! R1's votes as the host fixed them after round 1 (spec, "Host decisions
//! after R1 round 1", item 5): required checks, supplementary bands of which
//! a share must pass, and diagnostics that never vote. The visual gate is
//! judged on the sheets and is not here. A missing measurement fails.
use serde_json::{json, Value};

fn f(v: &Value, path: &[&str]) -> Option<f64> {
    let mut x = v;
    for k in path {
        x = match k.parse::<usize>() {
            Ok(i) => x.get(i)?,
            Err(_) => x.get(*k)?,
        };
    }
    x.as_f64().filter(|x| x.is_finite())
}

type Check = (&'static str, Option<f64>, bool);

fn within(v: Option<f64>, lo: f64, hi: f64) -> bool {
    v.is_some_and(|x| x >= lo && x <= hi)
}

fn list(c: &[Check]) -> Vec<Value> {
    c.iter().map(|(n, v, ok)| json!({"check": n, "value": v, "pass": ok})).collect()
}

/// `row` is the grown tree's row, `today` today's row of the same preset
/// and seed, `ms` the law's warm growth time.
pub fn judge(id: &str, row: &Value, today: &Value, ms: f64) -> Value {
    let s = &row["score"];
    let fine = &row["fine"];
    let tr = &row["trace"];
    let bole = f(s, &["t1_bole", "lowest_substantial"]);
    let div = f(s, &["t1_bole", "leaders2"]);
    let sec = f(s, &["t4_secondaries", "per_major_median"]);
    let rest = f(s, &["t4_secondaries", "len_over_rest_p25_p50_p75", "1"]);
    let o1 = f(s, &["t6_ratio_by_order", "1", "1"]);
    let fine_m = f(fine, &["fine_m"]);
    let today_fine = f(today, &["fine", "fine_m"]);
    let today_ms = f(today, &["time_ms", "warm_median"]);
    let born = f(tr, &["birth_elev_median"]);
    let rise = f(tr, &["rise_median"]);
    let fine_ok = fine_m.zip(today_fine).is_some_and(|(a, b)| a >= b);
    let time_ok = today_ms.is_some_and(|t| ms <= 1.5 * t);
    let common: [Check; 2] = [
        ("fine wood >= today's", fine_m, fine_ok),
        ("growth ms <= 1.5 x today's skeleton", Some(ms), time_ok),
    ];
    let (mut req, sup, need): (Vec<Check>, Vec<Check>, usize) = match id {
        "european-beech" | "oregon-white-oak" => {
            let beech = id == "european-beech";
            let (b_lo, b_hi, d_lo, d_hi) = if beech { (0.18, 0.30, 0.24, 0.56) } else { (0.14, 0.32, 0.15, 0.35) };
            // Troll: axes lean at birth and their bases turn up; Rauh: born
            // orthotropic and staying so (probe definitions, round 2).
            let dev = if beech {
                ("trace: born <= 45 deg, rise >= 10 deg (Troll)", rise, born.is_some_and(|b| b <= 45.0) && rise.is_some_and(|r| r >= 10.0))
            } else {
                ("trace: born >= 45 deg, rise >= -5 deg (Rauh)", born, born.is_some_and(|b| b >= 45.0) && rise.is_some_and(|r| r >= -5.0))
            };
            let (s_lo, r_lo, r_hi, o_lo, o_hi) = if beech { (4.0, 0.3, 0.8, 0.35, 0.60) } else { (3.0, 0.4, 1.0, 0.40, 0.70) };
            (
                vec![("clear bole", bole, within(bole, b_lo, b_hi)), ("division (projected)", div, within(div, d_lo, d_hi)), dev],
                vec![
                    ("junction ratio o1", o1, within(o1, o_lo, o_hi)),
                    ("secondaries per major", sec, within(sec, s_lo, 10.0)),
                    ("secondary length / remaining", rest, within(rest, r_lo, r_hi)),
                ],
                2,
            )
        }
        "norway-spruce" => {
            let top = f(tr, &["stem_top_share"]);
            let elev = f(fine, &["elevation_median"]);
            let low = f(fine, &["lowest_lateral_axis"]);
            (
                vec![
                    ("persistent leader (stem top >= 0.9 H)", top, top.is_some_and(|t| t >= 0.9)),
                    ("elevation median -5..30", elev, within(elev, -5.0, 30.0)),
                    ("no division below 0.85", div, div.is_none_or(|d| d >= 0.85)),
                ],
                vec![("lowest lateral (axis >= 0.05 H) <= 0.10", low, low.is_some_and(|x| x <= 0.10)), ("junction ratio o1", o1, within(o1, 0.08, 0.25))],
                1,
            )
        }
        _ => (vec![], vec![], 0),
    };
    req.extend(common);
    let req_ok = req.iter().all(|c| c.2);
    let sup_met = sup.iter().filter(|c| c.2).count();
    json!({
        "required_pass": req_ok, "required": list(&req),
        "supplementary_met": sup_met, "supplementary_of": sup.len(), "supplementary_need": need,
        "supplementary_pass": sup_met >= need, "supplementary": list(&sup),
        "numeric_pass": req_ok && sup_met >= need,
    })
}

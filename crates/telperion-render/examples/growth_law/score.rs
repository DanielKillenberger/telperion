//! Round 9, the bare scorecard's tree measures (Astra's seven bare targets,
//! `ASTRA-BARE-TARGETS.md`), on any pipeline tree: shipped or probe. The
//! image measures (W(h), patches, fine-line straightness) live in `score.py`
//! and read the bare and empty frames this run writes.
use telperion_core::math::Vec3;
use telperion_core::tree::{BudFate, NodeKind, Tree};
use telperion_render::Camera;

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

fn pct(v: &[f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f64 * q).round() as usize]
}

struct T<'a> {
    t: &'a Tree,
    kids: Vec<Vec<usize>>,
    cont: Vec<Option<usize>>,
}

impl T<'_> {
    fn pos(&self, i: usize) -> Vec3 {
        self.t.nodes[i].position
    }
    fn r(&self, i: usize) -> f64 {
        self.t.nodes[i].radius
    }
    fn par(&self, i: usize) -> usize {
        self.t.nodes[i].parent.unwrap() as usize
    }
    fn lateral(&self, i: usize) -> bool {
        let n = &self.t.nodes[i];
        n.shoot.bud_fate == BudFate::Lateral || n.codominant.is_some()
    }
    fn seg(&self, i: usize) -> f64 {
        self.t.nodes[i].parent.map_or(0.0, |p| (self.pos(i) - self.pos(p as usize)).length())
    }
    fn axis(&self, s: usize) -> Vec<usize> {
        let mut out = vec![s];
        while let Some(c) = self.cont[*out.last().unwrap()] {
            out.push(c);
        }
        out
    }
    fn len(&self, s: usize) -> f64 {
        self.axis(s).iter().map(|&i| self.seg(i)).sum()
    }
}

/// Projected image position at the pinned camera, 960 × 720.
pub fn project(cam: &Camera, p: Vec3) -> (f64, f64) {
    let fwd = (cam.target - cam.position).normalized();
    let right = fwd.cross(Vec3::Y).normalized();
    let up = right.cross(fwd);
    let v = p - cam.position;
    let z = v.dot(fwd);
    let f = 360.0 / (cam.field_of_view.to_radians() / 2.0).tan();
    (480.0 + f * v.dot(right) / z, 360.0 - f * v.dot(up) / z)
}

pub fn score(t: &Tree, cam: &Camera) -> serde_json::Value {
    let n = t.nodes.len();
    let mut kids = vec![vec![]; n];
    for i in 1..n {
        kids[t.nodes[i].parent.unwrap() as usize].push(i);
    }
    let mut s = T { t, kids, cont: vec![None; n] };
    for i in 1..n {
        let p = s.par(i);
        if !s.lateral(i) && s.cont[p].is_none() {
            s.cont[p] = Some(i);
        }
    }
    let h = (0..n).map(|i| s.pos(i).y).fold(0.0, f64::max);
    let root_r = s.r(0);
    let trunk = s.axis(0);
    let mut on_trunk = vec![false; n];
    for &i in &trunk {
        on_trunk[i] = true;
    }
    let lat: Vec<usize> = (1..n).filter(|&i| s.lateral(i)).collect();
    let alen: Vec<f64> = {
        let mut v = vec![0.0; n];
        for &i in &lat {
            v[i] = s.len(i);
        }
        v
    };
    let ratio = |i: usize| s.r(i) / s.r(s.par(i)).max(1e-12);

    // (1) Clear bole and division band.
    let substantial = |i: usize| ratio(i) >= 0.3 && alen[i] >= 1.0;
    let lowest_sub = lat.iter().filter(|&&i| on_trunk[s.par(i)] && substantial(i)).map(|&i| s.pos(s.par(i)).y).fold(h, f64::min);
    let lowest_any = lat.iter().filter(|&&i| on_trunk[s.par(i)]).map(|&i| s.pos(s.par(i)).y).fold(h, f64::min);
    let lowest_fine = (1..n).filter(|&i| t.nodes[i].kind != NodeKind::Structural).map(|i| s.pos(i).y).fold(h, f64::min);
    let bins = 100;
    let mut cross_max = vec![0.0f64; bins];
    let mut crossing: Vec<Vec<f64>> = vec![vec![]; bins];
    for i in 1..n {
        let (a, b) = (s.pos(s.par(i)).y, s.pos(i).y);
        let (lo, hi) = (a.min(b), a.max(b));
        let (k0, k1) = (((lo / h) * bins as f64).ceil() as usize, ((hi / h) * bins as f64).floor() as usize);
        for k in k0..=k1.min(bins - 1) {
            if (k as f64 / bins as f64) * h > lo && (k as f64 / bins as f64) * h <= hi {
                crossing[k].push(s.r(i));
                cross_max[k] = cross_max[k].max(s.r(i));
            }
        }
    }
    let comparable: Vec<usize> = (0..bins).map(|k| crossing[k].iter().filter(|&&r| r >= 0.5 * cross_max[k]).count()).collect();
    let first = |m: usize| (0..bins).find(|&k| comparable[k] >= m).map(|k| k as f64 / bins as f64);

    // (2) Major axes: laterals born at ≥ 0.4 of the root radius, persistent
    // (axis ≥ 0.2 H); and laterals off the trunk axis at ≥ 0.4 of the local
    // trunk radius, persistent.
    let persistent = |i: usize| alen[i] >= 0.2 * h;
    let major_root: Vec<usize> = lat.iter().copied().filter(|&i| s.r(i) >= 0.4 * root_r && persistent(i)).collect();
    let major_local: Vec<usize> = lat.iter().copied().filter(|&i| on_trunk[s.par(i)] && ratio(i) >= 0.4 && persistent(i)).collect();
    let mut major: Vec<usize> = major_root.clone();
    for &i in &major_local {
        if !major.contains(&i) {
            major.push(i);
        }
    }
    // The trunk's own leader counts as a path when it reaches past mid-height.
    let mut paths: Vec<Vec<usize>> = major.iter().map(|&i| {
        let mut v = vec![s.par(i)];
        v.extend(s.axis(i));
        v
    }).collect();
    if s.pos(*trunk.last().unwrap()).y >= 0.5 * h {
        paths.push(trunk.clone());
    }
    // Projected headings per third: the chord of each path's part inside the
    // band, its signed angle from image vertical.
    let mut thirds = vec![];
    for k in 0..3 {
        let (y0, y1) = (k as f64 / 3.0 * h, (k + 1) as f64 / 3.0 * h);
        let mut ang = vec![];
        for pth in &paths {
            let pts: Vec<Vec3> = pth.iter().map(|&i| s.pos(i)).filter(|p| p.y >= y0 && p.y < y1).collect();
            if pts.len() < 2 {
                continue;
            }
            let (a, b) = (project(cam, pts[0]), project(cam, *pts.last().unwrap()));
            let (dx, dy) = (b.0 - a.0, a.1 - b.1);
            if dx.hypot(dy) < 4.0 {
                continue;
            }
            ang.push(dx.atan2(dy).to_degrees());
        }
        let abs: Vec<f64> = ang.iter().map(|a| a.abs()).collect();
        let m = ang.iter().sum::<f64>() / ang.len().max(1) as f64;
        let sd = (ang.iter().map(|a| (a - m).powi(2)).sum::<f64>() / ang.len().max(1) as f64).sqrt();
        thirds.push(serde_json::json!({"paths": ang.len(), "abs_median": median(abs), "signed_sd": sd}));
    }
    // Separation heights of the major axes, as shares of H (staggering).
    let mut births: Vec<f64> = major.iter().map(|&i| s.pos(s.par(i)).y / h).collect();
    births.sort_by(|a, b| a.total_cmp(b));

    // (4) Secondaries along each major path.
    let (mut sec_n, mut sec_s, mut sec_len, mut lat_pm) = (vec![], vec![], vec![], vec![]);
    for &m in &major {
        let ax = s.axis(m);
        let total: f64 = ax.iter().map(|&i| s.seg(i)).sum();
        let (mut along, mut count, mut all) = (0.0, 0usize, 0usize);
        for &p in &ax {
            along += s.seg(p);
            for &c in &s.kids[p] {
                if Some(c) == s.cont[p] {
                    continue;
                }
                all += 1;
                if ratio(c) >= 0.3 && alen[c] >= 1.0 {
                    count += 1;
                    sec_s.push(along / total);
                    let rest = total - along;
                    if rest > 0.5 {
                        sec_len.push(alen[c] / rest);
                    }
                }
            }
        }
        sec_n.push(count as f64);
        lat_pm.push(all as f64 / total.max(1e-9));
    }

    // (5) Terminal wood: terminal runs (tip back to a lateral start, a
    // branching node or structural wood). Star-shaped crown from its centre:
    // per direction bin (8 elevations × 16 azimuths) the p98 distance R;
    // shell where d ≥ 0.75 R; densities per bin volume (R³ dΩ / 3).
    let structural = |i: usize| t.nodes[i].kind == NodeKind::Structural;
    let mut term = vec![false; n];
    for i in 1..n {
        if structural(i) || !s.kids[i].is_empty() {
            continue;
        }
        let mut j = i;
        loop {
            term[j] = true;
            let p = s.par(j);
            if s.lateral(j) || s.kids[p].len() != 1 || structural(p) {
                break;
            }
            j = p;
        }
    }
    let tw: Vec<(Vec3, f64)> = (1..n).filter(|&i| term[i]).map(|i| ((s.pos(i) + s.pos(s.par(i))) * 0.5, s.seg(i))).collect();
    let ys: Vec<f64> = tw.iter().map(|x| x.0.y).collect();
    let (cb, ct) = (pct(&ys, 0.02), pct(&ys, 1.0));
    let c = Vec3::new(0.0, 0.5 * (cb + ct), 0.0);
    let (ne, na) = (8usize, 16usize);
    let bin = |p: Vec3| {
        let d = p - c;
        let el = (d.y / d.length().max(1e-9)).clamp(-1.0, 1.0);
        let e = (((el + 1.0) / 2.0) * ne as f64).floor().min(ne as f64 - 1.0) as usize;
        let a = ((d.z.atan2(d.x) / std::f64::consts::TAU + 0.5) * na as f64).floor().min(na as f64 - 1.0) as usize;
        e * na + a
    };
    let mut dists: Vec<Vec<f64>> = vec![vec![]; ne * na];
    for (p, _) in &tw {
        dists[bin(*p)].push((*p - c).length());
    }
    let rr: Vec<f64> = dists.iter().map(|v| pct(v, 0.98)).collect();
    // Each elevation bin spans equal sin(el), so equal solid angle per bin.
    let vol = |b: usize| if rr[b].is_nan() { 0.0 } else { rr[b].powi(3) };
    let (mut shell, mut inner, mut up_l, mut lo_l) = (0.0, 0.0, 0.0, 0.0);
    for (p, l) in &tw {
        let b = bin(*p);
        if (*p - c).length() >= 0.75 * rr[b] {
            shell += l;
        } else {
            inner += l;
        }
        if p.y >= c.y { up_l += l } else { lo_l += l }
    }
    let total_v: f64 = (0..ne * na).map(vol).sum();
    let shell_v = total_v * (1.0 - 0.75f64.powi(3));
    let inner_v = total_v * 0.75f64.powi(3);
    let up_v: f64 = (0..ne * na).filter(|b| b / na >= ne / 2).map(vol).sum();
    let lo_v = total_v - up_v;

    // (6) Junction radius ratios by order (axes ≥ 1 m) and lateral axes by
    // birth radius against the root.
    let mut order = vec![0usize; n];
    for i in 1..n {
        order[i] = order[s.par(i)] + usize::from(s.lateral(i));
    }
    let mut by_order: Vec<Vec<f64>> = vec![vec![]; 5];
    for &i in &lat {
        if alen[i] >= 1.0 {
            by_order[order[i].min(4)].push(ratio(i));
        }
    }
    let band = |lo: f64, hi: f64| lat.iter().filter(|&&i| alen[i] >= 1.0 && s.r(i) >= lo * root_r && s.r(i) < hi * root_r).count();

    // (7, geometric) Fine wood at the photograph's scale: every
    // non-structural axis projected, scaled so the tree is 370 px tall,
    // resampled every 7 px of projected arc; turning angle between chords,
    // and orientation agreement cos 2Δθ between chords of different axes
    // whose midpoints lie within 7 px.
    let (top_px, base_px) = (project(cam, Vec3::new(0.0, h, 0.0)).1, project(cam, Vec3::ZERO).1);
    let k = 370.0 / (base_px - top_px).abs().max(1.0);
    let step = 7.0;
    let (mut turns, mut chords) = (vec![], vec![]);
    for i in 1..n {
        if structural(i) || !(s.lateral(i) || structural(s.par(i))) {
            continue;
        }
        let mut pts = vec![s.pos(s.par(i))];
        pts.extend(s.axis(i).iter().map(|&j| s.pos(j)));
        let px: Vec<(f64, f64)> = pts.iter().map(|&p| {
            let (x, y) = project(cam, p);
            (x * k, y * k)
        }).collect();
        let mut acc = vec![0.0];
        for w in px.windows(2) {
            acc.push(acc.last().unwrap() + (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1));
        }
        let total = *acc.last().unwrap();
        if total < 2.0 * step {
            continue;
        }
        let at = |sv: f64| {
            let j = acc.partition_point(|&a| a < sv).clamp(1, px.len() - 1);
            let f = (sv - acc[j - 1]) / (acc[j] - acc[j - 1]).max(1e-12);
            (px[j - 1].0 + (px[j].0 - px[j - 1].0) * f, px[j - 1].1 + (px[j].1 - px[j - 1].1) * f)
        };
        let m = (total / step).floor() as usize;
        let q: Vec<(f64, f64)> = (0..=m).map(|j| at(j as f64 * step)).collect();
        let dirs: Vec<f64> = q.windows(2).map(|w| (w[1].1 - w[0].1).atan2(w[1].0 - w[0].0)).collect();
        for w in dirs.windows(2) {
            let mut d = (w[1] - w[0]).rem_euclid(std::f64::consts::TAU);
            if d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            turns.push(d.abs().to_degrees());
        }
        for (j, w) in q.windows(2).enumerate() {
            chords.push(((w[0].0 + w[1].0) / 2.0, (w[0].1 + w[1].1) / 2.0, dirs[j], i));
        }
    }
    // Neighbour agreement on a grid of 7 px cells, a sample of chords.
    let mut grid: std::collections::HashMap<(i64, i64), Vec<usize>> = std::collections::HashMap::new();
    for (j, c) in chords.iter().enumerate() {
        grid.entry(((c.0 / step).floor() as i64, (c.1 / step).floor() as i64)).or_default().push(j);
    }
    let (mut agree, mut pairs) = (0.0, 0usize);
    for (j, c) in chords.iter().enumerate().step_by(7) {
        let (gx, gy) = ((c.0 / step).floor() as i64, (c.1 / step).floor() as i64);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for &o in grid.get(&(gx + dx, gy + dy)).map_or(&[][..], |v| &v[..]) {
                    let d = &chords[o];
                    if o == j || d.3 == c.3 || (d.0 - c.0).hypot(d.1 - c.1) > step {
                        continue;
                    }
                    agree += (2.0 * (d.2 - c.2)).cos();
                    pairs += 1;
                }
            }
        }
    }
    let r3 = |x: f64| (x * 1000.0).round() / 1000.0;
    serde_json::json!({
        "height_m": r3(h),
        "t1_bole": {"lowest_substantial": r3(lowest_sub / h), "lowest_any_lateral": r3(lowest_any / h), "lowest_fine": r3(lowest_fine / h),
            "leaders2": first(2), "leaders3": first(3), "leaders4": first(4)},
        "t2_major": {"root04": major_root.len(), "local04": major_local.len(), "union": major.len(), "paths": paths.len(),
            "births": births.iter().map(|&x| r3(x)).collect::<Vec<_>>(), "thirds": thirds},
        "t4_secondaries": {"per_major_median": median(sec_n.clone()), "per_major": sec_n, "s_over_l_median": r3(median(sec_s)),
            "len_over_rest_p25_p50_p75": [r3(pct(&sec_len, 0.25)), r3(pct(&sec_len, 0.5)), r3(pct(&sec_len, 0.75))],
            "laterals_per_m_on_major_median": r3(median(lat_pm))},
        "t5_terminal": {"m": r3(shell + inner), "shell_over_interior": r3((shell / shell_v) / (inner / inner_v).max(1e-12)),
            "upper_over_lower": r3((up_l / up_v.max(1e-9)) / (lo_l / lo_v.max(1e-9)).max(1e-12)), "shell_share": r3(shell / (shell + inner).max(1e-9))},
        "t6_ratio_by_order": by_order.iter().map(|v| [r3(pct(v, 0.25)), r3(pct(v, 0.5)), r3(pct(v, 0.75))]).collect::<Vec<_>>(),
        "t6_order_counts": by_order.iter().map(Vec::len).collect::<Vec<_>>(),
        "t6_axes_by_root_ratio": {"ge04": band(0.4, 9.0), "015_04": band(0.15, 0.4), "005_015": band(0.05, 0.15), "002_005": band(0.02, 0.05)},
        "t7_geom": {"scale_px_per_px": r3(k), "turn_deg_p50_p75": [r3(pct(&turns, 0.5)), r3(pct(&turns, 0.75))], "turns": turns.len(),
            "neighbour_cos2": r3(agree / pairs.max(1) as f64), "pairs": pairs},
    })
}

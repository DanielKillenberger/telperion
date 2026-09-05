//! Native tooling only. Dimensions use metres and Y-up geometry, never field occupancy.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use telperion_core::{
    foliage::{transform_point, Element, Instances},
    tree::{NodeKind, Tree},
};
fn scalar(value: impl Into<Value>, status: &str) -> Value {
    json!({"status":status,"value":value.into()})
}
fn missing(reason: &str) -> Value {
    json!({"status":"unavailable","reason":reason})
}
fn distribution(mut values: Vec<f64>, status: &str) -> Value {
    if values.is_empty() {
        return missing("no retained foliage");
    }
    values.sort_by(f64::total_cmp);
    let n = values.len();
    json!({"status":status,"min":values[0],"max":values[n-1],"median":(values[(n-1)/2]+values[n/2])/2.})
}
/// The wood positions are the actual surface mesh, not swept-sphere bounds.
/// Existing prototypes have no connector exclusion metadata: dimensions remain estimates.
pub fn measure(
    tree: &Tree,
    wood: &[f32],
    element: &Element,
    pre_cull: usize,
    kept: &Instances,
) -> Result<Value, String> {
    if tree.nodes.iter().any(|n| {
        !n.position.is_finite()
            || ![n.radius, n.start_radius, n.base_radius]
                .iter()
                .all(|v| v.is_finite())
    }) || wood.iter().any(|v| !v.is_finite())
        || element.positions.iter().any(|v| !v.is_finite())
        || kept.matrices.iter().flatten().any(|v| !v.is_finite())
    {
        return Err("non_finite: geometry or transform".into());
    }
    tree.validate_solved()
        .map_err(|e| format!("invalid: {e:?}"))?;
    element.validate().map_err(|e| format!("invalid: {e:?}"))?;
    kept.validate().map_err(|e| format!("invalid: {e:?}"))?;
    if !wood.len().is_multiple_of(3) || kept.matrices.len() > pre_cull {
        return Err("invalid: wood positions or retained accounting".into());
    }
    let mut m = json!({});
    m["growth"] = json!({"status":if tree.diagnostics.complete(){"complete"}else{"truncated"},"node_capped":tree.diagnostics.node_capped,"level_capped":tree.diagnostics.level_capped,"attraction_capped":tree.diagnostics.attraction_capped});
    m["nodes"] = scalar(tree.nodes.len(), "measured");
    let ground = tree.nodes.first().map_or(0., |n| n.position.y);
    let wood_top = wood
        .chunks_exact(3)
        .map(|p| p[1] as f64 - ground)
        .reduce(f64::max);
    m["wood_height_m"] = wood_top.map_or_else(
        || missing("wood surface absent"),
        |v| scalar(v.max(0.), "measured"),
    );
    let bounds = kept
        .bounds(element)
        .map_err(|e| format!("invalid: {e:?}"))?;
    m["height_m"] = wood_top.map_or_else(
        || missing("wood surface absent"),
        |v| {
            scalar(
                v.max(bounds.map_or(0., |b| b.max.y - ground)).max(0.),
                "measured",
            )
        },
    );
    for key in [
        "crown_width_m",
        "crown_span_x_m",
        "crown_span_z_m",
        "crown_base_m",
        "crown_width_height_ratio",
    ] {
        m[key] = missing("no retained foliage");
    }
    if let Some(b) = bounds {
        let x = b.max.x - b.min.x;
        let z = b.max.z - b.min.z;
        for (key, v) in [
            ("crown_width_m", x.max(z)),
            ("crown_span_x_m", x),
            ("crown_span_z_m", z),
            ("crown_base_m", b.min.y - ground),
        ] {
            m[key] = scalar(v, "measured");
        }
        if let Some(h) = m["height_m"]["value"].as_f64().filter(|h| *h > 0.) {
            m["crown_width_height_ratio"] = scalar(x.max(z) / h, "measured");
        }
    }
    let mut dbh = Vec::new();
    for n in tree
        .nodes
        .iter()
        .skip(1)
        .filter(|n| n.kind == NodeKind::Structural)
    {
        let p = &tree.nodes[n.parent.unwrap() as usize];
        let a = p.position.y - ground;
        let b = n.position.y - ground;
        if (a <= 1.3 && b > 1.3) || (b <= 1.3 && a > 1.3) {
            dbh.push(2. * (n.start_radius + (n.radius - n.start_radius) * (1.3 - a) / (b - a)));
        }
    }
    m["dbh_m"] = match dbh.as_slice() {
        [v] => scalar(*v, "measured_proxy"),
        [] => missing("no structural edge crosses breast height"),
        _ => {
            json!({"status":"ambiguous","reason":"multiple structural stems cross breast height","diameters_m":dbh})
        }
    };
    m["dbh_measurement_height_m"] = scalar(1.3, "defined");
    let mut children = vec![Vec::new(); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        children[n.parent.unwrap() as usize].push(i);
    }
    let mut axis = vec![0; tree.nodes.len()];
    let mut order = vec![0usize; tree.nodes.len()];
    let mut lengths = vec![0.];
    let mut histogram = BTreeMap::<usize, usize>::new();
    let mut insertion = Vec::new();
    for (p, kids) in children.iter().enumerate() {
        let dominant = kids.iter().copied().max_by(|a, b| {
            tree.nodes[*a]
                .start_radius
                .total_cmp(&tree.nodes[*b].start_radius)
                .then_with(|| b.cmp(a))
        });
        for &i in kids {
            let lateral = Some(i) != dominant;
            order[i] = order[p] + usize::from(lateral);
            axis[i] = if lateral {
                lengths.push(0.);
                *histogram.entry(order[i]).or_default() += 1;
                if order[i] == 1 {
                    insertion.push(tree.nodes[p].position.y - ground);
                }
                lengths.len() - 1
            } else {
                axis[p]
            };
            lengths[axis[i]] += (tree.nodes[i].position - tree.nodes[p].position).length();
        }
    }
    m["branch_count"] = scalar(lengths.len().saturating_sub(1), "estimated");
    m["branch_order"] = scalar(json!(histogram), "estimated");
    m["branch_lengths_m"] = scalar(json!(&lengths[1..]), "estimated");
    m["trunk_axis_length_m"] = scalar(lengths[0], "estimated");
    m["primary_insertion_m"] = scalar(json!(insertion), "estimated");
    m["stored_branch_runs"] = scalar(
        tree.nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Branch)
            .map(|n| n.branch)
            .collect::<BTreeSet<_>>()
            .len(),
        "measured",
    );
    m["twig_count"] = scalar(
        tree.nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Twig)
            .map(|n| n.branch)
            .collect::<BTreeSet<_>>()
            .len(),
        "measured",
    );
    for (key, n) in [
        ("units_per_instance", 1),
        ("pre_cull_instances", pre_cull),
        ("retained_instances", kept.matrices.len()),
        ("foliage_units", kept.matrices.len()),
        ("discarded_units", pre_cull - kept.matrices.len()),
    ] {
        m[key] = scalar(n, "measured");
    }
    let mut area = 0.;
    let mut lengths = Vec::new();
    let mut widths = Vec::new();
    let extent = |component: fn(&telperion_core::math::Vec3) -> f64| {
        let lo = element
            .positions
            .iter()
            .map(component)
            .reduce(f64::min)
            .unwrap_or(0.);
        let hi = element
            .positions
            .iter()
            .map(component)
            .reduce(f64::max)
            .unwrap_or(0.);
        hi - lo
    };
    for mat in &kept.matrices {
        let scale = |c: usize| {
            ((mat[c] as f64).powi(2) + (mat[c + 1] as f64).powi(2) + (mat[c + 2] as f64).powi(2))
                .sqrt()
        };
        lengths.push(extent(|p| p.y) * scale(4));
        widths.push(extent(|p| p.x) * scale(0));
        for tri in element.indices.chunks_exact(3) {
            let a = transform_point(mat, element.positions[tri[0] as usize]);
            let b = transform_point(mat, element.positions[tri[1] as usize]);
            let c = transform_point(mat, element.positions[tri[2] as usize]);
            let triangle = (b - a).cross(c - a).length() / 2.;
            if !triangle.is_finite() {
                return Err("non_finite: triangle area".into());
            }
            if triangle <= 0. {
                return Err("degenerate: foliage triangle".into());
            }
            area += triangle;
        }
    }
    if !area.is_finite() {
        return Err("non_finite: total area".into());
    }
    m["leaf_area_m2"] = if area > 0. {
        scalar(area, "estimated")
    } else {
        missing("empty foliage geometry")
    };
    m["foliage_length_m"] = distribution(lengths, "estimated");
    m["foliage_width_m"] = distribution(widths, "estimated");
    m["foliage_geometry_note"]=json!("Whole prototype dimensions and triangle surface area; blade/needle subset excluding petiole/peg unavailable. Area is not projected area; sheet triangles counted once. Generic element is not proof of species anatomy.");
    fn has_null(v: &Value) -> bool {
        match v {
            Value::Null => true,
            Value::Array(a) => a.iter().any(has_null),
            Value::Object(o) => o.values().any(has_null),
            _ => false,
        }
    }
    if has_null(&m) {
        return Err("non_finite: derived metric overflow".into());
    }
    Ok(m)
}
pub fn compare(profile: &Value, metrics: &Value) -> Result<(bool, Value), String> {
    let targets = profile["metrics"]
        .as_object()
        .filter(|m| !m.is_empty())
        .ok_or("invalid profile metrics")?;
    let mut pass = metrics["growth"]["status"] != "truncated";
    let mut checks = json!({});
    let mut gates = 0;
    for (key, target) in targets {
        let gating = match target["classification"].as_str() {
            Some("gating") => true,
            Some("contextual") => false,
            _ => return Err(format!("invalid classification: {key}")),
        };
        gates += usize::from(gating);
        let actual = &metrics[key];
        let range = if target["range"].is_null() {
            None
        } else {
            let a = target["range"]
                .as_array()
                .filter(|a| a.len() == 2)
                .ok_or_else(|| format!("invalid range: {key}"))?;
            let lo = a[0].as_f64().ok_or("invalid range lower bound")?;
            let hi = a[1].as_f64().ok_or("invalid range upper bound")?;
            if lo > hi {
                return Err(format!("reversed range: {key}"));
            }
            Some((lo, hi))
        };
        if gating && range.is_none() {
            return Err(format!("gating range missing: {key}"));
        }
        let measured = matches!(
            actual["status"].as_str(),
            Some("measured" | "measured_proxy")
        );
        let low = actual["min"].as_f64().or(actual["value"].as_f64());
        let high = actual["max"].as_f64().or(actual["value"].as_f64());
        let inside = match (range, low, high) {
            (Some((lo, hi)), Some(a), Some(b)) => a >= lo && b <= hi,
            _ => false,
        };
        let status = if !gating {
            "contextual"
        } else if !measured || low.is_none() || high.is_none() {
            "unassessed"
        } else if inside {
            "pass"
        } else {
            "fail"
        };
        if gating && status != "pass" {
            pass = false;
        }
        checks[key] = json!({"status":status,"target":target,"actual":actual,"reason":if !measured {"measurement missing, ambiguous or estimated"}else if !inside {"outside target or target unavailable"}else{"within range"}});
    }
    if gates == 0 {
        return Err("profile has no gating metrics".into());
    }
    Ok((pass, checks))
}

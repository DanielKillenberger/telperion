use serde_json::{json, Value};
use std::collections::BTreeSet;
use telperion_core::{
    foliage::{transform_point, Element, FoliageUnit, Instances},
    tree::Tree,
};

pub fn metric(
    value: Value,
    status: &str,
    definition: &str,
    unit: &str,
    included: usize,
    excluded: usize,
    reason: Option<&str>,
) -> Value {
    json!({"status":status,"value":value,"unit":unit,"definition":definition,"evidence_ids":[],"reason":reason,"support":{"included":included,"excluded":excluded,"flags":[]}})
}
pub(crate) fn distribution(mut values: Vec<f64>) -> Value {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n == 0 {
        return json!({"count":0,"min":null,"max":null,"median":null,"p10":null,"p90":null});
    }
    json!({"count":n,"min":values[0],"max":values[n-1],"median":values[(n-1)/2]/2.+values[n/2]/2.,"p10":values[(n as f64*0.1).ceil() as usize-1],"p90":values[(n as f64*0.9).ceil() as usize-1]})
}
#[derive(Default)]
struct Axis {
    order: usize,
    edges: Vec<usize>,
    ambiguous: bool,
}

pub fn axes(tree: &Tree) -> Result<Value, String> {
    tree.validate_solved()
        .map_err(|e| format!("invalid-geometry: {e:?}"))?;
    if tree.nodes.is_empty() {
        return Ok(metric(
            Value::Null,
            "unavailable",
            "axis-order-v1",
            "axes",
            0,
            0,
            Some("empty-tree"),
        ));
    }
    let mut children = vec![Vec::new(); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        children[n.parent.unwrap() as usize].push(i);
    }
    let mut axis_id = vec![0; tree.nodes.len()];
    let mut inherited = vec![false; tree.nodes.len()];
    let mut runs = vec![Axis::default()];
    for (p, kids) in children.iter().enumerate() {
        let dominant = kids.iter().copied().max_by(|a, b| {
            tree.nodes[*a]
                .start_radius
                .total_cmp(&tree.nodes[*b].start_radius)
                .then_with(|| b.cmp(a))
        });
        let tied = dominant.is_some_and(|d| {
            kids.iter()
                .filter(|&&i| tree.nodes[i].start_radius == tree.nodes[d].start_radius)
                .count()
                > 1
        });
        for &i in kids {
            inherited[i] = inherited[p] || tied;
            axis_id[i] = if Some(i) == dominant {
                axis_id[p]
            } else {
                runs.push(Axis {
                    order: runs[axis_id[p]].order + 1,
                    ..Axis::default()
                });
                runs.len() - 1
            };
            let a = &mut runs[axis_id[i]];
            a.ambiguous |= inherited[i];
            a.edges.push(i);
        }
    }
    let mut samples = Vec::new();
    for (id, a) in runs.iter().enumerate() {
        let lengths: Vec<_> = a
            .edges
            .iter()
            .map(|&i| {
                (tree.nodes[i].position
                    - tree.nodes[tree.nodes[i].parent.unwrap() as usize].position)
                    .length()
            })
            .collect();
        let length: f64 = lengths.iter().sum();
        if !length.is_finite() {
            return Err("nonfinite-geometry: axis length".into());
        }
        let mut flags = Vec::new();
        if a.ambiguous {
            flags.push("ambiguous-dominant");
        }
        if length <= 1e-9 || lengths.iter().any(|&l| l <= 1e-9) {
            flags.push("degenerate-axis");
        }
        let qualified = flags.is_empty();
        let first = a.edges.first().copied();
        let origin_diameter = first.map(|i| 2. * tree.nodes[i].start_radius);
        let mean = if length > 1e-9 {
            Some(
                a.edges
                    .iter()
                    .zip(&lengths)
                    .map(|(&i, l)| {
                        (tree.nodes[i].start_radius + tree.nodes[i].radius) * (l / length)
                    })
                    .sum::<f64>(),
            )
        } else {
            None
        };
        let diameter_at = |fraction: f64| -> Option<f64> {
            if !qualified {
                return None;
            }
            let target = fraction * length;
            let mut arc = 0.;
            for (&i, &l) in a.edges.iter().zip(&lengths) {
                if target < arc + l {
                    let n = &tree.nodes[i];
                    return Some(
                        2. * (n.start_radius + (n.radius - n.start_radius) * ((target - arc) / l)),
                    );
                }
                arc += l;
            }
            a.edges.last().map(|&i| 2. * tree.nodes[i].radius)
        };
        let proximal = diameter_at(0.1);
        let distal = diameter_at(0.9);
        if let (Some(p), Some(d)) = (proximal, distal) {
            if p > 0. && (!(d / p).is_finite() || !((p - d) / (0.8 * length)).is_finite()) {
                return Err("nonfinite-geometry: taper overflow".into());
            }
        }
        let taper = match (proximal, distal) {
            (Some(p), Some(d)) if p > 0. => {
                json!({"proximal_m":p,"distal_m":d,"ratio":d/p,"slope_m_per_m":(p-d)/(0.8*length)})
            }
            _ => Value::Null,
        };
        let angle = first.and_then(|i| {
            if id == 0 || !qualified {
                return None;
            }
            let p = tree.nodes[i].parent.unwrap() as usize;
            let gp = tree.nodes[p].parent? as usize;
            let incoming = tree.nodes[p].position - tree.nodes[gp].position;
            let outgoing = tree.nodes[i].position - tree.nodes[p].position;
            if incoming.length() <= 1e-9 || outgoing.length() <= 1e-9 {
                None
            } else {
                Some(
                    incoming
                        .normalized()
                        .dot(outgoing.normalized())
                        .clamp(-1., 1.)
                        .acos()
                        .to_degrees(),
                )
            }
        });
        if mean.is_some_and(|v| !v.is_finite()) || origin_diameter.is_some_and(|v| !v.is_finite()) {
            return Err("nonfinite-geometry: diameter".into());
        }
        samples.push(json!({"axis_id":id,"order":a.order,"edge_nodes":a.edges,"qualified":qualified,"excluded_reasons":flags,"length_m":length,"origin_diameter_m":origin_diameter,"mean_diameter_m":mean,"taper":taper,"angle_deg":angle,"angle_reason":if angle.is_none(){Some("ambiguous, degenerate or missing incoming tangent")}else{None}}));
    }
    let orders: BTreeSet<_> = runs.iter().map(|a| a.order).collect();
    let by_order: Vec<_>=orders.into_iter().map(|order| {
        let raw:Vec<_>=samples.iter().filter(|s|s["order"]==order).collect();
        let qualified:Vec<_>=raw.iter().filter(|s|s["qualified"]==true).collect();
        let values=|key:&str| qualified.iter().filter_map(|s|s[key].as_f64()).collect();
        json!({"order":order,"raw_count":raw.len(),"excluded_count":raw.len()-qualified.len(),"length_m":distribution(values("length_m")),"origin_diameter_m":distribution(values("origin_diameter_m")),"mean_diameter_m":distribution(values("mean_diameter_m")),"angle_deg":distribution(values("angle_deg"))})
    }).collect();
    let included = samples.iter().filter(|s| s["qualified"] == true).count();
    Ok(metric(
        json!({"samples":samples,"by_order":by_order,"raw_axis_count":runs.len(),"definitions":["axis-order-v1","axis-length-v1","axis-diameter-v1","axis-taper-v1","axis-angle-v1"]}),
        "estimated",
        "axis-order-v1",
        "axes",
        included,
        runs.len() - included,
        None,
    ))
}

pub fn foliage_bins(tree: &Tree, element: &Element, kept: &Instances) -> Result<Value, String> {
    tree.validate()
        .map_err(|e| format!("invalid-geometry: {e:?}"))?;
    element
        .validate()
        .map_err(|e| format!("invalid-geometry: {e:?}"))?;
    kept.validate()
        .map_err(|e| format!("invalid-geometry: {e:?}"))?;
    let unavailable = |reason| {
        metric(
            Value::Null,
            "unavailable",
            "foliage-bins-v1",
            "individual biological units",
            0,
            kept.matrices.len(),
            Some(reason),
        )
    };
    let Some(anatomy) = &element.anatomy else {
        return Ok(unavailable("unknown biological subset"));
    };
    let Some(root) = tree.nodes.first() else {
        return Ok(unavailable("empty-tree"));
    };
    if kept.matrices.is_empty() {
        return Ok(unavailable("empty-foliage"));
    }
    let mut centroids = Vec::new();
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    let mut radius: f64 = 0.;
    for matrix in &kept.matrices {
        let mut centroid = telperion_core::math::Vec3::ZERO;
        for vertex in &element.positions[anatomy.vertices.clone()] {
            let p = transform_point(matrix, *vertex);
            if !p.is_finite() {
                return Err("nonfinite-geometry: biological vertex".into());
            }
            ymin = ymin.min(p.y);
            ymax = ymax.max(p.y);
            centroid += p / (anatomy.vertices.len() as f64);
        }
        let r = (centroid.x - root.position.x).hypot(centroid.z - root.position.z);
        radius = radius.max(r);
        centroids.push((centroid.y, r));
    }
    if !(ymax - ymin).is_finite() || !radius.is_finite() {
        return Err("nonfinite-geometry: crown bounds".into());
    }
    if ymax - ymin <= 0. || radius <= 0. {
        return Ok(unavailable("zero crown height or radial span"));
    }
    let mut counts = [[0usize; 4]; 4];
    let mut height = [0usize; 4];
    let mut radial = [0usize; 4];
    let mut underflow = 0;
    let mut overflow = 0;
    for (y, r) in centroids {
        let h = (y - ymin) / (ymax - ymin);
        let r = r / radius;
        if h < 0. || r < 0. {
            underflow += 1;
            continue;
        }
        if h > 1. || r > 1. {
            overflow += 1;
            continue;
        }
        let hi = ((h * 4.).floor() as usize).min(3);
        let ri = ((r * 4.).floor() as usize).min(3);
        counts[hi][ri] += 1;
        height[hi] += 1;
        radial[ri] += 1;
    }
    let total: usize = height.iter().sum();
    Ok(metric(
        json!({"counts":counts,"height_marginal":height,"radial_marginal":radial,"total":total,"underflow":underflow,"overflow":overflow,"occupied_bins":counts.iter().flatten().filter(|&&n|n>0).count(),"normalization":{"ymin_m":ymin,"ymax_m":ymax,"max_centroid_radius_m":radius,"root_x_m":root.position.x,"root_z_m":root.position.z},"unit":if anatomy.unit==FoliageUnit::Needle{"needle"}else{"leaf"}}),
        "measured",
        "foliage-bins-v1",
        "individual biological units",
        total,
        underflow + overflow,
        None,
    ))
}

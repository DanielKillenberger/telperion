//! The corner map of species space (fn-272 R1b). Each shipped species is a
//! point: every catalogue row a walk moves, on the scale the walk moves it
//! (so a walk is a straight segment here), each row less its mean and over
//! its spread across the species, the age left out. The map is the
//! projection onto the first two principal components of those points.
use telperion_core::{blend, catalogue, presets::Family};
use telperion_space::{
    scale, settings, zone_settings, Scale, Species, Transcendental, OUTCOMES, ZONES,
};

/// One species' rows: a path and its value on the walk's own scale.
type Rows = Vec<(String, f64)>;

/// The two axes, the species' dots on them and what the axes say.
pub struct Map {
    paths: Vec<String>,
    mean: Vec<f64>,
    spread: Vec<f64>,
    axes: [Vec<f64>; 2],
    /// The shipped species by id, each at its point.
    pub dots: Vec<(String, [f64; 2])>,
    /// The share of the species' variance the two axes hold.
    pub share: f64,
    /// Each axis by its largest-loading rows, the age index left off.
    pub labels: [Vec<String>; 2],
}

impl Map {
    /// The map of `species`, each an id and its family; at least three.
    pub fn of(species: &[(&str, Family)]) -> Result<Map, String> {
        if species.len() < 3 {
            return Err("a map of species space wants three species or more".into());
        }
        let rows: Vec<Rows> = species
            .iter()
            .map(|(_, f)| rows(f))
            .collect::<Result<_, _>>()?;
        // A row every species states, and on which they differ.
        let n = rows.len() as f64;
        let (mut paths, mut mean, mut spread) = (Vec::new(), Vec::new(), Vec::new());
        for (path, _) in &rows[0] {
            let values: Option<Vec<f64>> = rows.iter().map(|r| value(r, path)).collect();
            let Some(values) = values else { continue };
            let m = values.iter().sum::<f64>() / n;
            let s = (values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / n).sqrt();
            if s > 1e-12 * m.abs().max(1.0) {
                (paths.push(path.clone()), mean.push(m), spread.push(s));
            }
        }
        let z: Vec<Vec<f64>> = rows
            .iter()
            .map(|r| standard(r, &paths, &mean, &spread))
            .collect::<Result<_, _>>()?;
        // The principal axes from the species' Gram matrix: as many species
        // as there are, never as many rows.
        let gram: Vec<Vec<f64>> = z
            .iter()
            .map(|a| z.iter().map(|b| dot(a, b)).collect())
            .collect();
        let (values, vectors) = eigen(gram);
        let total: f64 = values.iter().map(|v| v.max(0.0)).sum();
        let axis = |k: usize| -> Vec<f64> {
            let scale = 1.0 / values[k].max(1e-300).sqrt();
            let mut a: Vec<f64> = (0..paths.len())
                .map(|r| {
                    scale
                        * z.iter()
                            .zip(&vectors[k])
                            .map(|(p, u)| p[r] * u)
                            .sum::<f64>()
                })
                .collect();
            // The sign that makes the largest loading positive.
            let big = a
                .iter()
                .copied()
                .fold(0.0f64, |m, x| if x.abs() > m.abs() { x } else { m });
            if big < 0.0 {
                a.iter_mut().for_each(|x| *x = -*x);
            }
            a
        };
        let axes = [axis(0), axis(1)];
        let labels = [label(&paths, &axes[0]), label(&paths, &axes[1])];
        let mut map = Map {
            paths,
            mean,
            spread,
            axes,
            dots: Vec::new(),
            share: (values[0] + values[1]) / total,
            labels,
        };
        map.dots = species
            .iter()
            .map(|(id, f)| Ok((id.to_string(), map.project(f)?)))
            .collect::<Result<_, String>>()?;
        Ok(map)
    }

    /// Where `family` stands on the map.
    pub fn project(&self, family: &Family) -> Result<[f64; 2], String> {
        let z = standard(&rows(family)?, &self.paths, &self.mean, &self.spread)?;
        Ok([dot(&z, &self.axes[0]), dot(&z, &self.axes[1])])
    }

    /// The dot of the species `id`.
    pub fn dot(&self, id: &str) -> Result<[f64; 2], String> {
        self.dots
            .iter()
            .find(|(d, _)| d == id)
            .map(|(_, p)| *p)
            .ok_or(format!("{id} is no species on the map"))
    }
}

/// The family `t` of the way along the walk from `a` to `b` as the walk
/// mixes its rows: `blend::families`, with the species before the size
/// match, which shrinks a frame's laterals to its ends' size and is not a
/// place in species space.
pub fn walked(a: &Family, b: &Family, t: f64) -> Result<Family, String> {
    let mut f = blend::families(a, b, t).map_err(|e| format!("walk at {t}: {e}"))?;
    if let (Some(x), Some(y)) = (&a.space.species, &b.space.species) {
        f.space.species =
            Some(telperion_space::blend(x, y, t).map_err(|e| format!("walk at {t}: {e}"))?);
    }
    Ok(f)
}

fn value(rows: &Rows, path: &str) -> Option<f64> {
    rows.iter().find(|(p, _)| p == path).map(|(_, v)| *v)
}

fn standard(
    rows: &Rows,
    paths: &[String],
    mean: &[f64],
    spread: &[f64],
) -> Result<Vec<f64>, String> {
    let lookup: std::collections::HashMap<&str, f64> =
        rows.iter().map(|(p, v)| (p.as_str(), *v)).collect();
    paths
        .iter()
        .zip(mean.iter().zip(spread))
        .map(|(p, (m, s))| {
            lookup
                .get(p.as_str())
                .map(|v| (v - m) / s)
                .ok_or(format!("the family states no {p}"))
        })
        .collect()
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// Every row a walk moves, on its walk's scale: the catalogue's rows that
/// walk on their own (the age left out) and the engine species' settings.
fn rows(family: &Family) -> Result<Rows, String> {
    use catalogue::Blend::{Count, Degrees, Down, Linear, Many, Up, Weighted};
    let mut out: Rows = catalogue::entries()
        .filter(|e| {
            e.path() != "/age"
                && matches!(
                    e.blend(),
                    Linear | Weighted | Degrees | Count | Many | Down | Up
                )
        })
        .filter_map(|e| e.get(family).number().map(|v| (e.path().to_string(), v)))
        .collect();
    let species = family
        .space
        .species
        .as_ref()
        .ok_or("a map point carries an engine species")?;
    species_rows(species, &mut out)?;
    Ok(out)
}

/// The species' settings, age by age and zone by zone, each on the scale
/// `telperion_space::blend` mixes it on, so the walk is straight in them:
/// a magnitude by its shifted log, a delay by the share it keeps, a relay
/// by the share of its stops that relay.
fn species_rows(species: &Species, out: &mut Rows) -> Result<(), String> {
    for (k, state) in species.states.iter().enumerate() {
        let mut copy = state.clone();
        let named: Vec<(&str, f64)> = settings(&mut copy)
            .into_iter()
            .map(|(n, v)| (n, *v))
            .collect();
        let of = |name: &str| {
            named
                .iter()
                .find(|(n, _)| *n == name)
                .map_or(0.0, |(_, v)| *v)
        };
        for &(name, v) in &named {
            let x = match scale(name).ok_or(format!("{name} has no scale"))? {
                Scale::Linear => v,
                Scale::Log { floor } => (v + floor).ln_fixed(),
                Scale::Kept => 1.0 / (1.0 + v),
                Scale::Outcome => match OUTCOMES.iter().find(|(_, relay)| *relay == name) {
                    Some(&(stop, _)) => {
                        v * if stop == "abortion" {
                            of(stop)
                        } else {
                            1.0 - of(stop)
                        }
                    }
                    None => v,
                },
            };
            out.push((format!("/space/ages/{k}/{name}"), x));
        }
        let width = state.zones[0].lateral.len();
        for r in 0..ZONES {
            let mut zone = state.zones.get(r).cloned().unwrap_or_else(|| empty(width));
            let named = zone_settings(&mut zone).ok_or("a map point holds uniform node counts")?;
            let mut seen: Vec<&str> = Vec::new();
            for (name, v) in named {
                let j = seen.iter().filter(|n| **n == name).count();
                seen.push(name);
                let x = match scale(name).ok_or(format!("{name} has no scale"))? {
                    Scale::Log { floor } => (*v + floor).ln_fixed(),
                    _ => *v,
                };
                out.push((format!("/space/ages/{k}/zones/{r}/{name}/{j}"), x));
            }
        }
    }
    Ok(())
}

/// A role a growth unit leaves out, as the walk stands it in: no nodes.
fn empty(width: usize) -> telperion_space::Zone {
    telperion_space::Zone {
        nodes: telperion_space::NodeLaw::Uniform { min: 0.0, max: 0.0 },
        buds: 1.0,
        lateral: vec![0.0; width],
        dormant: vec![0.0; width],
        delay: 0.0,
        rate: 0.0,
    }
}

/// An axis by its two largest rows, each row's loadings summed in square
/// over the ages and zones it repeats at.
fn label(paths: &[String], axis: &[f64]) -> Vec<String> {
    let mut by: Vec<(String, f64)> = Vec::new();
    for (p, l) in paths.iter().zip(axis) {
        let name = name(p);
        match by.iter_mut().find(|(n, _)| *n == name) {
            Some((_, s)) => *s += l * l,
            None => by.push((name, l * l)),
        }
    }
    by.sort_by(|a, b| b.1.total_cmp(&a.1));
    by.into_iter().take(2).map(|(n, _)| n).collect()
}

/// A row's name without its age, zone or entry index.
fn name(path: &str) -> String {
    let words: Vec<&str> = path
        .split('/')
        .filter(|w| !w.is_empty() && w.parse::<usize>().is_err())
        .collect();
    match words.as_slice() {
        ["space", "ages", "zones", rest @ ..] | ["space", "ages", rest @ ..] => rest.join("/"),
        _ => words.last().map_or(String::new(), |w| w.to_string()),
    }
}

/// The eigenvalues of a symmetric matrix, largest first, each with its
/// unit eigenvector, by Jacobi rotations.
fn eigen(mut a: Vec<Vec<f64>>) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut v: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| f64::from(u8::from(i == j))).collect())
        .collect();
    for _ in 0..100 {
        let off: f64 = (0..n)
            .flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j].powi(2))
            .sum();
        if off < 1e-30 {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                if a[p][q].abs() < 1e-300 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let (c, s) = (1.0 / (t * t + 1.0).sqrt(), t / (t * t + 1.0).sqrt());
                for k in 0..n {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    (a[k][p], a[k][q]) = (c * akp - s * akq, s * akp + c * akq);
                }
                for k in 0..n {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    (a[p][k], a[q][k]) = (c * apk - s * aqk, s * apk + c * aqk);
                }
                for row in v.iter_mut() {
                    let (vp, vq) = (row[p], row[q]);
                    (row[p], row[q]) = (c * vp - s * vq, s * vp + c * vq);
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| a[j][j].total_cmp(&a[i][i]));
    let values = order.iter().map(|&i| a[i][i]).collect();
    let vectors = order
        .iter()
        .map(|&i| (0..n).map(|k| v[k][i]).collect())
        .collect();
    (values, vectors)
}

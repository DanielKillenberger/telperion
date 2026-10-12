//! A preset's engine species as indexed rows (fn-198 host decision 1): the
//! tree-space engine's settings at their place on its reference axis and
//! the ages that are stem, each one row of the preset's own value file
//! under `/space`. Every preset grows under the one site light (`SITE`),
//! which is no preset's row (fn-264 R1a). The engine judges what it grows
//! (`Species::validate`); these rows are judged for their place and form.
//!
//! - `/space/ages/<k>/<setting>`, `/space/ages/<k>/form/<setting>`: a
//!   setting of age `k`, named as the engine's `settings` names it, in the
//!   wire's camel case.
//! - `/space/ages/<k>/zones/<z>/<setting>`, `.../nodes/min`, `.../nodes/max`,
//!   `.../lateral/<j>`, `.../dormant/<j>`: a zone of age `k` by its role `z`;
//!   a zone is declared by its `buds` row.
//! - `/space/ages/<k>/trunk = true`: an axis of age `k` carried on from the
//!   trunk is stem.
//!
//! A row states a setting of an age the species lives in (a lifespan above
//! 0) that differs from an age passed through (`telperion_space::passed`);
//! every other age is derived as the engine derives it (`canonical`). An
//! unbounded span is the token `never`, admitted on `shedding` and
//! `leafYears` alone.
//! This file is shared with the build script, which judges every shipped
//! preset's rows as the crate compiles.
use telperion_space::{passed, settings, zone_settings, Light, PaState, Species, Zone, AGES};
use telperion_space::{NodeLaw, ZONES};

/// Where every row of the group sits.
pub const PREFIX: &str = "/space/";

/// A row's value as written: a number, a switch, or `never`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Number(f64),
    Switch(bool),
    Never,
}

impl Cell {
    /// A value file's row as written.
    pub fn of(line: &super::grammar::Line) -> Self {
        use super::grammar::Form;
        match line.form {
            Form::Whole | Form::Real => Self::Number(line.number()),
            Form::Switch => Self::Switch(line.number() != 0.0),
            Form::Never => Self::Never,
        }
    }
    /// The cell as the grammar writes it.
    pub fn text(self) -> String {
        match self {
            Self::Number(v) => format!("{v:?}"),
            Self::Switch(b) => b.to_string(),
            Self::Never => "never".into(),
        }
    }
}

/// The site's light every preset grows under (fn-264 R1a, host 2026-10-11):
/// environment, not species. Beer-Lambert's k for an individual crown,
/// leaf area density times the path through it, is 0.5 for leaves at every
/// angle: "k equals 0.5 if leaf-angle distribution is assumed to be
/// spherical (Ross, 1981)" (Bréda 2003, J. Exp. Bot. 54: 2403, on
/// individual tree crowns). The sky is the standard overcast one.
pub const SITE: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// A preset's engine species, the ages that are stem, and the site's light
/// it grows under (`SITE`). A preset with no `/space` setting row has no
/// species, and the pipeline refuses it by name.
#[derive(Debug, Clone, PartialEq)]
pub struct Space {
    pub species: Option<Species>,
    /// The trunk-level ages, ascending: only an axis of one of them, carried
    /// on from the trunk, is stem.
    pub trunk: Vec<usize>,
    pub light: Light,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            species: None,
            trunk: Vec::new(),
            light: SITE,
        }
    }
}

/// Whether `path` is a row of this group.
pub fn holds(path: &str) -> bool {
    path.starts_with(PREFIX)
}

/// The space `rows` state, or the first row refused, as `<path> <why>`.
pub fn read<'a>(rows: impl IntoIterator<Item = (&'a str, Cell)>) -> Result<Space, String> {
    let mut states: Vec<PaState> = (0..AGES).map(|_| passed()).collect();
    let mut stated = [false; AGES];
    let mut space = Space::default();
    for (path, cell) in rows {
        let words: Vec<&str> = path
            .strip_prefix(PREFIX)
            .ok_or(format!("{path} is not a space row"))?
            .split('/')
            .collect();
        match words[..] {
            ["light", ..] => {
                return Err(format!(
                    "{path}: the site's light is one setting, no preset's row"
                ));
            }
            ["ages", k, ref tail @ ..] => {
                let k = index(path, k, AGES)?;
                if tail == ["trunk"] {
                    let Cell::Switch(stem) = cell else {
                        return Err(format!("{path} holds true or false"));
                    };
                    if stem && !space.trunk.contains(&k) {
                        space.trunk.push(k);
                    }
                    continue;
                }
                stated[k] = true;
                put(&mut states[k], path, tail, cell)?;
            }
            _ => return Err(format!("{path} is not a space row")),
        }
    }
    space.trunk.sort_unstable();
    if let Some(k) = (0..AGES).find(|&k| stated[k] && states[k].lifespan <= 0.0) {
        return Err(format!("/space/ages/{k} states an age with no lifespan"));
    }
    if stated.contains(&true) {
        space.species = Some(Species { states }.canonical());
    }
    Ok(space)
}

/// Sets the setting at `tail` of one age.
fn put(state: &mut PaState, path: &str, tail: &[&str], cell: Cell) -> Result<(), String> {
    if let ["zones", z, ref rest @ ..] = tail[..] {
        let z = index(path, z, ZONES)?;
        while state.zones.len() <= z {
            state.zones.push(passed().zones[0].clone());
        }
        return zone(&mut state.zones[z], path, rest, cell);
    }
    let wire = tail.join("/");
    for (name, slot) in settings(state) {
        if camel(name) == wire {
            *slot = match (cell, name) {
                (Cell::Never, "shedding" | "leaf_years") => f64::INFINITY,
                _ => number(path, cell)?,
            };
            return Ok(());
        }
    }
    Err(format!("{path} is not a space row"))
}

fn zone(zone: &mut Zone, path: &str, rest: &[&str], cell: Cell) -> Result<(), String> {
    let v = number(path, cell)?;
    if let [table @ ("lateral" | "dormant"), j] = rest[..] {
        let j = index(path, j, AGES)?;
        match table {
            "lateral" => zone.lateral[j] = v,
            _ => zone.dormant[j] = v,
        }
        return Ok(());
    }
    let wire = rest.join("/");
    let named = zone_settings(zone).ok_or(format!("{path}: a zone's node law is uniform"))?;
    for (name, slot) in named {
        if let Some(n) = name.strip_prefix("zone.") {
            if !matches!(n, "lateral" | "dormant") && camel(n) == wire {
                *slot = v;
                return Ok(());
            }
        }
    }
    Err(format!("{path} is not a space row"))
}

/// The rows that state `space`, in the order a value file lists them, or
/// why it cannot be written as rows.
pub fn rows(space: &Space) -> Result<Vec<(String, Cell)>, String> {
    let mut out = Vec::new();
    let mut base = passed();
    let defaults: Vec<f64> = settings(&mut base).into_iter().map(|(_, v)| *v).collect();
    let bare = passed().zones[0].clone();
    if let Some(species) = &space.species {
        if species.states.len() != AGES {
            return Err(format!("a species holds the {AGES} ages of the chain"));
        }
        for (k, state) in species.states.iter().enumerate() {
            if state.lifespan <= 0.0 {
                continue;
            }
            let mut state = state.clone();
            for ((name, v), d) in settings(&mut state).into_iter().zip(&defaults) {
                if v.to_bits() != d.to_bits() {
                    let cell = if v.is_infinite() {
                        Cell::Never
                    } else {
                        Cell::Number(*v)
                    };
                    out.push((format!("{PREFIX}ages/{k}/{}", camel(name)), cell));
                }
            }
            for (z, zone) in state.zones.iter().enumerate() {
                zone_rows(
                    &mut out,
                    &format!("{PREFIX}ages/{k}/zones/{z}"),
                    zone,
                    &bare,
                )?;
            }
        }
    }
    for k in &space.trunk {
        out.push((format!("{PREFIX}ages/{k}/trunk"), Cell::Switch(true)));
    }
    if space.light != SITE {
        return Err("a preset grows under the site's light, which no row states".into());
    }
    Ok(out)
}

fn zone_rows(
    out: &mut Vec<(String, Cell)>,
    at: &str,
    zone: &Zone,
    bare: &Zone,
) -> Result<(), String> {
    let NodeLaw::Uniform { min, max } = zone.nodes else {
        return Err(format!("{at}: a zone's node law is uniform"));
    };
    if zone.lateral.len() != AGES || zone.dormant.len() != AGES {
        return Err(format!(
            "{at}: a zone's tables hold the {AGES} ages of the chain"
        ));
    }
    let NodeLaw::Uniform { min: m0, max: x0 } = bare.nodes else {
        unreachable!("a passed age's zone is uniform");
    };
    // A zone is declared by its buds, so a zone of the defaults is kept.
    out.push((format!("{at}/buds"), Cell::Number(zone.buds)));
    for (name, v, d) in [
        ("nodes/min", min, m0),
        ("nodes/max", max, x0),
        ("delay", zone.delay, bare.delay),
        ("rate", zone.rate, bare.rate),
    ] {
        if v.to_bits() != d.to_bits() {
            out.push((format!("{at}/{name}"), Cell::Number(v)));
        }
    }
    for (table, values) in [("lateral", &zone.lateral), ("dormant", &zone.dormant)] {
        for (j, &v) in values.iter().enumerate() {
            if v.to_bits() != 0f64.to_bits() {
                out.push((format!("{at}/{table}/{j}"), Cell::Number(v)));
            }
        }
    }
    Ok(())
}

fn number(path: &str, cell: Cell) -> Result<f64, String> {
    match cell {
        Cell::Number(v) if v.is_finite() => Ok(v),
        Cell::Never => Err(format!(
            "{path} is never only as a shedding delay or a leaf's years"
        )),
        _ => Err(format!("{path} holds a number")),
    }
}

fn index(path: &str, word: &str, count: usize) -> Result<usize, String> {
    match word.parse::<usize>() {
        Ok(i) if i < count && word == i.to_string() => Ok(i),
        _ => Err(format!("{path}: {word} is not an index below {count}")),
    }
}

/// An engine setting's name on the wire: `form.bend_length` as
/// `form/bendLength`, `nodes.min` as `nodes/min`.
pub fn camel(name: &str) -> String {
    name.split('.')
        .map(|part| {
            let mut words = part.split('_');
            let mut out = words.next().unwrap_or_default().to_string();
            for w in words {
                let mut c = w.chars();
                if let Some(first) = c.next() {
                    out.push(first.to_ascii_uppercase());
                    out.extend(c);
                }
            }
            out
        })
        .collect::<Vec<_>>()
        .join("/")
}

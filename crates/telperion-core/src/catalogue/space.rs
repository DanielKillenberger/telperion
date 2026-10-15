//! The engine's settings in the browser's table (fn-284 R2): each one's
//! place under an age's or a zone's rows, its scale, bounds, unit and
//! meaning as the engine's table states them (`telperion_space::SETTINGS`),
//! its value at an age that states none, and its group.
use super::browser::{number, text};
use super::group::{self, Group};
use crate::presets::space::camel;
use std::fmt::Write;
use telperion_space::{passed, settings, zone_settings, Scale, SETTINGS};

pub(super) const INTERFACE: &str =
    "/** One engine setting, as a control over every age a species states. */
export interface SpaceSetting {
  /** The engine's name for it, `form.bend_length`. */
  name: string;
  /** Its place under an age's rows, or under a zone's where `zone`: `form/bendLength`. */
  key: string;
  zone: boolean;
  label: string;
  /** How a control moves it: `log` multiplies (above `floor`), `linear` offsets,
   *  `kept` offsets the share 1 / (1 + value) it keeps, `outcome` offsets a share. */
  scale: \"linear\" | \"log\" | \"kept\" | \"outcome\";
  floor: number;
  low: number;
  high: number;
  lowOpen: boolean;
  /** Admits `never`, an unbounded span, on the wire. */
  never: boolean;
  /** Its value at an age or zone that states none. */
  neutral: number;
  unit: string;
  meaning: string;
  group: Group;
}

";

/// The `SPACE_SETTINGS` table: every setting but the bud tables, which are
/// one probability per age rather than one value.
pub(super) fn table(out: &mut String) {
    let mut age = passed();
    let mut zone = age.zones[0].clone();
    let mut neutral: Vec<(&str, f64)> = settings(&mut age)
        .into_iter()
        .map(|(n, v)| (n, *v))
        .collect();
    let zoned = zone_settings(&mut zone).expect("a passed age's zone is uniform");
    neutral.extend(zoned.into_iter().map(|(n, v)| (n, *v)));
    out.push_str("export const SPACE_SETTINGS: readonly SpaceSetting[] = [\n");
    for s in SETTINGS
        .iter()
        .filter(|s| !matches!(s.name, "zone.lateral" | "zone.dormant"))
    {
        let value = neutral.iter().find(|(n, _)| *n == s.name).map(|&(_, v)| v);
        row(out, s, value.expect("every setting has a neutral"));
    }
    out.push_str("];\n");
}

fn row(out: &mut String, s: &telperion_space::Setting, neutral: f64) {
    let (zone, bare) = match s.name.strip_prefix("zone.") {
        Some(rest) => (true, rest),
        None => (false, s.name),
    };
    let (scale, floor) = match s.scale {
        Scale::Linear => ("linear", 0.0),
        Scale::Log { floor } => ("log", floor),
        Scale::Kept => ("kept", 0.0),
        Scale::Outcome => ("outcome", 0.0),
    };
    let label = bare.rsplit('.').next().unwrap_or(bare).replace('_', " ");
    let group = group::of_setting(s.name).unwrap_or_else(|| panic!("{} has no group", s.name));
    let _ = writeln!(
        out,
        "  {{ name: {}, key: {}, zone: {zone}, label: {}, scale: \"{scale}\", floor: {}, \
         low: {}, high: {}, lowOpen: {}, never: {}, neutral: {}, unit: {}, meaning: {}, \
         group: \"{}\" }},",
        text(s.name),
        text(&camel(bare)),
        text(&label),
        number(floor),
        number(s.low),
        number(s.high),
        s.low_open,
        matches!(s.name, "shedding" | "leaf_years"),
        number(neutral),
        text(s.unit),
        text(s.meaning),
        id(group),
    );
}

/// A group's id in the browser's table.
pub(super) fn id(group: Group) -> String {
    format!("{group:?}").to_lowercase()
}

/// The `Group` type and the `GROUPS` table, in the panel's order.
pub(super) fn groups(out: &mut String) {
    let ids: Vec<String> = Group::ALL
        .iter()
        .map(|&g| format!("\"{}\"", id(g)))
        .collect();
    let _ = writeln!(
        out,
        "/** What a control does to the tree; the panel's groups. */"
    );
    let _ = writeln!(out, "export type Group = {};\n", ids.join(" | "));
    out.push_str("export const GROUPS: readonly { id: Group; title: string }[] = [\n");
    for g in Group::ALL {
        let _ = writeln!(
            out,
            "  {{ id: \"{}\", title: {} }},",
            id(g),
            text(g.title())
        );
    }
    out.push_str("];\n\n");
}

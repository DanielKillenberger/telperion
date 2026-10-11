//! A captured layout as the device's columns (`layout.wgsl`): each table
//! one word a row, column by column, as `capture_layout.rs` writes it.
use super::columns::Cap;

/// An axis's, a phytomer's, a unit's and a pencil's columns, a PA's form
/// row, and the tally's words at the head of the lists.
pub(super) const AXIS: usize = 31;
pub(super) const PHYT: usize = 19;
pub(super) const UNIT: usize = 4;
pub(super) const PENCIL: usize = 59;
pub(super) const TALLY: usize = 16;

/// The columns' names, for the agreement's report.
pub(super) const AXIS_NAMES: [&str; AXIS] = [
    "pa", "birth", "origin", "parent", "node", "whorl", "lineage lo", "lineage hi", "blend",
    "phase", "rank", "base x", "base y", "base z", "heading x", "heading y", "heading z",
    "side x", "side y", "side z", "b0", "b1", "sleep", "units", "unit first", "units from",
    "old first", "old count", "new first", "new count", "fault",
];
pub(super) const PHYT_NAMES: [&str; PHYT] = [
    "key lo", "key hi", "cycle", "node", "tip x", "tip y", "tip z", "heading x", "heading y",
    "heading z", "side x", "side y", "side z", "scale", "grown", "size", "rank", "leaf area",
    "laid",
];
pub(super) const PENCIL_NAMES: [&str; 16] = [
    "running x", "running y", "running z", "across x", "across y", "across z", "tip x", "tip y",
    "tip z", "run", "pull", "curve x", "curve y", "curve z", "turning", "pulls",
];
pub(super) const PENCIL_TAIL: [&str; 11] = [
    "base scale", "running", "rank", "units", "laid", "trunk", "wood", "light", "size", "memory",
    "kept",
];

/// A pencil column's name: its walk's, a pull's field, or a scalar's.
pub(super) fn pencil_name(k: usize) -> String {
    match k {
        0..16 => PENCIL_NAMES[k].to_string(),
        16..48 => format!("pull {} {}", (k - 16) / 4, ["bend", "pull", "total", "offset"][(k - 16) % 4]),
        _ => PENCIL_TAIL[k - 48].to_string(),
    }
}

/// The cycle before its layout: its counts, tables and columns.
pub(super) struct Laid {
    pub cycle: u32,
    pub first_new: u32,
    pub axes: u32,
    pub phyts: u32,
    pub units: u32,
    pub form: Vec<f32>,
    pub grew: Vec<u32>,
    pub generations: Vec<u32>,
    pub in_axes: Vec<u32>,
    pub in_units: Vec<u32>,
    pub in_phyts: Vec<u32>,
    pub in_pencils: Vec<u32>,
}

impl Laid {
    pub(super) fn new(c: &Cap) -> Self {
        let [cycle, _age, first_new, axes, phyts] = c.u32s("lay_counts")[..] else {
            panic!("cycle, age, first new, axes, phytomers")
        };
        let in_units = c.u32s("in_units");
        let mut form = c.f32s("lay_globals");
        form.extend(c.f32s("lay_form"));
        Self {
            cycle,
            first_new,
            axes,
            phyts,
            units: (in_units.len() / UNIT) as u32,
            form,
            grew: c.u32s("lay_grew"),
            generations: c.u32s("lay_generations"),
            in_axes: c.u32s("in_axes"),
            in_units,
            in_phyts: c.u32s("in_phyts"),
            in_pencils: c.u32s("in_pencils"),
        }
    }

    pub fn new_axes(&self) -> u32 {
        self.axes - self.first_new
    }

    pub fn grown(&self) -> u32 {
        self.grew.len() as u32
    }

    /// The lists: the tally, the grown axes, then room for the new axes'
    /// generations and a generation's list.
    pub fn lists(&self) -> Vec<u32> {
        let mut out = vec![0; TALLY];
        out.extend(&self.grew);
        out.resize(out.len() + 2 * self.new_axes() as usize, 0);
        out
    }
}

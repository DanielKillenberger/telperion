//! The settled tree, the device's node table and the CPU's capture
//! (`grow_capture_settled_at`), as the same named columns in the final
//! order: each axis's, each unit's and each phytomer's (`grown.rs`'s
//! tables, so `grown_compare` holds them exactly or by lineage), and
//! beside them each grown axis's final index, the leaf term's mean, the
//! disused pipes and the error a settle met.
use super::columns::Cap;
use super::grown::{Field, Kind, Snapshot};
use super::hier_scan::read_words;
use super::settle_device as d;
use super::settle_run::Settled;
use super::state::State;
use crate::{Gpu, Result};

/// The values beside the tables.
#[derive(Debug, Default, PartialEq)]
pub(super) struct Beside {
    pub index: Vec<u32>,
    pub leaf_mean: Vec<u64>,
    pub disused: Vec<u64>,
    pub error: Vec<u32>,
}

/// The axes' columns a step compares: name, the node table's column
/// and the capture's name, and the kind.
const AXES: [(&str, u32, Kind); 16] = [
    ("kind", 0, Kind::U32),
    ("parent", 1, Kind::U32),
    ("node", 2, Kind::U32),
    ("whorl", 3, Kind::U64),
    ("lineage", 5, Kind::U64),
    ("pa", 7, Kind::U32),
    ("birth", 8, Kind::U32),
    ("apex_end", 9, Kind::U32),
    ("vigour", 10, Kind::F64),
    ("blend", 12, Kind::F64),
    ("alive", 14, Kind::F32),
    ("rank", 15, Kind::F64),
    ("sleep", 17, Kind::U32),
    ("phase", 18, Kind::F32),
    ("units_n", 19, Kind::U32),
    ("phyt_n", 21, Kind::U32),
];
/// The phytomers' columns: an f32 column is widened to the f64 the CPU
/// holds.
const PHYTS: [(&str, u32, Kind); 7] = [
    ("cycle", 0, Kind::U32),
    ("key", 1, Kind::U64),
    ("rank", 3, Kind::F64),
    ("size", 5, Kind::F64),
    ("light", 7, Kind::F64),
    ("grown", 9, Kind::F32),
    ("scale", 10, Kind::F32),
];

/// An f32 column's words as the f64s that hold them.
fn widened(words: &[u32]) -> Vec<u32> {
    words.iter().flat_map(|&x| two(f64::from(f32::from_bits(x)).to_bits())).collect()
}

fn two(x: u64) -> [u32; 2] {
    [x as u32, (x >> 32) as u32]
}

fn field(name: &str, kind: Kind, words: Vec<u32>) -> Field {
    match kind {
        Kind::F32 if !name.starts_with("phase") => Field { name: name.into(), kind: Kind::F64, words: widened(&words) },
        _ => Field { name: name.into(), kind, words },
    }
}

/// Each row's axis, from each axis's count of rows.
fn owners(counts: &[u32]) -> Vec<u32> {
    counts.iter().enumerate().flat_map(|(a, &n)| std::iter::repeat_n(a as u32, n as usize)).collect()
}

impl Settled {
    /// The device's settled tree and what lies beside it.
    pub(super) fn snapshot(&self, gpu: &Gpu, state: &State) -> Result<(Snapshot, Beside)> {
        let mut beside = Beside { error: vec![u32::from(self.collapsed), u32::MAX], ..Beside::default() };
        if self.collapsed {
            return Ok((Snapshot::default(), beside));
        }
        let (t, l) = (&self.table, &self.layout);
        let nt = read_words(gpu, &self.nt, 0, t.words)?;
        let k = t.kept as usize;
        let col = |c: u32, kind: Kind| -> Vec<u32> {
            let at = (c * t.sk) as usize;
            match kind.words() {
                1 => nt[at..at + k].to_vec(),
                _ => (0..k).flat_map(|f| [nt[at + f], nt[at + t.sk as usize + f]]).collect(),
            }
        };
        let mut s = Snapshot::default();
        for (name, c, kind) in AXES {
            s.axes.fields.push(field(name, kind, col(c, kind)));
        }
        s.axes.axis = (0..t.kept).collect();
        s.lineage = col(5, Kind::U64).chunks(2).map(|w| u64::from(w[0]) | u64::from(w[1]) << 32).collect();
        let units = &nt[t.at_nu as usize..t.at_nu as usize + t.units as usize];
        s.units.fields.push(field("presence", Kind::F32, units.to_vec()));
        s.units.axis = owners(&col(19, Kind::U32));
        let p = t.phyts as usize;
        for (name, c, kind) in PHYTS {
            let at = (t.at_np + c * t.skp) as usize;
            let words = match kind.words() {
                1 => nt[at..at + p].to_vec(),
                _ => (0..p).flat_map(|r| [nt[at + r], nt[at + t.skp as usize + r]]).collect(),
            };
            s.phyts.fields.push(field(name, kind, words));
        }
        s.phyts.axis = owners(&col(21, Kind::U32));
        beside.index = read_words(gpu, &self.work, l.a(d::W_FINAL, 0), u64::from(l.n))?;
        let mean = read_words(gpu, &self.atom, d::T_LEAF as u64 + 2, 1)?[0];
        let has = self.flags & 3 == 3;
        let value = if has { f64::from(f32::from_bits(mean)).to_bits() } else { 0 };
        beside.leaf_mean = vec![u64::from(has), value];
        beside.disused = self.disused(gpu, state, &beside.index)?;
        Ok((s, beside))
    }

    /// The disused pipes kept, in grown order: each one's bearer's final
    /// index, its node (none at the tip), radius and share.
    fn disused(&self, gpu: &Gpu, state: &State, index: &[u32]) -> Result<Vec<u64>> {
        let l = &self.layout;
        let n = u64::from(l.n);
        let read = |c: u32| read_words(gpu, &self.work, l.a(c, 0), n);
        let (on, radius, share, node) = (read(d::W_DON)?, read(d::W_DR)?, read(d::W_DS)?, read(d::W_NODE)?);
        let cap = u64::from(state.caps.axes);
        let parent = read_words(gpu, &state.axes, 4 * cap, n)?;
        let origin = read_words(gpu, &state.axes, 3 * cap, n)?;
        let real = |x: u32| f64::from(f32::from_bits(x)).to_bits();
        let mut out = Vec::new();
        for i in (0..l.n as usize).filter(|&i| on[i] != 0) {
            let at = if origin[i] & 0xff == 1 { u64::from(node[i]) } else { u64::from(u32::MAX) };
            out.extend([u64::from(index[parent[i] as usize]), at, real(radius[i]), real(share[i])]);
        }
        Ok(out)
    }
}

/// The CPU's settled tree from its capture, in the same columns.
pub(super) fn cpu_settled(c: &Cap) -> (Snapshot, Beside) {
    let error = c.u32s("s_error");
    let mut beside = Beside { error: error.clone(), ..Beside::default() };
    if error[0] != 0 {
        return (Snapshot::default(), beside);
    }
    let words = |name: &str, kind: Kind| -> Vec<u32> {
        match kind {
            Kind::U32 | Kind::F32 => c.u32s(&format!("s_{name}")),
            _ => c.u64s(&format!("s_{name}")).into_iter().flat_map(two).collect(),
        }
    };
    let mut s = Snapshot::default();
    for (name, _, kind) in AXES {
        let kind = if name == "alive" { Kind::F64 } else { kind };
        s.axes.fields.push(Field { name: name.into(), kind, words: words(name, kind) });
    }
    s.lineage = c.u64s("s_lineage");
    s.axes.axis = (0..s.lineage.len() as u32).collect();
    s.units.fields.push(Field { name: "presence".into(), kind: Kind::F64, words: words("unit", Kind::F64) });
    s.units.axis = owners(&c.u32s("s_units_n"));
    for (name, _, kind) in PHYTS {
        let kind = if kind == Kind::F32 { Kind::F64 } else { kind };
        s.phyts.fields.push(Field { name: name.into(), kind, words: words(&format!("ph_{name}"), kind) });
    }
    s.phyts.axis = owners(&c.u32s("s_phyt_n"));
    beside.index = c.u32s("s_index");
    beside.leaf_mean = c.u64s("s_leaf_mean");
    beside.disused = c.u64s("s_disused");
    (s, beside)
}

/// The two sides beside the tables, compared exactly.
pub(super) fn beside_agree(a: &mut super::compare::Agreement, dev: &Beside, cpu: &Beside) {
    a.eq("error", &dev.error, &cpu.error);
    a.eq("grown index", &dev.index, &cpu.index);
    a.eq("leaf mean", &dev.leaf_mean, &cpu.leaf_mean);
    a.eq("disused", &dev.disused, &cpu.disused);
}

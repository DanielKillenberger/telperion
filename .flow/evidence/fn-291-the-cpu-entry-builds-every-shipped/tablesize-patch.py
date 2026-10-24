"""fn-291 task 3: a throwaway count of growth's tables, never committed.

`apply <root> <save-dir>` saves the files it edits into <save-dir>, adds a
module that sums `capacity() * size_of` over every table the grower holds,
and samples it after each cycle and as the settle begins. Each sample is a
JSON line on stderr with the live native heap at that moment, which the
driver (`tablesize.rs`) counts into `telperion_space::TABLE_HEAP`.
`revert <root> <save-dir>` copies the saved files back.
"""
import shutil
import sys
from pathlib import Path

MODULE = r'''//! fn-291 task 3 throwaway: growth's tables by bytes.
use super::Grower;
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub static TABLE_HEAP: AtomicUsize = AtomicUsize::new(0);

fn v<T>(x: &Vec<T>) -> usize {
    x.capacity() * size_of::<T>()
}

impl Grower<'_> {
    pub(crate) fn table_sample(&self, at: &str) {
        let mut t: Vec<(&str, usize)> = Vec::new();
        t.push(("axes", v(&self.axes)));
        t.push(("axes.phytomers", self.axes.iter().map(|a| v(&a.phytomers)).sum()));
        t.push(("axes.units", self.axes.iter().map(|a| v(&a.units)).sum()));
        t.push(("draws", v(&self.draws)));
        t.push(("draws.units", self.draws.iter().map(|d| v(&d.units)).sum()));
        t.push(("draws.nodes", self.draws.iter().map(|d| v(&d.nodes)).sum()));
        t.push(("draws.shade", self.draws.iter().map(|d| v(&d.shade)).sum()));
        t.push(("horizon", v(&self.horizon)));
        t.push(("marked", v(&self.marked)));
        t.push(("live", v(&self.live)));
        t.push(("units", v(&self.units)));
        t.push(("successor", v(&self.successor) + self.successor.iter().map(v).sum::<usize>()));
        t.push(("roots", v(&self.roots)));
        t.push(("asleep", v(&self.asleep) + self.asleep.iter().map(v).sum::<usize>()));
        t.push(("woken", v(&self.woken)));
        t.push(("leaves", v(&self.leaves)));
        t.push(("wakes", v(&self.wakes)));
        t.push(("handed", v(&self.handed)));
        t.push(("pencils", self.sketch.as_ref().map_or(0, |s| v(&s.pencils))));
        let s = &self.spare;
        t.push(("spare", v(&s.next) + v(&s.leads) + v(&s.node_buds) + v(&s.sleeping)));
        if let Some(w) = self.working.as_ref() {
            t.extend(w.table_bytes());
        }
        let total: usize = t.iter().map(|x| x.1).sum();
        let phytomers: usize = self.axes.iter().map(|a| a.phytomers.len()).sum();
        let mib = |b: usize| b as f64 / 1048576.0;
        let rows: Vec<String> = t.iter().map(|(k, b)| format!("\"{k}\":{:.1}", mib(*b))).collect();
        eprintln!(
            "{{\"at\":\"{at}\",\"axes\":{},\"phytomers\":{phytomers},\"tablesMiB\":{:.1},\"heapMiB\":{:.1},\"t\":{{{}}}}}",
            self.axes.len(), mib(total), mib(TABLE_HEAP.load(Relaxed)), rows.join(",")
        );
    }
}

pub fn sizes() -> String {
    use crate::structure::*;
    format!(
        "{{\"Axis\":{},\"Phytomer\":{},\"Origin\":{},\"Join\":{},\"Draws\":{},\"Unit\":{},\"Sleeper\":{},\"Woken\":{},\"Pencil\":{},\"Apex\":{},\"Link\":{},\"Pending\":{}}}",
        size_of::<Axis>(), size_of::<Phytomer>(), size_of::<Origin>(), size_of::<Join>(),
        size_of::<crate::presence::Draws>(), size_of::<crate::presence::Unit>(),
        size_of::<crate::dormant::Sleeper>(), size_of::<crate::dormant::Woken>(),
        size_of::<super::sketch::Pencil>(), size_of::<super::Apex>(),
        size_of::<crate::allocation::Link>(), size_of::<super::alone::Pending>(),
    )
}
'''

WORKING = r'''
impl Working {
    pub(in crate::grow) fn table_bytes(&self) -> Vec<(&'static str, usize)> {
        fn v<T>(x: &Vec<T>) -> usize { x.capacity() * std::mem::size_of::<T>() }
        let r = &self.retire;
        vec![
            ("w.links", v(&self.links)), ("w.pa", v(&self.pa)),
            ("w.by_node", v(&self.by_node) + self.by_node.iter().map(v).sum::<usize>()),
            ("w.frozen", v(&self.frozen)), ("w.open", v(&self.open)), ("w.list", v(&self.list)),
            ("w.closed", v(&self.closed)), ("w.at", v(&self.at)),
            ("w.passes", v(&self.light) + v(&self.cost) + v(&self.living) + v(&self.sub)
                + v(&self.incoming) + v(&self.bud) + v(&self.rest) + v(&self.kids)),
            ("r.fate", v(&r.fate)), ("r.frozen", v(&r.frozen)), ("r.log", v(&r.log)),
            ("r.rest", r.rest_bytes()),
        ]
    }
}
'''

RETIRE = r'''
impl Retirement {
    pub(in crate::grow) fn rest_bytes(&self) -> usize {
        fn v<T>(x: &Vec<T>) -> usize { x.capacity() * std::mem::size_of::<T>() }
        v(&self.woken) + v(&self.pending) + v(&self.fading)
    }
}
'''

FILES = {
    "crates/telperion-space/src/grow.rs": [("mod wake;\n", "mod wake;\npub mod tablesize;\n")],
    "crates/telperion-space/src/lib.rs": [(
        "pub use grow::{grow, sketch, Pass, Request, Stage};\n",
        "pub use grow::{grow, sketch, Pass, Request, Stage};\n"
        "pub use grow::tablesize::{sizes as table_sizes, TABLE_HEAP};\n",
    )],
    "crates/telperion-space/src/grow/cycle/paused.rs": [(
        "            self.g.count(cycle);\n",
        "            self.g.count(cycle);\n            self.g.table_sample(&format!(\"cycle {cycle}\"));\n",
    )],
    "crates/telperion-space/src/grow/settle.rs": [(
        "        self.mark();\n",
        "        self.table_sample(\"settle\");\n        self.mark();\n",
    )],
    "crates/telperion-space/src/grow/cycle/working.rs": [("APPEND", WORKING)],
    "crates/telperion-space/src/grow/cycle/retire.rs": [("APPEND", RETIRE)],
}
NEW = "crates/telperion-space/src/grow/tablesize.rs"


def apply(root: Path, save: Path) -> None:
    save.mkdir(parents=True, exist_ok=True)
    for rel, edits in FILES.items():
        path = root / rel
        shutil.copy2(path, save / rel.replace("/", "__"))
        text = path.read_text()
        for old, new in edits:
            if old == "APPEND":
                text += new
                continue
            if text.count(old) != 1:
                raise SystemExit(f"{rel}: anchor not found once")
            text = text.replace(old, new)
        path.write_text(text)
    (root / NEW).write_text(MODULE)


def revert(root: Path, save: Path) -> None:
    for rel in FILES:
        shutil.copy2(save / rel.replace("/", "__"), root / rel)
    (root / NEW).unlink()


if __name__ == "__main__":
    {"apply": apply, "revert": revert}[sys.argv[1]](Path(sys.argv[2]), Path(sys.argv[3]))

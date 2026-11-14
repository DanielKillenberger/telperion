"""fn-364 R9 (host, 14 Nov): turning follows the time grown, travel follows
the stretch. The lay's own steering (tropism towards the elevation, the
wander's kick, the bend memory's relaxation and turn) reads the internode's
allotted length; the travel, the run, the sag and the landing the laid one.
Each replacement matches once."""
import sys

ROOT = sys.argv[1]
S = f"{ROOT}/crates/telperion-space/src"


def edit(path, pairs):
    p = f"{S}/{path}"
    s = open(p).read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            raise SystemExit(f"{path}: {n} matches for {old[:70]!r}")
        s = s.replace(old, new)
    open(p, "w").write(s)


edit("geometry/lay.rs", [
    ("""    /// to none at `total`, the axis's scaled reach. With `load`, it turns
    /// down by its sag first. The internode's length laid, in metres.
    pub fn step(
        &mut self,
        phytomer: &mut impl Node,""",
     """    /// to none at `total`, the axis's scaled reach. With `load`, it turns
    /// down by its sag first. The internode's length laid, in metres: its
    /// allotted length (`Node::scale`) at its `stretch` (fn-364). The
    /// shoot's own steering, its tropism and its wander's kick and memory,
    /// turns by the length grown, the allotted one, so a phytomer laid at
    /// stretch 0 turns as it grew and does not move (fn-364 R9, host).
    pub fn step(
        &mut self,
        (phytomer, stretch): (&mut impl Node, f32),"""),
    ("""        let scale = phytomer.length() as f32;""",
     """        let allotted = phytomer.scale() as f32;
        let scale = allotted * stretch;"""),
    ("""        let length = state.internode as f32 * scale * varied;
        let tropic = form.tropism as f32 * self.turning * length;""",
     """        let length = state.internode as f32 * scale * varied;
        let grown = state.internode as f32 * allotted * varied;
        let tropic = form.tropism as f32 * self.turning * grown;"""),
    ("""            let keep = falloff_keep(length / form.bend_length as f32);""",
     """            let keep = falloff_keep(grown / form.bend_length as f32);"""),
    ("""            let angle = turns(q::length(curve) * length);""",
     """            let angle = turns(q::length(curve) * grown);"""),
    ("""            let angle = turns(form.wander as f32 * length * signed);""",
     """            let angle = turns(form.wander as f32 * grown * signed);"""),
    ("""        layer.step(phytomer, state, bend, load.map(|l| &l[k]), trunk)?;""",
     """        let stretch = phytomer.stretch;
        layer.step((phytomer, stretch), state, bend, load.map(|l| &l[k]), trunk)?;"""),
])

edit("grow/draw.rs", [
    ("""            // Laid at its size and its tip's share, then left at its bare
            // scale for what it bears.
            let bare = p.scale;
            p.scale = bare * p.size * shade.stretch;
            let length = (pencil.layer)
                .step(p, state, (bent, total), None, pencil.trunk)""",
     """            // Laid at its size and its tip's share, turning by its size
            // alone (fn-364 R9), then left at its bare scale for what it
            // bears.
            let bare = p.scale;
            p.scale = bare * p.size;
            let length = (pencil.layer)
                .step((p, shade.stretch), state, (bent, total), None, pencil.trunk)"""),
])

edit("geometry/lay_tests.rs", [
    ("""    layer.step(&mut p, &state, (0.0, 0.0), None, false).unwrap();""",
     """    layer.step((&mut p, 1.0), &state, (0.0, 0.0), None, false).unwrap();"""),
])
print("r9 applied")

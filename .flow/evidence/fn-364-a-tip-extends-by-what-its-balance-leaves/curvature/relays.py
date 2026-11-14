"""fn-364 RELAYS (tooling, never committed): prints every apex stop
(STOPLOG=1) and, for one lineage (LAYWATCH=<lineage>), every settled lay
step's turn split by term. Applies to fn-364's and fn-363's checkouts."""
import sys

W = sys.argv[1]
S = f"{W}/crates/telperion-space/src"


def edit(path, pairs):
    p = f"{S}/{path}"
    s = open(p).read()
    for old, new in pairs:
        if s.count(old) != 1:
            raise SystemExit(f"{path}: {s.count(old)} matches for {old[:60]!r}")
        s = s.replace(old, new)
    open(p, "w").write(s)


edit("geometry/lay.rs", [
    ("""    pub fn step(
        &mut self,""",
     """    #[allow(clippy::too_many_lines)]
    pub fn step(
        &mut self,"""),
    ("""        let tropic = form.tropism as f32 * self.turning * length;""",
     """        let r0 = running;
        let tropic = form.tropism as f32 * self.turning * length;"""),
    ("""        running = bent;
        // The node's wander""",
     """        running = bent;
        let r1 = running;
        // The node's wander"""),
    ("""        // Sag: the beam's curvature, its moment over its stiffness, turns""",
     """        let r2 = running;
        // Sag: the beam's curvature, its moment over its stiffness, turns"""),
    ("""        // Its own pull, and those it carries on from the axis it relays,""",
     """        let r3 = running;
        // Its own pull, and those it carries on from the axis it relays,"""),
    ("""        let radius = phytomer.radius() as f32;
        if !trunk && direction[2] < 0.0 {""",
     """        let d0 = direction;
        let radius = phytomer.radius() as f32;
        if !trunk && direction[2] < 0.0 {"""),
    ("""        let mut next = add(tip, scaled(direction, length));""",
     """        let d1 = direction;
        let mut next = add(tip, scaled(direction, length));"""),
    ("""        let side = sub(beside, scaled(direction, dot(beside, direction)));
        phytomer.lay(next, direction, unit(side).unwrap_or(beside));""",
     """        if WATCH.with(|w| w.get()) {
            let a = |x: Q3, y: Q3| f64::from(dot(x, y).clamp(-1.0, 1.0)).acos().to_degrees();
            eprintln!(
                "LAY|{}|{}|{scale:.4}|{length:.5}|{:.4}|{bend:.4}|{lift:.4}|{fade:.4}|{over:.4}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.4}",
                phytomer.key(), phytomer.cycle(), total, a(r0, r1), a(r1, r2), a(r2, r3), a(r3, d0), a(d0, d1), a(d1, direction), run
            );
        }
        let side = sub(beside, scaled(direction, dot(beside, direction)));
        phytomer.lay(next, direction, unit(side).unwrap_or(beside));"""),
    ("""pub(crate) fn lay(
    axis: &mut Axis,""",
     """thread_local! {
    /// fn-364 RELAYS: whether the axis being laid is the watched lineage.
    pub(crate) static WATCH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn watched(lineage: u64) -> bool {
    std::env::var("LAYWATCH").ok().and_then(|v| v.parse::<u64>().ok()) == Some(lineage)
}

pub(crate) fn lay(
    axis: &mut Axis,"""),
    ("""    let turning = tropism(axis.lineage, state.form.tropism_spread);
    let mut layer = Layer::bent(frame, curve, turning)""",
     """    let turning = tropism(axis.lineage, state.form.tropism_spread);
    WATCH.with(|w| w.set(watched(axis.lineage)));
    if watched(axis.lineage) {
        eprintln!("AXIS|{}|{}|{}|{:?}", axis.lineage, axis.pa, axis.birth, axis.origin);
    }
    let mut layer = Layer::bent(frame, curve, turning)"""),
])

edit("grow/stop.rs", [
    ("""        *self.units_mut(apex.axis) = spent;
        if why == Stop::Ended {""",
     """        if std::env::var_os("STOPLOG").is_some() {
            let a = self.axis(apex.axis);
            eprintln!("STOP|{}|{}|{}|{:?}|{used:.3}", a.lineage, cycle, a.pa, why);
        }
        *self.units_mut(apex.axis) = spent;
        if why == Stop::Ended {"""),
])

edit("grow/cycle/passes.rs", [
    ("""                    self.axes[apex.axis].apex_end = Some(cycle - 1);""",
     """                    if std::env::var_os("STOPLOG").is_some() {
                        let a = &self.axes[apex.axis];
                        eprintln!("STOP|{}|{}|{}|Shed|1.000", a.lineage, cycle - 1, a.pa);
                    }
                    self.axes[apex.axis].apex_end = Some(cycle - 1);"""),
])
print("relays instrumented")

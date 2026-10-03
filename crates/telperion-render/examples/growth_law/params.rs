//! The law's settings: every one continuous, each with a neutral value.
//! A species is a JSON object of the settings it moves off neutral.

#[derive(Clone, Copy, Debug)]
pub struct Params {
    // Environment and resource (Pałubicki 2009).
    /// Metamer unit as a share of the envelope height.
    pub unit: f64,
    /// Markers per unit cube of the envelope's box.
    pub density: f64,
    /// Perception radius and occupancy radius, in units.
    pub perception: f64,
    pub occupancy: f64,
    /// Perception cone half-angle, degrees.
    pub cone: f64,
    /// Growth cycles (years).
    pub cycles: f64,
    /// Share of the cycles over which the crown grows to the envelope; the
    /// seedling's crown is `seedling` of it.
    pub ontogeny: f64,
    pub seedling: f64,
    /// How markers spread over the crown's ages (3 = evenly by volume).
    pub young: f64,
    /// Borchert-Honda apical control and the base resource factor.
    pub lambda: f64,
    pub alpha: f64,
    /// Light's weight on a bud's resource: (1 - w) + w * exposure.
    pub light: f64,
    pub shade: f64,
    pub falloff: f64,
    /// Direction weights: the environment's pull and the tropism.
    pub xi: f64,
    pub eta: f64,
    // The architecture space.
    /// Apical persistence: the chance a terminal bud survives its shoot
    /// (1 monopodial, toward 0 sympodial).
    pub persistence: f64,
    /// Lean by physiological age: the tropism turns from up toward
    /// horizontal-outward by clamp(lean0 + lean1 * phi).
    pub lean0: f64,
    pub lean1: f64,
    /// Straightening: radians per cycle that wood turns toward up, scaled by
    /// its share of the root's radius.
    pub straighten: f64,
    /// Lateral position profile along a shoot: weight exp(acrotony * 3 * (s - 0.5)),
    /// positive acrotonic, negative basitonic.
    pub acrotony: f64,
    /// Rhythm: the share of a shoot's laterals set at its distal node (tiers).
    pub rhythm: f64,
    /// Laterals per metamer at physiological age 0 (0 = unbranched).
    pub branching: f64,
    /// Lateral development falls with age as (1 - phi)^fate.
    pub fate: f64,
    /// Shoot length falls with age to `short` of a young shoot's.
    pub short: f64,
    /// Physiological age a lateral starts above its parent's shoot.
    pub phi_step: f64,
    /// Age rises by `drift` a cycle at no vigour, none at the reference vigour.
    pub drift: f64,
    /// Age falls by `reiteration` per unit of vigour above the reference.
    pub reiteration: f64,
    /// The reference vigour (metamers a cycle).
    pub v_ref: f64,
    /// Branch angle at departure, degrees, at age 0 and age 1.
    pub angle0: f64,
    pub angle1: f64,
    /// Two-ranked bud bearing blend (0 spiral, 1 distichous).
    pub distich: f64,
    /// Shedding: a lateral branch whose remembered resource per tip falls
    /// under `shed` is cast off; `memory` is the remembering rate.
    pub shed: f64,
    pub memory: f64,
    /// Smallest age (cycles) at which a branch can be shed.
    pub shed_age: f64,
    /// Pipe exponent (None: the preset's).
    pub exponent: f64,
    pub threads: f64,
    pub max_nodes: f64,
}

impl Default for Params {
    /// The neutral point: Pałubicki's space colonisation with every axis
    /// alike, monopodial, no lean, no straightening, even laterals.
    fn default() -> Self {
        Self {
            unit: 1.0 / 128.0,
            density: 0.05,
            perception: 5.0,
            occupancy: 1.5,
            cone: 45.0,
            cycles: 40.0,
            ontogeny: 0.6,
            seedling: 0.12,
            young: 3.0,
            lambda: 0.52,
            alpha: 2.0,
            light: 0.0,
            shade: 0.03,
            falloff: 1.8,
            xi: 0.1,
            eta: 0.05,
            persistence: 1.0,
            lean0: 0.0,
            lean1: 0.0,
            straighten: 0.0,
            acrotony: 0.0,
            rhythm: 0.0,
            branching: 1.0,
            fate: 0.0,
            short: 1.0,
            phi_step: 0.0,
            drift: 0.0,
            reiteration: 0.0,
            v_ref: 2.0,
            angle0: 40.0,
            angle1: 40.0,
            distich: 0.0,
            shed: 0.0,
            memory: 0.3,
            shed_age: 3.0,
            exponent: 0.0,
            threads: 16.0,
            max_nodes: 600_000.0,
        }
    }
}

macro_rules! fields {
    ($($f:ident = $k:literal),* $(,)?) => {
        impl Params {
            /// Applies the keys of a JSON object; an unknown key is refused.
            pub fn apply(&mut self, v: &serde_json::Value) -> Result<(), String> {
                let Some(o) = v.as_object() else { return Err("settings must be an object".into()) };
                for (k, x) in o {
                    let x = x.as_f64().ok_or_else(|| format!("{k}: not a number"))?;
                    match k.as_str() {
                        $($k => self.$f = x,)*
                        _ if k.starts_with('_') => {}
                        _ => return Err(format!("unknown setting {k}")),
                    }
                }
                Ok(())
            }
            pub fn to_json(&self) -> serde_json::Value {
                serde_json::json!({ $($k: self.$f),* })
            }
            pub fn set(&mut self, k: &str, x: f64) -> Result<(), String> {
                match k { $($k => self.$f = x,)* _ => return Err(format!("unknown setting {k}")) }
                Ok(())
            }
            pub fn get(&self, k: &str) -> Option<f64> {
                match k { $($k => Some(self.$f),)* _ => None }
            }
        }
    };
}

fields! {
    unit = "unit", density = "density", perception = "perception", occupancy = "occupancy", cone = "cone",
    cycles = "cycles", ontogeny = "ontogeny", seedling = "seedling", young = "young", lambda = "lambda", alpha = "alpha",
    light = "light", shade = "shade", falloff = "falloff", xi = "xi", eta = "eta",
    persistence = "persistence", lean0 = "lean0", lean1 = "lean1", straighten = "straighten",
    acrotony = "acrotony", rhythm = "rhythm", branching = "branching", fate = "fate", short = "short",
    phi_step = "phiStep", drift = "drift", reiteration = "reiteration", v_ref = "vRef",
    angle0 = "angle0", angle1 = "angle1", distich = "distich", shed = "shed", memory = "memory",
    shed_age = "shedAge", exponent = "exponent", threads = "threads", max_nodes = "maxNodes",
}

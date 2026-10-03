//! The law's settings: every one continuous, each with a neutral value.
//! A species is a JSON object of the settings it moves off neutral.

#[derive(Clone, Copy, Debug)]
pub struct Params {
    // Environment and resource (Pałubicki 2009).
    /// Metamer unit as a share of the envelope height.
    pub unit: f64,
    /// Growth cycles (years).
    pub cycles: f64,
    /// Borchert-Honda apical control and the base resource factor.
    pub lambda: f64,
    pub alpha: f64,
    /// Shadow grid: a bud's shade a b^-q on the q-th layer below, `depth` layers.
    pub shade: f64,
    pub falloff: f64,
    pub depth: f64,
    /// Pull toward the authored crown from outside it (0 neutral).
    pub envelope: f64,
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
    /// Lateral development falls with age as (1 - phi)^fate and with vigour
    /// below the reference as (v / vRef)^vigourFate.
    pub fate: f64,
    pub vigour_fate: f64,
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
    /// Carbon balance: upkeep per unit of wood volume against light (0 = free wood).
    pub upkeep: f64,
    /// Shedding: the yearly chance, at most shed, that a branch whose
    /// remembered relative balance is past -tolerance is cast off, rising
    /// smoothly over shedWidth; memory is the remembering rate.
    pub shed: f64,
    pub tolerance: f64,
    pub shed_width: f64,
    pub memory: f64,
    /// A dormant bud dies each year with chance budDeath x its dormant years.
    pub bud_death: f64,
    /// Smallest age (cycles) at which a branch can be shed.
    pub shed_age: f64,
    /// Pipe exponent (None: the preset's).
    pub exponent: f64,
    pub threads: f64,
    pub max_nodes: f64,
}

impl Default for Params {
    /// The neutral point: Pałubicki's light-driven tree with every axis
    /// alike, monopodial, no lean, no straightening, even laterals.
    fn default() -> Self {
        Self {
            unit: 1.0 / 128.0,
            cycles: 40.0,
            lambda: 0.52,
            alpha: 2.0,
            shade: 0.03,
            falloff: 1.8,
            depth: 6.0,
            envelope: 0.0,
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
            vigour_fate: 0.0,
            short: 1.0,
            phi_step: 0.0,
            drift: 0.0,
            reiteration: 0.0,
            v_ref: 2.0,
            angle0: 40.0,
            angle1: 40.0,
            distich: 0.0,
            upkeep: 0.0,
            shed: 0.0,
            tolerance: 0.0,
            shed_width: 0.3,
            bud_death: 0.0,
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
    unit = "unit", cycles = "cycles", lambda = "lambda", alpha = "alpha",
    shade = "shade", falloff = "falloff", depth = "depth", envelope = "envelope", xi = "xi", eta = "eta",
    persistence = "persistence", lean0 = "lean0", lean1 = "lean1", straighten = "straighten",
    acrotony = "acrotony", rhythm = "rhythm", branching = "branching", fate = "fate", vigour_fate = "vigourFate", short = "short",
    phi_step = "phiStep", drift = "drift", reiteration = "reiteration", v_ref = "vRef",
    angle0 = "angle0", angle1 = "angle1", distich = "distich", upkeep = "upkeep", shed = "shed", tolerance = "tolerance", shed_width = "shedWidth", bud_death = "budDeath", memory = "memory",
    shed_age = "shedAge", exponent = "exponent", threads = "threads", max_nodes = "maxNodes",
}

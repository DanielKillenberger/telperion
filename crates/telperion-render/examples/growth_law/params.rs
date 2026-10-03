//! The probe's settings (R1a, stage 1's Troll module rule): every one continuous, each with a neutral value. A
//! species is a JSON object of the settings it moves off neutral. Every
//! curve of physiological age phi is a blend between its phi = 0 (trunk-like,
//! young) and phi = 1 (short-shoot, old) values.

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Metamer length at phi 0, as a share of the envelope height.
    pub unit: f64,
    /// Growth cycles (years).
    pub cycles: f64,
    /// Metamers per yearly growth unit at phi 0 and phi 1.
    pub n0: f64,
    pub n1: f64,
    /// Internode length at phi 1 as a share of phi 0's.
    pub short: f64,
    /// Metamers and internode length blend by phi^vigourShape (1 linear;
    /// lower makes a lateral weaker than its carrier sooner).
    pub vigour_shape: f64,
    /// The share of the way to phi 1 an axis moves per year it grows.
    pub drift: f64,
    /// Establishment: the tree's vigour at age 0 (1 flat) and the years
    /// over which it rises toward 1.
    pub est0: f64,
    pub est_years: f64,
    /// Troll's module (stage 1, [M98]): a module's tip tilts toward its
    /// plagiotropic heading as the module ages, by tilt x (1 - exp(-years /
    /// tiltYears)) added to the lean (0: the tip stays as phi sets it).
    pub tilt: f64,
    pub tilt_years: f64,
    /// The curvature zone: the first node whose direction falls below this
    /// elevation, degrees. Wood before it is the module's erect base.
    pub bend_elev: f64,
    /// The relay: the yearly chance, once a module has its curvature zone,
    /// that a bud there on the upper side takes over (0: monopodial), times
    /// (1 - the module's phi at birth)^relayShape.
    pub relay: f64,
    pub relay_shape: f64,
    /// The relay's elevation above the curvature zone's, degrees, and the
    /// share of a random turn of its heading about the vertical.
    pub relay_up: f64,
    pub relay_turn: f64,
    /// The relayed head becomes a branch: its phi jumps by headJump (as a
    /// lateral's birth jump) and it lives headLife x the establishment
    /// curve more years (short early, longer as the tree establishes).
    pub head_jump: f64,
    pub head_life: f64,
    /// The fork near maturity: relays per relay event are 1 + fork x a
    /// logistic of the tree's age about forkAge (width forkWidth years);
    /// forked relays leave at forkAngle off the vertical, spread evenly,
    /// and age by reiterStep x (relays - 1) as a birth jump in phi.
    pub fork: f64,
    pub fork_age: f64,
    pub fork_width: f64,
    pub fork_angle: f64,
    pub reiter_step: f64,
    /// Sequential reiteration: near maturity a lateral's phi falls back
    /// toward its carrier's by reiterate x maturity x its place along the
    /// unit x the shoot's vigour (0: laterals keep their birth jump).
    pub reiterate: f64,
    /// Laterals per metamer at phi 0, falling as (1 - phi)^fate.
    pub branching: f64,
    pub fate: f64,
    /// A lateral's birth jump: phiStep + zone x (1 - u) + vigourJump x
    /// (1 - the shoot's vigour), u its place from the unit's base; its phi
    /// moves toward 1 by the fraction 1 - exp(-jump), never past it.
    pub phi_step: f64,
    pub zone: f64,
    pub vigour_jump: f64,
    /// Axis lifespan in years at phi 0 and phi 1, blended by
    /// 1 - (1 - phi)^lifeShape (higher falls sooner); an axis past it is
    /// pruned with what it carries.
    pub life0: f64,
    pub life1: f64,
    pub life_shape: f64,
    /// Lateral count profile along the unit: weight exp(acrotony x 3 x (u - 0.5)).
    pub acrotony: f64,
    /// Rhythm: the share of a unit's laterals set at its distal node (tiers).
    pub rhythm: f64,
    /// Lean by phi: the tropism turns from up toward horizontal-outward by
    /// clamp(lean0 + lean1 x phi); eta is its pull per metamer.
    pub lean0: f64,
    pub lean1: f64,
    pub eta: f64,
    /// The most an axis leans (1: horizontal; lower keeps plagiotropic
    /// axes oblique, at atan((1 - plagio) / plagio) above horizontal).
    pub plagio: f64,
    /// Gravitropism near the base: the reach, in internodes, of a pull up
    /// that is 1 at the ground (0 off).
    pub ground: f64,
    /// Straightening: radians per year that a module's erect base (wood
    /// before its curvature zone) turns toward up, times (1 - its axis's phi
    /// at birth); the end pass is the yearly turn's sum.
    pub straighten: f64,
    /// How far an erect base straightens: the share (1 - phi)^shape of its
    /// gap to vertical (0: all the way; higher leaves older axes oblique).
    pub straighten_shape: f64,
    /// Branch angle at departure, degrees, at the lateral's phi 0 and 1.
    pub angle0: f64,
    pub angle1: f64,
    /// Two-ranked bud bearing blend (0 spiral, 1 distichous).
    pub distich: f64,
    /// Pipe exponent (0: the preset's).
    pub exponent: f64,
    /// A tip's radius, metres, so a young tree is drawn at its own girth
    /// (0: the root takes the preset's radius whatever the age).
    pub tip_radius: f64,
    /// A tree past this many nodes is an error, never a capped tree.
    pub max_nodes: f64,
}

impl Default for Params {
    /// The neutral point: a monopodial tree (no tilt, no relay, no fork)
    /// whose laterals age by a quarter a generation, even along the unit,
    /// upright, unstraightened.
    fn default() -> Self {
        Self {
            unit: 1.0 / 128.0,
            cycles: 40.0,
            n0: 3.0,
            n1: 2.0,
            short: 1.0,
            vigour_shape: 1.0,
            drift: 0.02,
            est0: 1.0,
            est_years: 10.0,
            tilt: 0.0,
            tilt_years: 3.0,
            bend_elev: 45.0,
            relay: 0.0,
            relay_shape: 4.0,
            relay_up: 30.0,
            relay_turn: 0.0,
            head_jump: 0.0,
            head_life: 1000.0,
            fork: 0.0,
            fork_age: 40.0,
            fork_width: 4.0,
            fork_angle: 35.0,
            reiter_step: 0.0,
            reiterate: 0.0,
            branching: 1.0,
            fate: 1.0,
            phi_step: 0.25,
            zone: 0.0,
            vigour_jump: 0.0,
            life0: 1000.0,
            life1: 1000.0,
            life_shape: 1.0,
            acrotony: 0.0,
            rhythm: 0.0,
            lean0: 0.0,
            lean1: 0.0,
            eta: 0.05,
            plagio: 1.0,
            ground: 0.0,
            straighten: 0.0,
            straighten_shape: 0.0,
            angle0: 40.0,
            angle1: 40.0,
            distich: 0.0,
            exponent: 0.0,
            tip_radius: 0.0,
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
    unit = "unit", cycles = "cycles", n0 = "n0", n1 = "n1", short = "short", vigour_shape = "vigourShape", drift = "drift", est0 = "est0", est_years = "estYears",
    tilt = "tilt", tilt_years = "tiltYears", bend_elev = "bendElev", relay = "relay", relay_shape = "relayShape", relay_up = "relayUp",
    relay_turn = "relayTurn", head_jump = "headJump", head_life = "headLife", fork = "fork", fork_age = "forkAge", fork_width = "forkWidth",
    fork_angle = "forkAngle", reiter_step = "reiterStep", reiterate = "reiterate",
    branching = "branching", fate = "fate", phi_step = "phiStep", zone = "zone", vigour_jump = "vigourJump", life0 = "life0", life1 = "life1", life_shape = "lifeShape", acrotony = "acrotony", rhythm = "rhythm",
    lean0 = "lean0", lean1 = "lean1", eta = "eta", plagio = "plagio", ground = "ground", straighten = "straighten", straighten_shape = "straightenShape",
    angle0 = "angle0", angle1 = "angle1", distich = "distich", exponent = "exponent", tip_radius = "tipRadius", max_nodes = "maxNodes",
}

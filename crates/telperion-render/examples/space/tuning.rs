//! The still runner's light and carbon flags (fn-197): the site's light,
//! and values set on every PA of the species before it grows, so a probe
//! needs no edit to the species' file.
use telperion_space::{Light, Species};

/// The flags read, each unset until named.
#[derive(Debug, Default)]
pub struct Tuning {
    light: Option<Light>,
    shade: Option<(f64, f64)>,
    control: Option<f64>,
    controls: Vec<(usize, f64)>,
    balance: Option<(f64, f64, f64)>,
    girth: Option<(f64, f64)>,
    leaf_area: Option<f64>,
    unsagged: bool,
    /// Appended to every still's name.
    pub tag: Option<String>,
    /// The shots rendered, all when unset.
    pub only: Option<Vec<String>>,
}

/// The usage line of these flags.
pub const USAGE: &str = "[--light k,sky] [--shade φ,ψ] [--control λ] [--control-pas <pas>:λ] \
     [--balance r,κ,τ] [--girth retained,χ] [--leaf-area m2] [--unsagged] [--tag t] [--shots a,b]";

impl Tuning {
    /// Reads `word` and its value from `words` when it is one of these
    /// flags: Ok(false) when it is not.
    pub fn take<'a>(
        &mut self,
        word: &str,
        words: &mut impl Iterator<Item = &'a String>,
    ) -> Result<bool, String> {
        let mut value = || words.next().ok_or(format!("{word} needs a value"));
        match word {
            "--light" => {
                let [extinction, sky] = numbers(value()?, word)?[..] else {
                    return Err("--light needs k,sky".into());
                };
                self.light = Some(Light { extinction, sky });
            }
            "--shade" => self.shade = Some(two(value()?, word)?),
            "--girth" => self.girth = Some(two(value()?, word)?),
            "--balance" => {
                let [r, k, t] = numbers(value()?, word)?[..] else {
                    return Err("--balance needs r,κ,τ".into());
                };
                self.balance = Some((r, k, t));
            }
            "--control" => self.control = Some(one(value()?, word)?),
            "--leaf-area" => self.leaf_area = Some(one(value()?, word)?),
            "--control-pas" => {
                let w = value()?;
                let (pas, l) = w.split_once(':').ok_or("--control-pas needs <pas>:λ")?;
                let l = one(l, word)?;
                for pa in pas.split(',') {
                    let pa = pa
                        .parse::<usize>()
                        .map_err(|e| format!("{word} {pa}: {e}"))?;
                    self.controls.push((pa, l));
                }
            }
            "--unsagged" => self.unsagged = true,
            "--tag" => self.tag = Some(value()?.clone()),
            "--shots" => self.only = Some(value()?.split(',').map(String::from).collect()),
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// The site's light, neutral unless named.
    pub fn light(&self) -> Light {
        self.light.unwrap_or(Light::NEUTRAL)
    }

    /// Sets the named values on every PA of `species`, `--control-pas`
    /// after `--control`. Err: a PA past the species' table.
    pub fn apply(&self, species: &mut Species) -> Result<(), String> {
        for state in &mut species.states {
            if let Some((r, k, t)) = self.balance {
                (state.upkeep, state.balance_hazard, state.tolerance) = (r, k, t);
            }
            if let Some(lambda) = self.control {
                state.apical_control = lambda;
            }
            if let Some(area) = self.leaf_area {
                state.leaf_area = area;
            }
            if let Some((retained, chi)) = self.girth {
                (state.retained, state.leaf_girth) = (retained, chi);
            }
            if let Some((phi, psi)) = self.shade {
                (state.shade_hazard, state.shade_size) = (phi, psi);
            }
            if self.unsagged {
                state.form.sag = 0.0;
            }
        }
        let pas = species.states.len();
        for &(pa, lambda) in &self.controls {
            let state = species
                .states
                .get_mut(pa)
                .ok_or(format!("--control-pas: no PA {pa} in a species of {pas}"))?;
            state.apical_control = lambda;
        }
        Ok(())
    }

    /// Whether `shot` is rendered.
    pub fn shoots(&self, shot: &str) -> bool {
        self.only
            .as_ref()
            .is_none_or(|o| o.iter().any(|x| x == shot))
    }
}

fn numbers(w: &str, flag: &str) -> Result<Vec<f64>, String> {
    w.split(',').map(|t| one(t, flag)).collect()
}

fn one(t: &str, flag: &str) -> Result<f64, String> {
    t.parse::<f64>().map_err(|e| format!("{flag} {t}: {e}"))
}

fn two(w: &str, flag: &str) -> Result<(f64, f64), String> {
    let [a, b] = numbers(w, flag)?[..] else {
        return Err(format!("{flag} needs <a>,<b>"));
    };
    Ok((a, b))
}

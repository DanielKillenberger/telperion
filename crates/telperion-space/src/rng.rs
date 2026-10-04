use crate::species::NodeLaw;

/// SplitMix64: one stream per tree, drawn in the engine's fixed order.
#[derive(Debug, Clone)]
pub(crate) struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform on [0, 1).
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// True with probability `p`; draws nothing when the answer is certain.
    pub fn chance(&mut self, p: f64) -> bool {
        p >= 1.0 || (p > 0.0 && self.unit() < p)
    }

    /// One node count under `law`.
    pub fn nodes(&mut self, law: NodeLaw) -> u32 {
        match law {
            NodeLaw::Uniform { min, max } if min == max => min,
            NodeLaw::Uniform { min, max } => min + (self.unit() * f64::from(max - min + 1)) as u32,
            NodeLaw::Poisson { mean } => self.poisson(mean),
        }
    }

    /// Knuth's multiplication method; the mean is at most 500, where
    /// e^-mean is still a normal double.
    fn poisson(&mut self, mean: f64) -> u32 {
        if mean <= 0.0 {
            return 0;
        }
        let floor = (-mean).exp();
        let mut product = self.unit();
        let mut count = 0;
        while product > floor {
            count += 1;
            product *= self.unit();
        }
        count
    }

    /// A bud's PA under `lateral`, or none for a bare bud.
    pub fn bud(&mut self, lateral: &[f64]) -> Option<usize> {
        let total: f64 = lateral.iter().sum();
        if total <= 0.0 {
            return None;
        }
        let draw = self.unit();
        let mut cumulative = 0.0;
        for (pa, &p) in lateral.iter().enumerate() {
            cumulative += p;
            if draw < cumulative {
                return Some(pa);
            }
        }
        None
    }
}

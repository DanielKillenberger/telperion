/// SplitMix32, including seed zero. Integer arithmetic is identical on native and Wasm.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u32,
}
impl Rng {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x9e3779b9);
        let mut z = self.state;
        z = (z ^ (z >> 16)).wrapping_mul(0x21f0aaad);
        z = (z ^ (z >> 15)).wrapping_mul(0x735a2d97);
        z ^ (z >> 15)
    }
    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / 4294967296.0
    }
    pub fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_f64()
    }
}

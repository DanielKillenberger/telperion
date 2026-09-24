//! One pinned pure-Rust implementation on native and wasm, including hypot.
//! Distinct method names prevent accidental dispatch to f64's platform libm.
mod hypot;
pub(crate) trait Transcendental {
    fn sin_fixed(self) -> f64;
    fn cos_fixed(self) -> f64;
    fn sin_cos_fixed(self) -> (f64, f64);
    fn acos_fixed(self) -> f64;
    fn atan2_fixed(self, other: f64) -> f64;
    fn exp_fixed(self) -> f64;
    fn powf_fixed(self, other: f64) -> f64;
    fn cbrt_fixed(self) -> f64;
    fn hypot_fixed(self, other: f64) -> f64;
}
impl Transcendental for f64 {
    fn sin_fixed(self) -> f64 {
        libm::sin(self)
    }
    fn cos_fixed(self) -> f64 {
        libm::cos(self)
    }
    fn sin_cos_fixed(self) -> (f64, f64) {
        libm::sincos(self)
    }
    fn acos_fixed(self) -> f64 {
        libm::acos(self)
    }
    fn atan2_fixed(self, other: f64) -> f64 {
        libm::atan2(self, other)
    }
    fn exp_fixed(self) -> f64 {
        libm::exp(self)
    }
    fn powf_fixed(self, other: f64) -> f64 {
        libm::pow(self, other)
    }
    fn cbrt_fixed(self) -> f64 {
        libm::cbrt(self)
    }
    fn hypot_fixed(self, other: f64) -> f64 {
        hypot::hypot(self, other)
    }
}

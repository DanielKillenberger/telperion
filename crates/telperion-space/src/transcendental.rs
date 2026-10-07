//! One pinned pure-Rust implementation on native and wasm, including hypot,
//! for the engine and the core alike (fn-11; fn-198 host decision 10): std's
//! transcendentals are the platform's, so a tree grown natively and in Wasm
//! would differ in its last bits. Distinct method names prevent accidental
//! dispatch to f64's platform libm.
mod hypot;
pub trait Transcendental {
    fn sin_fixed(self) -> f64;
    fn cos_fixed(self) -> f64;
    fn sin_cos_fixed(self) -> (f64, f64);
    fn tan_fixed(self) -> f64;
    fn asin_fixed(self) -> f64;
    fn acos_fixed(self) -> f64;
    fn atan_fixed(self) -> f64;
    fn atan2_fixed(self, other: f64) -> f64;
    fn exp_fixed(self) -> f64;
    fn exp_m1_fixed(self) -> f64;
    fn ln_fixed(self) -> f64;
    fn ln_1p_fixed(self) -> f64;
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
    fn tan_fixed(self) -> f64 {
        libm::tan(self)
    }
    fn asin_fixed(self) -> f64 {
        libm::asin(self)
    }
    fn acos_fixed(self) -> f64 {
        libm::acos(self)
    }
    fn atan_fixed(self) -> f64 {
        libm::atan(self)
    }
    fn atan2_fixed(self, other: f64) -> f64 {
        libm::atan2(self, other)
    }
    fn exp_fixed(self) -> f64 {
        libm::exp(self)
    }
    fn exp_m1_fixed(self) -> f64 {
        libm::expm1(self)
    }
    fn ln_fixed(self) -> f64 {
        libm::log(self)
    }
    fn ln_1p_fixed(self) -> f64 {
        libm::log1p(self)
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

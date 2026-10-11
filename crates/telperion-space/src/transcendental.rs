//! One pinned pure-Rust implementation on native and wasm, including hypot,
//! for the engine and the core alike (fn-11; fn-198 host decision 10): std's
//! transcendentals are the platform's, so a tree grown natively and in Wasm
//! would differ in its last bits. Distinct method names prevent accidental
//! dispatch to f64's or f32's platform libm.
mod hypot;
pub trait Transcendental: Sized {
    fn sin_fixed(self) -> Self;
    fn cos_fixed(self) -> Self;
    fn sin_cos_fixed(self) -> (Self, Self);
    fn tan_fixed(self) -> Self;
    fn asin_fixed(self) -> Self;
    fn acos_fixed(self) -> Self;
    fn atan_fixed(self) -> Self;
    fn atan2_fixed(self, other: Self) -> Self;
    fn exp_fixed(self) -> Self;
    fn exp_m1_fixed(self) -> Self;
    fn ln_fixed(self) -> Self;
    fn ln_1p_fixed(self) -> Self;
    fn log2_fixed(self) -> Self;
    fn exp2_fixed(self) -> Self;
    fn powf_fixed(self, other: Self) -> Self;
    fn cbrt_fixed(self) -> Self;
    fn hypot_fixed(self, other: Self) -> Self;
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
    fn log2_fixed(self) -> f64 {
        libm::log2(self)
    }
    fn exp2_fixed(self) -> f64 {
        libm::exp2(self)
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

/// The same functions in single precision, for growth's f32 arithmetic
/// (fn-254), pinned as the f64 ones are.
impl Transcendental for f32 {
    fn sin_fixed(self) -> f32 {
        libm::sinf(self)
    }
    fn cos_fixed(self) -> f32 {
        libm::cosf(self)
    }
    fn sin_cos_fixed(self) -> (f32, f32) {
        libm::sincosf(self)
    }
    fn tan_fixed(self) -> f32 {
        libm::tanf(self)
    }
    fn asin_fixed(self) -> f32 {
        libm::asinf(self)
    }
    fn acos_fixed(self) -> f32 {
        libm::acosf(self)
    }
    fn atan_fixed(self) -> f32 {
        libm::atanf(self)
    }
    fn atan2_fixed(self, other: f32) -> f32 {
        libm::atan2f(self, other)
    }
    fn exp_fixed(self) -> f32 {
        libm::expf(self)
    }
    fn exp_m1_fixed(self) -> f32 {
        libm::expm1f(self)
    }
    fn ln_fixed(self) -> f32 {
        libm::logf(self)
    }
    fn ln_1p_fixed(self) -> f32 {
        libm::log1pf(self)
    }
    fn log2_fixed(self) -> f32 {
        libm::log2f(self)
    }
    fn exp2_fixed(self) -> f32 {
        libm::exp2f(self)
    }
    fn powf_fixed(self, other: f32) -> f32 {
        libm::powf(self, other)
    }
    fn cbrt_fixed(self) -> f32 {
        libm::cbrtf(self)
    }
    fn hypot_fixed(self, other: f32) -> f32 {
        libm::hypotf(self, other)
    }
}

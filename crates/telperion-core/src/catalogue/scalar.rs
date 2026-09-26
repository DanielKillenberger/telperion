//! A row's value, whatever its type, as the catalogue reads and writes it.
/// The value kinds a row holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Real,
    Count,
    Size,
    Switch,
    OptionalReal,
    OptionalSize,
}
impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Real => "real",
            Self::Count => "count (u32)",
            Self::Size => "count (usize)",
            Self::Switch => "switch",
            Self::OptionalReal => "optional real",
            Self::OptionalSize => "optional count (usize)",
        }
    }
    pub fn whole(self) -> bool {
        matches!(self, Self::Count | Self::Size | Self::OptionalSize)
    }
}

/// One row's value, whatever its type.
pub trait Scalar {
    fn kind(&self) -> Kind;
    /// The value as a real, `None` where an option is unset.
    fn number(&self) -> Option<f64>;
    /// Stores a real the way `as` converts it to the row's type.
    fn put(&mut self, value: f64);
    #[cfg(feature = "json")]
    fn encode(&self) -> serde_json::Value;
    #[cfg(feature = "json")]
    fn decode(&mut self, value: serde_json::Value) -> crate::Result<()>;
}
macro_rules! scalar {
    ($($t:ty => $kind:ident, |$v:ident| $number:expr, |$p:ident| $put:expr;)+) => {$(
        impl Scalar for $t {
            fn kind(&self) -> Kind {
                Kind::$kind
            }
            fn number(&self) -> Option<f64> {
                let $v = *self;
                $number
            }
            fn put(&mut self, $p: f64) {
                *self = $put;
            }
            #[cfg(feature = "json")]
            fn encode(&self) -> serde_json::Value {
                serde_json::json!(self)
            }
            #[cfg(feature = "json")]
            fn decode(&mut self, value: serde_json::Value) -> crate::Result<()> {
                *self = serde_json::from_value(value)
                    .map_err(|_| crate::Error::InvalidInput("parameter type or range"))?;
                Ok(())
            }
        }
    )+};
}
scalar! {
    f64 => Real, |v| Some(v), |p| p;
    u32 => Count, |v| Some(f64::from(v)), |p| p as u32;
    usize => Size, |v| Some(v as f64), |p| p as usize;
    bool => Switch, |v| Some(f64::from(u8::from(v))), |p| p != 0.0;
    Option<f64> => OptionalReal, |v| v, |p| Some(p);
    Option<usize> => OptionalSize, |v| v.map(|n| n as f64), |p| Some(p as usize);
}

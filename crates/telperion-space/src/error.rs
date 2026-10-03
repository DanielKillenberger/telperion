use std::fmt;

/// Why the engine drew no tree. Every refusal names the input it refused;
/// the engine never substitutes a value for one it cannot draw.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An input outside what the engine can draw, by its path in the request
    /// (`states[1].zones[0].lateral[0]`).
    Refused { input: String, reason: &'static str },
    /// The tree outgrew the request's phytomer budget.
    Budget { limit: u32 },
    /// The tree holds no phytomer of any size: its seed bud died before it
    /// grew, or all it grew stands exactly at its draws.
    Collapsed,
    /// A phytomer's tip lies below the ground plane.
    BelowGround { axis: usize, height: f64 },
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn refuse<T>(input: impl Into<String>, reason: &'static str) -> Result<T> {
    Err(Error::Refused {
        input: input.into(),
        reason,
    })
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Refused { input, reason } => write!(f, "{input}: {reason}"),
            Error::Budget { limit } => {
                write!(f, "the tree outgrew its budget of {limit} phytomers")
            }
            Error::Collapsed => write!(f, "the tree collapsed: its seed bud died before it grew"),
            Error::BelowGround { axis, height } => {
                write!(f, "axis {axis} reaches {height:.3} m below the ground")
            }
        }
    }
}

impl std::error::Error for Error {}

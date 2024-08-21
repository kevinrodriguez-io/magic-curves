use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents the possible errors that can occur during decimal operations.
#[derive(Debug)]
pub enum BondingCurveError {
    /// Indicates that an overflow occurred during the operation.
    Overflow,
    /// Indicates that a division by zero occurred during the operation.
    DivisionByZero,
}

impl Display for BondingCurveError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BondingCurveError::Overflow => {
                write!(f, "An overflow occurred during the operation.")
            }
            BondingCurveError::DivisionByZero => {
                write!(f, "A division by zero occurred during the operation.")
            }
        }
    }
}

impl Error for BondingCurveError {}

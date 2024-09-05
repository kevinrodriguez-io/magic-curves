use super::BondingCurveError;

/// Represents the side of an operation in a bonding curve.
///
/// This enum is used to specify whether an operation is adding to or removing from
/// the supply of tokens in a bonding curve system.
///
/// # Variants
///
/// * `Add` - Represents an operation that adds tokens to the supply.
/// * `Remove` - Represents an operation that removes tokens from the supply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationSide {
    Add,
    Remove,
}

pub trait BondingCurve<T> {
    fn calculate_price(&self, supply: u64) -> T;
    fn calculate_price_many(&self, starting_supply: u64, amount: u64, side: OperationSide) -> T;
}

pub trait BondingCurveWithCheckedOperations<T> {
    fn calculate_price_checked(&self, supply: u64) -> Result<T, BondingCurveError>;
    fn calculate_price_many_checked(
        &self,
        starting_supply: u64,
        amount: u64,
        side: OperationSide,
    ) -> Result<T, BondingCurveError>;
}

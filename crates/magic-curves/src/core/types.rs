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

/// Represents a bonding curve for token pricing.
///
/// This trait defines the interface for a bonding curve, which is used to calculate
/// token prices based on the current supply and other parameters.
///
/// # Type Parameters
///
/// * `T` - The type used to represent prices. Currently a numeric type like `u64` or `f64`.
///
/// The bonding curves that are implemented using `u64` use exact formulas with no
/// floating point precission loss. Currently, this crate provides two of them:
///
/// * `LinearBondingCurve` - A linear bonding curve.
/// * `ExponentialBondingCurve` - An exponential bonding curve.
///
/// The bonding curves that are implemented using `f64` use floating point numbers
/// for the calculations, which means that there is a risk of precision loss.
/// Currently, this crate provides one of them:
///
/// * `ExponentialBondingCurve` - An exponential bonding curve.
/// * `LogarithmicBondingCurve` - A logarithmic bonding curve.
/// * `SigmoidBondingCurve` - A sigmoid bonding curve.
pub trait BondingCurve<T> {
    /// Calculates the price for a single token at the given supply.
    ///
    /// # Arguments
    ///
    /// * `supply` - The current total supply of tokens.
    ///
    /// # Returns
    ///
    /// The price of a single token at the given supply.
    fn calculate_price(&self, supply: u64) -> T;

    /// Calculates the total price for a given amount of tokens starting from a specific supply.
    ///
    /// # Arguments
    ///
    /// * `starting_supply` - The initial supply before the operation.
    /// * `amount` - The number of tokens to add or remove.
    /// * `side` - Specifies whether tokens are being added or removed.
    ///
    /// # Returns
    ///
    /// The total price for the specified amount of tokens.
    fn calculate_price_many(&self, starting_supply: u64, amount: u64, side: OperationSide) -> T;
}

/// Represents a bonding curve with checked operations for token pricing.
///
/// This trait extends the `BondingCurve` trait by providing methods that return
/// `Result` types, allowing for error handling in case of overflow or other
/// computational issues.
///
/// # Type Parameters
///
/// * `T` - The type used to represent prices. Typically a numeric type like `u64` or `f64`.
pub trait BondingCurveWithCheckedOperations<T> {
    /// Calculates the price for a single token at the given supply, with error checking.
    ///
    /// # Arguments
    ///
    /// * `supply` - The current total supply of tokens.
    ///
    /// # Returns
    ///
    /// A `Result` containing the price of a single token at the given supply,
    /// or a `BondingCurveError` if the calculation fails.
    fn calculate_price_checked(&self, supply: u64) -> Result<T, BondingCurveError>;

    /// Calculates the total price for a given amount of tokens starting from a specific supply,
    /// with error checking.
    ///
    /// # Arguments
    ///
    /// * `starting_supply` - The initial supply before the operation.
    /// * `amount` - The number of tokens to add or remove.
    /// * `side` - Specifies whether tokens are being added or removed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the total price for the specified amount of tokens,
    /// or a `BondingCurveError` if the calculation fails.
    fn calculate_price_many_checked(
        &self,
        starting_supply: u64,
        amount: u64,
        side: OperationSide,
    ) -> Result<T, BondingCurveError>;
}

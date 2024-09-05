mod utils;

use magic_curves::{
    BondingCurve, ExponentialBondingCurve, LinearBondingCurve, LogarithmicBondingCurve,
    OperationSide, QuadraticBondingCurve, SigmoidBondingCurve,
};
use wasm_bindgen::prelude::*;

/// Represents the side of a bonding curve operation.
#[wasm_bindgen]
pub enum Side {
    Add,
    Remove,
}

impl Side {
    /// Converts the Side enum to the corresponding OperationSide.
    pub fn to_operation_side(&self) -> OperationSide {
        match self {
            Side::Add => OperationSide::Add,
            Side::Remove => OperationSide::Remove,
        }
    }
}

/// Calculates the price for a linear bonding curve at the current supply.
///
/// # Arguments
///
/// * `linear` - The linear coefficient of the curve.
/// * `base` - The base price of the curve.
/// * `current_supply` - The current token supply.
///
/// # Returns
///
/// The calculated price as a u64.
#[wasm_bindgen]
pub fn price_at_current_supply_linear(linear: u64, base: u64, current_supply: u64) -> u64 {
    LinearBondingCurve::new(linear, base).calculate_price(current_supply)
}

/// Calculates the total price for multiple tokens in a linear bonding curve.
///
/// # Arguments
///
/// * `linear` - The linear coefficient of the curve.
/// * `base` - The base price of the curve.
/// * `current_supply` - The current token supply.
/// * `amount` - The number of tokens to calculate the price for.
/// * `side` - The side of the operation (Add or Remove).
///
/// # Returns
///
/// The calculated total price as a u64.
#[wasm_bindgen]
pub fn price_many_at_current_supply_linear(
    linear: u64,
    base: u64,
    current_supply: u64,
    amount: u64,
    side: Side,
) -> u64 {
    LinearBondingCurve::new(linear, base).calculate_price_many(
        current_supply,
        amount,
        side.to_operation_side(),
    )
}

/// Calculates the price for a quadratic bonding curve at the current supply.
///
/// # Arguments
///
/// * `quadratic` - The quadratic coefficient of the curve.
/// * `linear` - The linear coefficient of the curve.
/// * `base` - The base price of the curve.
/// * `current_supply` - The current token supply.
///
/// # Returns
///
/// The calculated price as a u64.
#[wasm_bindgen]
pub fn price_at_current_supply_quadratic(
    quadratic: u64,
    linear: u64,
    base: u64,
    current_supply: u64,
) -> u64 {
    QuadraticBondingCurve::new(quadratic, linear, base).calculate_price(current_supply)
}

/// Calculates the total price for multiple tokens in a quadratic bonding curve.
///
/// # Arguments
///
/// * `quadratic` - The quadratic coefficient of the curve.
/// * `linear` - The linear coefficient of the curve.
/// * `base` - The base price of the curve.
/// * `current_supply` - The current token supply.
/// * `amount` - The number of tokens to calculate the price for.
/// * `side` - The side of the operation (Add or Remove).
///
/// # Returns
///
/// The calculated total price as a u64.
#[wasm_bindgen]
pub fn price_many_at_current_supply_quadratic(
    quadratic: u64,
    linear: u64,
    base: u64,
    current_supply: u64,
    amount: u64,
    side: Side,
) -> u64 {
    QuadraticBondingCurve::new(quadratic, linear, base).calculate_price_many(
        current_supply,
        amount,
        side.to_operation_side(),
    )
}

/// Calculates the price for an exponential bonding curve at the current supply.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `current_supply` - The current token supply.
///
/// # Returns
///
/// The calculated price as an f64.
#[wasm_bindgen]
pub fn price_at_current_supply_exponential(base: f64, growth: f64, current_supply: u64) -> f64 {
    ExponentialBondingCurve::new(base, growth).calculate_price(current_supply)
}

/// Calculates the total price for multiple tokens in an exponential bonding curve.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `current_supply` - The current token supply.
/// * `amount` - The number of tokens to calculate the price for.
/// * `side` - The side of the operation (Add or Remove).
///
/// # Returns
///
/// The calculated total price as an f64.
#[wasm_bindgen]
pub fn price_many_at_current_supply_exponential(
    base: f64,
    growth: f64,
    current_supply: u64,
    amount: u64,
    side: Side,
) -> f64 {
    ExponentialBondingCurve::new(base, growth).calculate_price_many(
        current_supply,
        amount,
        side.to_operation_side(),
    )
}

/// Calculates the price for a logarithmic bonding curve at the current supply.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `current_supply` - The current token supply.
///
/// # Returns
///
/// The calculated price as an f64.
#[wasm_bindgen]
pub fn price_at_current_supply_logarithmic(base: f64, growth: f64, current_supply: u64) -> f64 {
    LogarithmicBondingCurve::new(base, growth).calculate_price(current_supply)
}

/// Calculates the total price for multiple tokens in a logarithmic bonding curve.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `current_supply` - The current token supply.
/// * `amount` - The number of tokens to calculate the price for.
/// * `side` - The side of the operation (Add or Remove).
///
/// # Returns
///
/// The calculated total price as an f64.
#[wasm_bindgen]
pub fn price_many_at_current_supply_logarithmic(
    base: f64,
    growth: f64,
    current_supply: u64,
    amount: u64,
    side: Side,
) -> f64 {
    LogarithmicBondingCurve::new(base, growth).calculate_price_many(
        current_supply,
        amount,
        side.to_operation_side(),
    )
}

/// Calculates the price for a sigmoid bonding curve at the current supply.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `mid_supply` - The supply at the midpoint of the sigmoid curve.
/// * `current_supply` - The current token supply.
///
/// # Returns
///
/// The calculated price as an f64.
#[wasm_bindgen]
pub fn price_at_current_supply_sigmoid(
    base: f64,
    growth: f64,
    mid_supply: u64,
    current_supply: u64,
) -> f64 {
    SigmoidBondingCurve::new(base, growth, mid_supply).calculate_price(current_supply)
}

/// Calculates the total price for multiple tokens in a sigmoid bonding curve.
///
/// # Arguments
///
/// * `base` - The base price of the curve.
/// * `growth` - The growth rate of the curve.
/// * `mid_supply` - The supply at the midpoint of the sigmoid curve.
/// * `current_supply` - The current token supply.
/// * `amount` - The number of tokens to calculate the price for.
/// * `side` - The side of the operation (Add or Remove).
///
/// # Returns
///
/// The calculated total price as an f64.
#[wasm_bindgen]
pub fn price_many_at_current_supply_sigmoid(
    base: f64,
    growth: f64,
    mid_supply: u64,
    current_supply: u64,
    amount: u64,
    side: Side,
) -> f64 {
    SigmoidBondingCurve::new(base, growth, mid_supply).calculate_price_many(
        current_supply,
        amount,
        side.to_operation_side(),
    )
}

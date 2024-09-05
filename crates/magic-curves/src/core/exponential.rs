use std::f64::consts::E;

use super::{BondingCurve, OperationSide};

/// Represents an exponential bonding curve.
///
/// This struct defines an exponential bonding curve with a base price and a growth rate.
///
/// # Fields
///
/// * `base`: The base price, which is the initial price for the first token.
/// * `growth`: The growth rate that determines how quickly the price increases.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExponentialBondingCurve {
    pub base: f64,
    pub growth: f64,
}

impl ExponentialBondingCurve {
    /// Creates a new `ExponentialBondingCurve` with the specified base price and growth rate.
    ///
    /// # Arguments
    ///
    /// * `base` - The base price, which is the initial price for the first token.
    /// * `growth` - The growth rate that determines how quickly the price increases.
    ///
    /// # Returns
    ///
    /// A new instance of `ExponentialBondingCurve`.
    ///
    /// # Example
    ///
    /// ```
    /// use magic_curves::ExponentialBondingCurve;
    ///
    /// let curve = ExponentialBondingCurve::new(0.01, 0.02);
    /// ```
    pub fn new(base: f64, growth: f64) -> Self {
        Self { base, growth }
    }
}

impl BondingCurve<f64> for ExponentialBondingCurve {
    /// Calculates the price based on the supply.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// f(x) = base * e^(growth * x)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `supply` - The current supply of tokens.
    ///
    /// # Returns
    ///
    /// The price of the token based on the supply.
    fn calculate_price(&self, supply: u64) -> f64 {
        self.base * E.powf(self.growth * supply as f64)
    }

    /// Calculates the price for a given amount of tokens.
    ///
    /// # Formula
    ///
    /// The integral of the exponential function is used:
    /// ```ignore
    /// F(x) = (base / growth) * (e^(growth * x) - e^(growth * start))
    /// ```
    ///
    /// # Arguments
    ///
    /// * `starting_supply` - The current supply of tokens.
    /// * `amount` - The amount of tokens to calculate the price for.
    /// * `side` - The side of the operation (add or remove).
    ///
    /// # Returns
    ///
    /// The total price for the given amount of tokens.
    fn calculate_price_many(&self, starting_supply: u64, amount: u64, side: OperationSide) -> f64 {
        let start = starting_supply as f64;
        let end = match side {
            OperationSide::Add => (starting_supply + amount) as f64,
            OperationSide::Remove => (starting_supply - amount) as f64,
        };
        // Calculate the integral of the exponential function
        let integral =
            self.base / self.growth * (E.powf(self.growth * end) - E.powf(self.growth * start));
        match side {
            OperationSide::Add => integral,
            OperationSide::Remove => -integral,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        fixed_point_to_float, float_to_fixed_point, BondingCurve, ExponentialBondingCurve,
        OperationSide,
    };

    #[test]
    pub fn test_exponential_price_calculus() {
        let curve = ExponentialBondingCurve::new(0.01, 0.02);
        let price = curve.calculate_price(100);
        assert_eq!(price, 0.073890560989306492);
    }

    #[test]
    pub fn test_exponential_price_calculus_fixed_point() {
        let base = fixed_point_to_float(1, 2);
        let growth = fixed_point_to_float(2, 2);
        let curve = ExponentialBondingCurve::new(base, growth);
        let price = curve.calculate_price(100);
        assert_eq!(float_to_fixed_point(price, 9), 0_073_890_560);
    }

    #[test]
    pub fn test_exponential_price_calculus_many() {
        let amount = 10;
        let starting_supply = 1000;
        let curve = ExponentialBondingCurve::new(0.05, 0.01);
        let add_price_many =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Add);
        assert_eq!(add_price_many, 11582.718148008316);
        let remove_price_many =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Remove);
        assert_eq!(remove_price_many, 10480.476782882088);
    }
}

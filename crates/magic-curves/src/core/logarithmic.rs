use super::{BondingCurve, OperationSide};

/// Represents a logarithmic bonding curve.
///
/// This struct defines a logarithmic bonding curve with a base price and a growth rate.
///
/// # Fields
///
/// * `base`: The base price, which is the initial price for the first token.
/// * `growth`: The growth rate that determines how quickly the price increases.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogarithmicBondingCurve {
    pub base: f64,
    pub growth: f64,
}

impl LogarithmicBondingCurve {
    /// Creates a new `LogarithmicBondingCurve` with the specified base price and growth rate.
    ///
    /// # Arguments
    ///
    /// * `base` - The base price, which is the initial price for the first token.
    /// * `growth` - The growth rate that determines how quickly the price increases.
    ///
    /// # Returns
    ///
    /// A new instance of `LogarithmicBondingCurve`.
    ///
    /// # Example
    ///
    /// ```
    /// use magic_curves::LogarithmicBondingCurve;
    ///
    /// let curve = LogarithmicBondingCurve::new(0.02, 0.01);
    /// ```
    pub fn new(base: f64, growth: f64) -> Self {
        Self { base, growth }
    }
}

impl BondingCurve<f64> for LogarithmicBondingCurve {
    /// Calculates the price based on the supply.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// f(x) = growth * ln(x) + base
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
        if supply == 0 {
            return self.base; // Avoid taking the log of 0
        }
        self.growth * (supply as f64).ln() + self.base
    }

    /// Calculates the price for a given amount of tokens.
    ///
    /// # Formula
    ///
    /// The integral of the logarithmic function is used:
    /// ```ignore
    /// F(x) = growth * x * ln(x) - growth * x + base * x
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

        // Calculate the integral of the logarithmic function
        let integral = |x: f64| self.growth * x * x.ln() - self.growth * x + self.base * x;

        // Calculate the difference between the integrals at the end and start points
        let price = match side {
            OperationSide::Add => integral(end) - integral(start),
            OperationSide::Remove => integral(start) - integral(end),
        };

        // Handle the case where starting_supply is 0 for Add operation
        if starting_supply == 0 && side == OperationSide::Add {
            price + self.base // Add base price for the first token
        } else {
            price
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        fixed_point_to_float, float_to_fixed_point, BondingCurve, LogarithmicBondingCurve,
        OperationSide,
    };

    #[test]
    pub fn test_logarithmic_price_calculus() {
        let curve = LogarithmicBondingCurve::new(0.02, 0.01);
        let price = curve.calculate_price(100);
        assert_eq!(price, 0.06605170185988092);
    }

    #[test]
    pub fn test_logarithmic_price_calculus_fixed_point() {
        let base = fixed_point_to_float(2, 2);
        let growth = fixed_point_to_float(1, 2);
        let curve = LogarithmicBondingCurve::new(base, growth);
        let price = curve.calculate_price(100);
        assert_eq!(float_to_fixed_point(price, 9), 0_066_051_701);
    }

    #[test]
    pub fn test_logarithmic_price_calculus_many() {
        let curve = LogarithmicBondingCurve::new(0.02, 0.01);
        let price_add = curve.calculate_price_many(100, 10, OperationSide::Add);
        assert_eq!(price_add, 0.6653582163835674);
        let price_remove = curve.calculate_price_many(100, 10, OperationSide::Remove);
        assert_eq!(price_remove, 0.6553414826908526);
    }
}

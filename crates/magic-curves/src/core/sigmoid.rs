use super::{BondingCurve, OperationSide};

/// Represents a sigmoid bonding curve.
///
/// This struct defines a sigmoid bonding curve with a maximum price, growth rate, and mid-supply point.
///
/// # Fields
///
/// * `max_price`: The maximum price that the curve approaches asymptotically.
/// * `growth`: The growth rate that determines how quickly the price increases.
/// * `mid_supply`: The supply at which the price is half of the maximum price.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SigmoidBondingCurve {
    pub max_price: f64,
    pub growth: f64,
    pub mid_supply: u64,
}

impl SigmoidBondingCurve {
    /// Creates a new `SigmoidBondingCurve` with the specified maximum price, growth rate, and mid-supply point.
    ///
    /// # Arguments
    ///
    /// * `max_price` - The maximum price that the curve approaches asymptotically.
    /// * `growth` - The growth rate that determines how quickly the price increases.
    /// * `mid_supply` - The supply at which the price is half of the maximum price.
    ///
    /// # Returns
    ///
    /// A new instance of `SigmoidBondingCurve`.
    ///
    /// # Example
    ///
    /// ```
    /// use magic_curves::SigmoidBondingCurve;
    ///
    /// let curve = SigmoidBondingCurve::new(100.0, 0.01, 500);
    /// ```
    pub fn new(max_price: f64, growth: f64, mid_supply: u64) -> Self {
        Self {
            max_price,
            growth,
            mid_supply,
        }
    }
}

impl BondingCurve<f64> for SigmoidBondingCurve {
    /// Calculates the price based on the supply.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// f(x) = max_price / (1 + e^(-growth * (x - mid_supply)))
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
        let s = supply as f64;
        self.max_price / (1.0 + (-self.growth * (s - self.mid_supply as f64)).exp())
    }

    /// Calculates the price for a given amount of tokens.
    ///
    /// # Formula
    ///
    /// The integral of the sigmoid function is used:
    /// ```ignore
    /// F(x) = (max_price / growth) * ln(1 + e^(growth * (x - mid_supply)))
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
        let s = starting_supply as f64;
        let n = amount as f64;
        let mid_supply = self.mid_supply as f64;
        let growth = self.growth;
        let max_price = self.max_price;

        // Get the bounds based on whether it's an Add or Remove operation
        let (start_supply, end_supply) = match side {
            OperationSide::Add => (s, s + n),    // Buying (adding supply)
            OperationSide::Remove => (s - n, s), // Selling (removing supply)
        };

        // Perform the integral of the sigmoid function over the range
        let price_at_end_supply = (1.0 + (growth * (end_supply - mid_supply)).exp()).ln();
        let price_at_start_supply = (1.0 + (growth * (start_supply - mid_supply)).exp()).ln();

        // Total price is the difference in the integral values
        (max_price / growth) * (price_at_end_supply - price_at_start_supply)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        fixed_point_to_float, float_to_fixed_point, BondingCurve, OperationSide,
        SigmoidBondingCurve,
    };

    #[test]
    pub fn test_sigmoid_price_calculus() {
        let curve = SigmoidBondingCurve::new(100.0, 0.01, 500);
        let price = curve.calculate_price(480);
        assert_eq!(price, 45.016600268752214);
    }

    #[test]
    pub fn test_sigmoid_price_calculus_fixed_point() {
        let curve = crate::SigmoidBondingCurve::new(
            fixed_point_to_float(100, 0),
            fixed_point_to_float(1, 2),
            500,
        );
        let price = curve.calculate_price(480);
        assert_eq!(float_to_fixed_point(price, 9), 45_016_600_268);
    }

    #[test]
    pub fn test_sigmoid_price_calculus_many() {
        let curve = SigmoidBondingCurve::new(100.0, 0.01, 500);
        let many_price_add = curve.calculate_price_many(480, 10, OperationSide::Add);
        assert_eq!(many_price_add, 462.5779069197911, "Add price is wrong");
        let many_price_remove = curve.calculate_price_many(480, 10, OperationSide::Remove);
        assert_eq!(
            many_price_remove, 437.83624913064756,
            "Remove price is wrong"
        );
    }
}

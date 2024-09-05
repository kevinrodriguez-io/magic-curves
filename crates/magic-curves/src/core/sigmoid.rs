use super::{BondingCurve, OperationSide};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SigmoidBondingCurve {
    pub max_price: f64,
    pub growth: f64,
    pub mid_supply: u64,
}

impl SigmoidBondingCurve {
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
    /// # Formula:
    ///
    /// ```ignore
    /// f(x) =  L
    ///       ------
    ///    1+e^(-k[x-x0])
    /// ```
    ///
    /// where:
    ///
    /// - x is the supply.
    /// - L is the maximum price (upper asymptote of the curve).
    /// - k is the growth factor.
    /// - x0 is the inflection point. (Can be: Max supply / 2)
    fn calculate_price(&self, supply: u64) -> f64 {
        let s = supply as f64;
        self.max_price / (1.0 + (-self.growth * (s - self.mid_supply as f64)).exp())
    }

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

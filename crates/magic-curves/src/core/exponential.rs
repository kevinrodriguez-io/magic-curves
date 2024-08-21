use std::f64::consts::E;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExponentialBondingCurve {
    pub base: f64,
    pub growth: f64,
}

impl ExponentialBondingCurve {
    pub fn new(base: f64, growth: f64) -> Self {
        Self { base, growth }
    }

    /// Calculates the price based on the supply.
    ///
    /// This function is lossy because it deals with floating point numbers.
    /// In theory, the price should be calculated using fixed point arithmetic,
    /// however, using a Taylor series expansion, we can calculate the price
    /// without using floating point numbers, but it is more complex, and
    /// precision is lost due to type conversions and large exponentiation.
    ///
    /// # Formula:
    ///
    /// `f(x) = a*e^(b*x)`
    ///
    /// where:
    ///
    /// - e is euler's number.
    /// - x is the supply.
    /// - a is a constant that scales the price, often called the base price.
    /// - b is the growth rate of the curve, which determines how quickly the price increases as the supply increases.
    pub fn calculate_price_lossy(&self, supply: u64) -> f64 {
        self.base * E.powf(self.growth * supply as f64)
    }
}

#[cfg(test)]
mod test {
    use crate::{fixed_point_to_float, float_to_fixed_point};

    #[test]
    pub fn test_exponential_price_calculus() {
        let curve = crate::ExponentialBondingCurve::new(0.01, 0.02);
        let price = curve.calculate_price_lossy(100);
        assert_eq!(price, 0.073890560989306492);
    }

    #[test]
    pub fn test_exponential_price_calculus_fixed_point() {
        let base = fixed_point_to_float(1, 2);
        let growth = fixed_point_to_float(2, 2);
        let curve = crate::ExponentialBondingCurve::new(base, growth);
        let price = curve.calculate_price_lossy(100);
        assert_eq!(float_to_fixed_point(price, 9), 0_073_890_560);
    }
}

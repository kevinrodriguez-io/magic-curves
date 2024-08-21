#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogarithmicBondingCurve {
    pub base: f64,
    pub growth: f64,
}

impl LogarithmicBondingCurve {
    pub fn new(base: f64, growth: f64) -> Self {
        Self { base, growth }
    }

    /// Calculates the price based on the supply.
    ///
    /// # Formula:
    ///
    /// `f(x) = a*ln(x)+b`
    ///
    /// where:
    ///
    /// - x is the supply
    /// - a is the growth rate of the curve, which determines how quickly the price increases as the supply increases.
    /// - b is a constant that scales the price, often called the base price.
    pub fn calculate_price_lossy(&self, supply: u64) -> f64 {
        if supply == 0 {
            return self.base; // Avoid taking the log of 0
        }
        self.growth * (supply as f64).ln() + self.base
    }
}

#[cfg(test)]
mod test {
    use crate::{fixed_point_to_float, float_to_fixed_point};

    #[test]
    pub fn test_logarithmic_price_calculus() {
        let curve = crate::LogarithmicBondingCurve::new(0.02, 0.01);
        let price = curve.calculate_price_lossy(100);
        assert_eq!(price, 0.06605170185988092);
    }

    #[test]
    pub fn test_logarithmic_price_calculus_fixed_point() {
        let base = fixed_point_to_float(1, 2);
        let growth = fixed_point_to_float(2, 2);
        let curve = crate::LogarithmicBondingCurve::new(base, growth);
        let price = curve.calculate_price_lossy(100);
        assert_eq!(float_to_fixed_point(price, 9), 0_066_051_701);
    }
}

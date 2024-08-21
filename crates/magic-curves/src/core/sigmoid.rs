use std::f64::consts::E;

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

    /// Calculates the price based on the supply.
    ///
    /// # Formula:
    ///
    /// ```no_run
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
    pub fn calculate_price_lossy(&self, supply: u64) -> f64 {
        let s = supply as f64;
        self.max_price / (1.0 + (-self.growth * (s - self.mid_supply as f64)).exp())
    }
}

#[cfg(test)]
mod test {
    use crate::{fixed_point_to_float, float_to_fixed_point};

    #[test]
    pub fn test_sigmoid_price_calculus() {
        let curve = crate::SigmoidBondingCurve::new(100.0, 0.01, 500);
        let price = curve.calculate_price_lossy(100);
        assert_eq!(price, 45.016600268752214);
    }

    #[test]
    pub fn test_sigmoid_price_calculus_fixed_point() {
        let curve = crate::SigmoidBondingCurve::new(
            fixed_point_to_float(100, 0),
            fixed_point_to_float(1, 2),
            500,
        );
        let price = curve.calculate_price_lossy(100);
        assert_eq!(float_to_fixed_point(price, 9), 4_501_660_026);
    }
}

use std::f64::consts::E;

use super::{BondingCurve, OperationSide};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExponentialBondingCurve {
    pub base: f64,
    pub growth: f64,
}

impl ExponentialBondingCurve {
    pub fn new(base: f64, growth: f64) -> Self {
        Self { base, growth }
    }
}

impl BondingCurve<f64> for ExponentialBondingCurve {
    fn calculate_price(&self, supply: u64) -> f64 {
        self.base * E.powf(self.growth * supply as f64)
    }
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

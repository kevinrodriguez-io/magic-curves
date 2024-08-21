use super::BondingCurveError;
use num_traits::{CheckedAdd, CheckedMul, Zero};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearBondingCurve<T> {
    pub linear: T,
    pub base: T,
}

impl<T> LinearBondingCurve<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + CheckedMul + CheckedAdd + Zero,
{
    pub fn new(linear: T, base: T) -> Self {
        Self { linear, base }
    }

    /// Calculates the price based on the supply.
    ///
    /// # Formula:
    ///
    /// f(x) = ax + b
    ///
    /// where:
    ///
    /// - x is the supply
    /// - a is the linear coefficient
    /// - b is the base price
    pub fn calculate_price_checked(&self, supply: T) -> Result<T, BondingCurveError> {
        let result = self
            .linear
            .checked_mul(&supply)
            .and_then(|x| x.checked_add(&self.base));

        result.ok_or(BondingCurveError::Overflow)
    }

    pub fn calculate_price(&self, supply: T) -> T {
        self.linear * supply + self.base
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_linear_price_calculus() {
        let linear = 500_000_000u128;
        let base = 1_000_000_000u128;

        let curve = crate::LinearBondingCurve::new(linear, base);
        let r1 = curve.base;
        let r2 = 1_500_000_000u128;
        let r3 = 5_000_000_000u128;
        let r4 = 401_000_000_000u128;

        let price = curve.calculate_price(0);
        assert_eq!(price, r1);
        let price = curve.calculate_price_checked(0).unwrap();
        assert_eq!(price, r1);

        let price = curve.calculate_price(1);
        assert_eq!(price, r2);
        let price = curve.calculate_price_checked(1).unwrap();
        assert_eq!(price, r2);

        let price = curve.calculate_price(8);
        assert_eq!(price, r3);
        let price = curve.calculate_price_checked(8).unwrap();
        assert_eq!(price, r3);

        let price = curve.calculate_price(800);
        assert_eq!(price, r4);
        let price = curve.calculate_price_checked(800).unwrap();
        assert_eq!(price, r4);
    }
}

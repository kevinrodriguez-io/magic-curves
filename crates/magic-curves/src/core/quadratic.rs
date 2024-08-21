use super::BondingCurveError;
use num_traits::{CheckedAdd, CheckedMul, Zero};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuadraticBondingCurve<T> {
    pub quadratic: T,
    pub linear: T,
    pub base: T,
}

impl<T> QuadraticBondingCurve<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + CheckedMul + CheckedAdd + Zero,
{
    pub fn new(quadratic: T, linear: T, base: T) -> Self {
        Self {
            quadratic,
            linear,
            base,
        }
    }

    /// Calculates the price based on the supply.
    ///
    /// # Formula:
    ///
    /// f(x) = xa^2 + bx + c
    ///
    /// where:
    ///
    /// - x is the supply
    /// - a is the quadratic coefficient
    /// - b is the linear coefficient
    /// - c is the base price
    pub fn calculate_price_checked(&self, supply: T) -> Result<T, BondingCurveError> {
        let result = self
            .quadratic
            .checked_mul(&supply)
            .and_then(|x| x.checked_mul(&supply))
            .and_then(|x| x.checked_add(&self.linear.checked_mul(&supply)?))
            .and_then(|x| x.checked_add(&self.base));

        result.ok_or(BondingCurveError::Overflow)
    }

    pub fn calculate_price(&self, supply: T) -> T {
        self.quadratic * supply * supply + self.linear * supply + self.base
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_quadratic_price_calculus() {
        let curve = crate::QuadraticBondingCurve::new(10_000_000, 500_000_000, 1_000_000_000);

        let r1 = curve.base;
        let r2 = 1_510_000_000u64;
        let r3 = 5_640_000_000u64;
        let r4 = 6_801_000_000_000u64;

        // Minimum cost is base.
        let price = curve.calculate_price_checked(0).unwrap();
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

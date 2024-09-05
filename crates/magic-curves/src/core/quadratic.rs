use super::{BondingCurve, BondingCurveError, BondingCurveWithCheckedOperations, OperationSide};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuadraticBondingCurve {
    pub quadratic: u64,
    pub linear: u64,
    pub base: u64,
}

impl QuadraticBondingCurve {
    pub fn new(quadratic: u64, linear: u64, base: u64) -> Self {
        Self {
            quadratic,
            linear,
            base,
        }
    }
}

impl BondingCurve<u64> for QuadraticBondingCurve {
    fn calculate_price(&self, supply: u64) -> u64 {
        self.quadratic * supply * supply + self.linear * supply + self.base
    }

    fn calculate_price_many(&self, starting_supply: u64, amount: u64, side: OperationSide) -> u64 {
        let n = amount;
        let a = starting_supply;

        let sum_quadratic = match side {
            OperationSide::Add => {
                // Sum of quadratic terms: (a^2 * n) + (a * n * (n-1)) + (n * (n-1) * (2n-1) / 6)
                (self.quadratic * a * a * n)
                    + (self.quadratic * a * n * (n - 1))
                    + (self.quadratic * n * (n - 1) * (2 * n - 1) / 6)
            }
            OperationSide::Remove => {
                // Sum of quadratic terms: (a^2 * n) - (a * n * (n-1)) + (n * (n-1) * (2n-1) / 6)
                (self.quadratic * a * a * n) - (self.quadratic * a * n * (n - 1))
                    + (self.quadratic * n * (n - 1) * (2 * n - 1) / 6)
            }
        };

        let sum_linear = match side {
            OperationSide::Add => {
                // Sum of linear terms: b * (a * n + n * (n-1) / 2)
                self.linear * (a * n + n * (n - 1) / 2)
            }
            OperationSide::Remove => {
                // Sum of linear terms: b * (a * n - n * (n-1) / 2)
                self.linear * (a * n - n * (n - 1) / 2)
            }
        };

        // Sum of constant terms: c * n
        let sum_constant = self.base * n;

        sum_quadratic + sum_linear + sum_constant
    }
}

impl BondingCurveWithCheckedOperations<u64> for QuadraticBondingCurve {
    fn calculate_price_checked(&self, supply: u64) -> Result<u64, BondingCurveError> {
        let result = self
            .quadratic
            .checked_mul(supply)
            .and_then(|x| x.checked_mul(supply))
            .and_then(|x| x.checked_add(self.linear.checked_mul(supply)?))
            .and_then(|x| x.checked_add(self.base));
        result.ok_or(BondingCurveError::Overflow)
    }

    fn calculate_price_many_checked(
        &self,
        starting_supply: u64,
        amount: u64,
        side: OperationSide,
    ) -> Result<u64, BondingCurveError> {
        let n = amount;
        let a = starting_supply;
        let n_minus_1 = n.checked_sub(1).ok_or(BondingCurveError::Overflow)?;

        // Sum of quadratic terms
        let first_term = self
            .quadratic
            .checked_mul(
                a.checked_mul(a)
                    .and_then(|x| x.checked_mul(n))
                    .ok_or(BondingCurveError::Overflow)?,
            )
            .ok_or(BondingCurveError::Overflow)?;

        let second_term = self
            .quadratic
            .checked_mul(
                a.checked_mul(n)
                    .and_then(|x| x.checked_mul(n_minus_1))
                    .ok_or(BondingCurveError::Overflow)?,
            )
            .ok_or(BondingCurveError::Overflow)?;

        let third_term_pow = 2u64
            .checked_mul(n)
            .and_then(|x| x.checked_sub(1))
            .ok_or(BondingCurveError::Overflow)?;

        let third_term = self
            .quadratic
            .checked_mul(
                n.checked_mul(n_minus_1)
                    .and_then(|x| x.checked_mul(third_term_pow))
                    .and_then(|x| x.checked_div(6))
                    .ok_or(BondingCurveError::Overflow)?,
            )
            .ok_or(BondingCurveError::Overflow)?;

        let sum_quadratic = match side {
            OperationSide::Add => first_term
                .checked_add(second_term)
                .and_then(|x| x.checked_add(third_term)),
            OperationSide::Remove => first_term
                .checked_sub(second_term)
                .and_then(|x| x.checked_add(third_term)),
        }
        .ok_or(BondingCurveError::Overflow)?;

        // Sum of linear terms
        let sum_linear = match side {
            OperationSide::Add => self
                .linear
                .checked_mul(
                    a.checked_mul(n)
                        .and_then(|x| x.checked_add(n.checked_mul(n_minus_1)?.checked_div(2)?))
                        .ok_or(BondingCurveError::Overflow)?,
                )
                .ok_or(BondingCurveError::Overflow)?,
            OperationSide::Remove => self
                .linear
                .checked_mul(
                    a.checked_mul(n)
                        .and_then(|x| x.checked_sub(n.checked_mul(n_minus_1)?.checked_div(2)?))
                        .ok_or(BondingCurveError::Overflow)?,
                )
                .ok_or(BondingCurveError::Overflow)?,
        };

        // Sum of constant terms
        let sum_constant = self
            .base
            .checked_mul(n)
            .ok_or(BondingCurveError::Overflow)?;

        // Final sum
        sum_quadratic
            .checked_add(sum_linear)
            .and_then(|x| x.checked_add(sum_constant))
            .ok_or(BondingCurveError::Overflow)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        BondingCurve, BondingCurveWithCheckedOperations, OperationSide, QuadraticBondingCurve,
    };

    #[test]
    pub fn test_quadratic_price_calculus() {
        let curve = QuadraticBondingCurve::new(10_000_000, 500_000_000, 1_000_000_000);

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

    #[test]
    pub fn test_quadratic_price_many_calculus() {
        let quadratic = 10_000_000u64;
        let linear = 500_000_000u64;
        let base = 1_000_000_000u64;
        let amount = 10u64;
        let starting_supply = 100u64;

        let curve = QuadraticBondingCurve::new(quadratic, linear, base);

        let many_price_add =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Add);
        // Do it with a loop with calculate_price
        let mut looped_price_add = 0u64;
        for i in 0..amount {
            looped_price_add += curve.calculate_price(starting_supply + i);
        }
        assert_eq!(looped_price_add, many_price_add);
        let checked_many_price_add = curve
            .calculate_price_many_checked(starting_supply, amount, OperationSide::Add)
            .unwrap();
        assert_eq!(checked_many_price_add, many_price_add);

        let many_price_remove =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Remove);
        // Do it with a loop with calculate_price
        let mut looped_price_remove = 0u64;
        for i in 0..amount {
            looped_price_remove += curve.calculate_price(starting_supply - i);
        }
        assert_eq!(looped_price_remove, many_price_remove);
        let checked_many_price_remove = curve
            .calculate_price_many_checked(starting_supply, amount, OperationSide::Remove)
            .unwrap();
        assert_eq!(checked_many_price_remove, many_price_remove);
    }
}

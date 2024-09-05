use super::{BondingCurve, BondingCurveError, BondingCurveWithCheckedOperations, OperationSide};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearBondingCurve {
    pub linear: u64,
    pub base: u64,
}

impl LinearBondingCurve {
    pub fn new(linear: u64, base: u64) -> Self {
        Self { linear, base }
    }
}

impl BondingCurve<u64> for LinearBondingCurve {
    fn calculate_price(&self, supply: u64) -> u64 {
        self.linear * supply + self.base
    }
    fn calculate_price_many(&self, starting_supply: u64, amount: u64, side: OperationSide) -> u64 {
        let a1 = self.linear * starting_supply + self.base;
        let an = match side {
            OperationSide::Add => self.linear * (starting_supply + amount - 1) + self.base,
            OperationSide::Remove => self.linear * (starting_supply - amount + 1) + self.base,
        };
        (amount * (a1 + an)) / 2
    }
}

impl BondingCurveWithCheckedOperations<u64> for LinearBondingCurve {
    fn calculate_price_checked(&self, supply: u64) -> Result<u64, BondingCurveError> {
        let result = self
            .linear
            .checked_mul(supply)
            .and_then(|x| x.checked_add(self.base));

        result.ok_or(BondingCurveError::Overflow)
    }

    fn calculate_price_many_checked(
        &self,
        starting_supply: u64,
        amount: u64,
        side: OperationSide,
    ) -> Result<u64, BondingCurveError> {
        let a1 = self
            .linear
            .checked_mul(starting_supply)
            .and_then(|x| x.checked_add(self.base))
            .ok_or(BondingCurveError::Overflow)?;

        let an = match side {
            OperationSide::Add => self
                .linear
                .checked_mul(starting_supply + amount - 1)
                .and_then(|x| x.checked_add(self.base))
                .ok_or(BondingCurveError::Overflow)?,
            OperationSide::Remove => self
                .linear
                .checked_mul(starting_supply - amount + 1)
                .and_then(|x| x.checked_add(self.base))
                .ok_or(BondingCurveError::Overflow)?,
        };

        let sum = a1
            .checked_add(an)
            .and_then(|x| x.checked_mul(amount))
            .and_then(|x| x.checked_div(2))
            .ok_or(BondingCurveError::Overflow)?;
        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        BondingCurve, BondingCurveWithCheckedOperations, LinearBondingCurve, OperationSide,
    };

    #[test]
    pub fn test_linear_price_calculus() {
        let linear = 500_000_000u64;
        let base = 1_000_000_000u64;

        let curve = LinearBondingCurve::new(linear, base);
        let r1 = curve.base;
        let r2 = 1_500_000_000u64;
        let r3 = 5_000_000_000u64;
        let r4 = 401_000_000_000u64;

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

    #[test]
    pub fn test_increase_linear_price_many() {
        let linear = 500_000_000u64;
        let base = 1_000_000_000u64;
        let amount = 10u64;
        let starting_supply = 100u64;

        let curve = LinearBondingCurve::new(linear, base);

        let many_price_add =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Add);

        // Do it with a loop with calculate_price
        let mut looped_price_add = 0u64;
        for i in 0..amount {
            looped_price_add += curve.calculate_price(starting_supply + i);
        }
        assert_eq!(many_price_add, looped_price_add);

        let checked_many_price_add = curve
            .calculate_price_many_checked(starting_supply, amount, OperationSide::Add)
            .unwrap();

        assert_eq!(checked_many_price_add, looped_price_add);

        let many_price_remove =
            curve.calculate_price_many(starting_supply, amount, OperationSide::Remove);

        // Do it with a loop with calculate_price
        let mut looped_price_remove = 0u64;
        for i in 0..amount {
            looped_price_remove += curve.calculate_price(starting_supply - i);
        }

        assert_eq!(many_price_remove, looped_price_remove);

        let checked_many_price_remove = curve
            .calculate_price_many_checked(starting_supply, amount, OperationSide::Remove)
            .unwrap();

        assert_eq!(checked_many_price_remove, looped_price_remove);
    }
}

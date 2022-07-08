use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Provider,
    User,
}

// FIXME: Handle overflow in decimals
#[zero_copy]
#[derive(PartialEq, Default, Debug, AnchorDeserialize, AnchorSerialize)]
pub struct Decimal {
    pub val: u128,
    pub decimals: u8,
}

impl Decimal {
    pub fn new(value: u128, decimals: u8) -> Self {
        Self {
            val: value,
            decimals,
        }
    }
    pub fn denominator(self) -> u128 {
        10u128.pow(self.decimals.into())
    }

    pub fn to_decimals(self, decimals: u8) -> Self {
        Self {
            val: if self.decimals > decimals {
                self.val
                    .checked_div(10u128.pow((self.decimals - decimals).into()))
                    .unwrap()
            } else {
                self.val
                    .checked_mul(10u128.pow((decimals - self.decimals).into()))
                    .unwrap()
            },
            decimals,
        }
    }
    fn to_equal_decimals(one: Decimal, two: Decimal) -> (Decimal, Decimal) {
        let mut one_decimalsd = one;
        let mut two_decimalsd = two;
        if one.decimals > two.decimals {
            two_decimalsd = two.to_decimals(one.decimals);
        } else if one.decimals < two.decimals {
            one_decimalsd = one.to_decimals(two.decimals);
        };
        (one_decimalsd, two_decimalsd)
    }
}

impl Mul<Decimal> for Decimal {
    fn mul(self, other: Decimal) -> Self {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        Self {
            val: self_decimalsd.val.checked_mul(other_decimalsd.val).unwrap(),
            decimals: self_decimalsd.decimals,
        }
    }
}

impl Add<Decimal> for Decimal {
    fn add(self, other: Decimal) -> Self {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        Self {
            val: self_decimalsd.val.checked_add(other_decimalsd.val).unwrap(),
            decimals: self_decimalsd.decimals,
        }
    }
}
impl Sub<Decimal> for Decimal {
    fn sub(self, other: Decimal) -> Self {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        Self {
            val: self_decimalsd.val.checked_sub(other_decimalsd.val).unwrap(),
            decimals: self_decimalsd.decimals,
        }
    }
}
impl Div<Decimal> for Decimal {
    fn div(self, other: Decimal) -> Self {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        Self {
            val: self_decimalsd
                .val
                .checked_mul(other_decimalsd.denominator())
                .unwrap()
                .checked_div(other_decimalsd.val)
                .unwrap(),
            decimals: self_decimalsd.decimals,
        }
    }
}

impl Compare<Decimal> for Decimal {
    fn lte(self, other: Decimal) -> bool {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        self_decimalsd.val <= other_decimalsd.val
    }
    fn lt(self, other: Decimal) -> bool {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        self_decimalsd.val < other_decimalsd.val
    }
    fn gt(self, other: Decimal) -> bool {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        self_decimalsd.val > other_decimalsd.val
    }

    fn gte(self, other: Decimal) -> bool {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        self_decimalsd.val >= other_decimalsd.val
    }
    fn eq(self, other: Decimal) -> bool {
        let (self_decimalsd, other_decimalsd) = Self::to_equal_decimals(self, other);

        self_decimalsd == other_decimalsd
    }
}
pub trait Sub<T>: Sized {
    fn sub(self, rhs: T) -> Self;
}
pub trait Add<T>: Sized {
    fn add(self, rhs: T) -> Self;
}
pub trait Div<T>: Sized {
    fn div(self, rhs: T) -> Self;
}

pub trait Mul<T>: Sized {
    fn mul(self, rhs: T) -> Self;
}

pub trait PowAccuracy<T>: Sized {
    fn pow_with_accuracy(self, rhs: T) -> Self;
}
pub trait Compare<T>: Sized {
    fn eq(self, rhs: T) -> bool;
    fn lt(self, rhs: T) -> bool;
    fn gt(self, rhs: T) -> bool;
    fn gte(self, rhs: T) -> bool;
    fn lte(self, rhs: T) -> bool;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_mul_decimal() {
        let decimal = Decimal::new(1234, 3);
        let multiply_by = Decimal::new(4321, 5);
        let actual = decimal.mul(multiply_by);
        let expected = Decimal::new(533211400, 5);

        assert_eq!({ actual.val }, { expected.val });
        assert_eq!(actual.decimals, expected.decimals);
    }

    #[test]
    #[should_panic]
    fn test_mul_decimal_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 3);
        let multiply_by = Decimal::new(2, 3);
        decimal.mul(multiply_by);
    }

    #[test]
    fn test_add() {
        {
            let decimal = Decimal::new(1337, 6);
            let increase_by = Decimal::new(555, 2);
            let actual = decimal.add(increase_by);
            let expected = Decimal::new(5551337, 6);
            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }

        {
            let decimal = Decimal::new(1337, 6);
            let increase_by = Decimal::new(555, 6);
            let actual = decimal.add(increase_by);
            let expected = Decimal::new(1892, 6);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 2);
        let increase_by = Decimal::new(2, 2);
        decimal.add(increase_by);
    }

    #[test]
    fn test_sub() {
        {
            let decimal = Decimal::new(555, 2);
            let decrease_by = Decimal::new(1337, 6);
            let actual = decimal.sub(decrease_by);
            let expected = Decimal::new(5548663, 6);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }

        {
            let decimal = Decimal::new(1337, 6);
            let decrease_by = Decimal::new(555, 6);
            let actual = decimal.sub(decrease_by);
            let expected = Decimal::new(782, 6);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_panic() {
        let decimal = Decimal::new(1337, 6);
        let decrease_by = Decimal::new(555, 2);
        decimal.sub(decrease_by);
    }

    #[test]
    fn test_div() {
        {
            let decimal = Decimal::new(20, 8);
            let divide_by = Decimal::new(2, 3);
            let actual = decimal.div(divide_by);
            let expected = Decimal::new(10000, 8);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }

        {
            let decimal = Decimal::new(20, 8);
            let divide_by = Decimal::new(3, 3);
            let actual = decimal.div(divide_by);
            let expected = Decimal::new(6666, 8);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.decimals, expected.decimals);
        }
    }

    #[test]
    #[should_panic]
    fn test_div_panic() {
        let decimal = Decimal::new(10, 3);
        let divide_by = Decimal::new(0, 1);
        decimal.div(divide_by);
    }

    #[test]
    fn test_lte() {
        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(33, 2); // 0.33
            let actual = decimal.lte(other); // true
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(1001, 6); // 0.001001
            let actual = decimal.lte(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(100100, 6); // 0.100100
            let actual = decimal.lte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_lt() {
        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(33, 2); // 0.33
            let actual = decimal.lt(other); // true
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(1001, 6); // 0.001001
            let actual = decimal.lt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(100100, 6); // 0.1001
            let actual = decimal.lt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other);
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gt() {
        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(33, 2); // 0.33
            let actual = decimal.gt(other); // false
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(1001, 6); // 0.001001
            let actual = decimal.gt(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(100100, 6); // 0.1001
            let actual = decimal.gt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other);
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gte() {
        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(33, 2); // 0.33
            let actual = decimal.gte(other); // true
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(1001, 6); // 0.001001
            let actual = decimal.gte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(100100, 6); // 0.1001
            let actual = decimal.gte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other);
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_eq() {
        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(33, 2); // 0.33
            let actual = decimal.eq(other); // false
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(1001, 6); // 0.001001
            let actual = decimal.eq(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4); // 0.1001
            let other = Decimal::new(100100, 6); // 0.1001
            let actual = decimal.eq(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other);
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other);
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other);
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_0_decimals() {
        let decimal100 = Decimal::new(100, 0);
        let decimal5 = Decimal::new(5, 0);

        assert_eq!(decimal100.mul(decimal5), Decimal::new(500, 0));
    }
}

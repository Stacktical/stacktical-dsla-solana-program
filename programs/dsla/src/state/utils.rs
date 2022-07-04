use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use std::convert::TryInto;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Provider,
    User,
}
#[zero_copy]
#[derive(PartialEq, Default, Debug, AnchorDeserialize, AnchorSerialize)]
pub struct Decimal {
    // 17
    pub val: u128, // 16
    pub scale: u8, // 1
}

impl Decimal {
    pub fn new(value: u128, scale: u8) -> Self {
        Self { val: value, scale }
    }
    pub fn denominator(self) -> u128 {
        10u128.pow(self.scale.into())
    }

    pub fn to_scale(self, scale: u8) -> Self {
        Self {
            val: if self.scale > scale {
                self.val
                    .checked_div(10u128.pow((self.scale - scale).into()))
                    .unwrap()
            } else {
                self.val
                    .checked_mul(10u128.pow((scale - self.scale).into()))
                    .unwrap()
            },
            scale,
        }
    }
    pub fn to_scale_up(self, scale: u8) -> Self {
        let decimal = Self::new(self.val, scale);
        if self.scale >= scale {
            decimal.div_up(Self::new(
                10u128.pow((self.scale - scale).try_into().unwrap()),
                0,
            ))
        } else {
            decimal.mul_up(Self::new(
                10u128.pow((scale - self.scale).try_into().unwrap()),
                0,
            ))
        }
    }
}

impl Mul<Decimal> for Decimal {
    fn mul(self, value: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(value.val)
                .unwrap()
                .checked_div(value.denominator())
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl Mul<u128> for Decimal {
    fn mul(self, value: u128) -> Self {
        Self {
            val: self.val.checked_mul(value).unwrap(),
            scale: self.scale,
        }
    }
}
impl MulUp<Decimal> for Decimal {
    fn mul_up(self, other: Decimal) -> Self {
        let denominator = other.denominator();

        Self {
            val: self
                .val
                .checked_mul(other.val)
                .unwrap()
                .checked_add(denominator.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(denominator)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl Add<Decimal> for Decimal {
    fn add(self, value: Decimal) -> Result<Self> {
        require!(self.scale == value.scale, ErrorCode::DifferentScale);

        Ok(Self {
            val: self.val.checked_add(value.val).unwrap(),
            scale: self.scale,
        })
    }
}
impl Sub<Decimal> for Decimal {
    fn sub(self, value: Decimal) -> Result<Self> {
        require!(self.scale == value.scale, ErrorCode::DifferentScale);
        Ok(Self {
            val: self.val.checked_sub(value.val).unwrap(),
            scale: self.scale,
        })
    }
}
impl Div<Decimal> for Decimal {
    fn div(self, other: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(other.denominator())
                .unwrap()
                .checked_div(other.val)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl DivUp<Decimal> for Decimal {
    fn div_up(self, other: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(other.denominator())
                .unwrap()
                .checked_add(other.val.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(other.val)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl DivScale<Decimal> for Decimal {
    fn div_to_scale(self, other: Decimal, to_scale: u8) -> Self {
        let decimal_difference = (self.scale as i32)
            .checked_sub(to_scale.into())
            .unwrap()
            .checked_sub(other.scale.into())
            .unwrap();

        let val = if decimal_difference > 0 {
            self.val
                .checked_div(other.val)
                .unwrap()
                .checked_div(10u128.pow(decimal_difference.try_into().unwrap()))
                .unwrap()
        } else {
            self.val
                .checked_mul(10u128.pow((-decimal_difference).try_into().unwrap()))
                .unwrap()
                .checked_div(other.val)
                .unwrap()
        };
        Self {
            val,
            scale: to_scale,
        }
    }
}
impl PowAccuracy<u128> for Decimal {
    fn pow_with_accuracy(self, exp: u128) -> Self {
        let one = Decimal {
            val: self.denominator(),
            scale: self.scale,
        };
        if exp == 0 {
            return one;
        }
        let mut current_exp = exp;
        let mut base = self;
        let mut result = one;

        while current_exp > 0 {
            if current_exp % 2 != 0 {
                result = result.mul(base);
            }
            current_exp /= 2;
            base = base.mul(base);
        }
        return result;
    }
}
impl Into<u64> for Decimal {
    fn into(self) -> u64 {
        self.val.try_into().unwrap()
    }
}
impl Into<u128> for Decimal {
    fn into(self) -> u128 {
        self.val.try_into().unwrap()
    }
}
impl Compare<Decimal> for Decimal {
    fn lte(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, ErrorCode::DifferentScale);
        Ok(self.val <= other.val)
    }
    fn lt(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, ErrorCode::DifferentScale);
        Ok(self.val < other.val)
    }
    fn gt(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, ErrorCode::DifferentScale);
        Ok(self.val > other.val)
    }
    fn gte(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, ErrorCode::DifferentScale);
        Ok(self.val >= other.val)
    }
    fn eq(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, ErrorCode::DifferentScale);
        Ok(self.val == other.val)
    }
}
pub trait Sub<T>: Sized {
    fn sub(self, rhs: T) -> Result<Self>;
}
pub trait Add<T>: Sized {
    fn add(self, rhs: T) -> Result<Self>;
}
pub trait Div<T>: Sized {
    fn div(self, rhs: T) -> Self;
}
pub trait DivScale<T> {
    fn div_to_scale(self, rhs: T, to_scale: u8) -> Self;
}
pub trait DivUp<T>: Sized {
    fn div_up(self, rhs: T) -> Self;
}
pub trait Mul<T>: Sized {
    fn mul(self, rhs: T) -> Self;
}
pub trait MulUp<T>: Sized {
    fn mul_up(self, rhs: T) -> Self;
}
pub trait PowAccuracy<T>: Sized {
    fn pow_with_accuracy(self, rhs: T) -> Self;
}
pub trait Compare<T>: Sized {
    fn eq(self, rhs: T) -> Result<bool>;
    fn lt(self, rhs: T) -> Result<bool>;
    fn gt(self, rhs: T) -> Result<bool>;
    fn gte(self, rhs: T) -> Result<bool>;
    fn lte(self, rhs: T) -> Result<bool>;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_mul_decimal() {
        let decimal = Decimal::new(1234, 3);
        let multiply_by = Decimal::new(4321, 5);
        let actual = decimal.mul(multiply_by);
        let expected = Decimal::new(53, 3);

        assert_eq!({ actual.val }, { expected.val });
        assert_eq!(actual.scale, expected.scale);
    }

    #[test]
    #[should_panic]
    fn test_mul_decimal_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 3);
        let multiply_by = Decimal::new(2, 3);
        decimal.mul(multiply_by);
    }

    #[test]
    fn test_mul_u128() {
        {
            let decimal = Decimal::new(9876, 2);
            let multiply_by: u128 = 555;
            let actual = decimal.mul(multiply_by);
            let expected = Decimal::new(5481180, 2);

            assert_eq!({ actual.val }, { expected.val });
            assert_eq!(actual.scale, expected.scale);
        }
    }

    #[test]
    #[should_panic]
    fn test_mul_u128_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 2);
        let multiply_by = 2;
        decimal.mul(multiply_by);
    }

    #[test]
    fn test_add() {
        {
            let decimal = Decimal::new(1337, 6);
            let increase_by = Decimal::new(555, 2);
            let actual = decimal.add(increase_by);

            assert!(actual.is_err());
        }

        {
            let decimal = Decimal::new(1337, 6);
            let increase_by = Decimal::new(555, 6);
            let actual = decimal.add(increase_by).unwrap();
            let expected = Decimal::new(1892, 6);

            assert_eq!({ actual.val }, { expected.val });
        }
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 2);
        let increase_by = Decimal::new(2, 2);
        assert!(decimal.add(increase_by).is_err());
    }

    #[test]
    fn test_sub() {
        {
            let decimal = Decimal::new(1337, 6);
            let decrease_by = Decimal::new(555, 2);
            let actual = decimal.sub(decrease_by);

            assert!(actual.is_err());
        }

        {
            let decimal = Decimal::new(1337, 6);
            let decrease_by = Decimal::new(555, 6);
            let actual = decimal.sub(decrease_by).unwrap();
            let expected = Decimal::new(782, 6);

            assert_eq!({ actual.val }, { expected.val });
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_panic() {
        let decimal = Decimal::new(1, 1);
        let decrease_by = Decimal::new(2, 1);
        assert!(decimal.sub(decrease_by).is_err());
    }

    #[test]
    fn test_div() {
        {
            let decimal = Decimal::new(20, 8);
            let divide_by = Decimal::new(2, 3);
            let actual = decimal.div(divide_by);
            let expected = Decimal::new(10000, 8);

            assert_eq!({ actual.val }, { expected.val });
        }

        {
            let decimal = Decimal::new(20, 8);
            let divide_by = Decimal::new(3, 3);
            let actual = decimal.div(divide_by);
            let expected = Decimal::new(6666, 8);

            assert_eq!({ actual.val }, { expected.val });
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
    fn test_into_u64() {
        {
            let decimal = Decimal::new(333333333333333, 15);
            let actual: u64 = decimal.into();
            let expected: u64 = 333333333333333;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_into_u64_panic() {
        let decimal = Decimal::new(u128::MAX - 1, 15);
        let result: u64 = decimal.into();
    }

    #[test]
    fn test_into_u128() {
        {
            let decimal = Decimal::new(111000111, 10);
            let actual: u128 = decimal.into();
            let expected: u128 = 111000111;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_lte() {
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 2);
            let result = decimal.lte(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_lt() {
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 2);
            let result = decimal.lt(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.lt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gt() {
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 2);
            let result = decimal.gt(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gt(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_gte() {
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 2);
            let result = decimal.gte(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.gte(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_eq() {
        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 2);
            let result = decimal.eq(other);

            assert!(result.is_err());
        }

        {
            let decimal = Decimal::new(1001, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(33, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other).unwrap();
            let expected = true;

            assert_eq!(actual, expected);
        }

        {
            let decimal = Decimal::new(10, 4);
            let other = Decimal::new(33, 4);
            let actual = decimal.eq(other).unwrap();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_mul_up() {
        // mul of little
        {
            let a = Decimal::new(1, 10);
            let b = Decimal::new(1, 10);
            assert_eq!(a.mul_up(b), Decimal::new(1, 10));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::new(1000, 3);
            let b = Decimal::new(300, 3);
            assert_eq!(a.mul_up(b), Decimal::new(300, 3));
        }
        // mul by zero
        {
            let a = Decimal::new(1000, 3);
            let b = Decimal::new(0, 0);
            assert_eq!(a.mul_up(b), Decimal::new(0, 3));
        }
        // mul with different decimals
        {
            let a = Decimal::new(1_000_000_000, 9);
            let b = Decimal::new(3, 8);
            assert_eq!(a.mul_up(b), Decimal::new(30, 9));
        }
    }

    #[test]
    fn test_div_up() {
        // div of zero
        {
            let a = Decimal::new(0, 0);
            let b = Decimal::new(1, 0);
            assert_eq!(a.div_up(b), Decimal::new(0, 0));
        }
        // div check rounding up
        {
            let a = Decimal::new(1, 0);
            let b = Decimal::new(2, 0);
            assert_eq!(a.div_up(b), Decimal::new(1, 0));
        }
        // div big number
        {
            let a = Decimal::new(200_000_000_001, 6);
            let b = Decimal::new(2_000, 3);
            assert!(!a.div_up(b).lt(Decimal::new(100_000_000_001, 6)).unwrap());
        }
        {
            let a = Decimal::new(42, 2);
            let b = Decimal::new(10, 0);
            assert_eq!(a.div_up(b), Decimal::new(5, 2));
        }
    }
}

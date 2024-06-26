//!
//! Value type returned when an expression is evaluated.
//! The module includes implementations of the
//! add, sub, mul and div traits to make it easy to
//! operate on ExpressValue instances directly.
//!
use std::fmt::Display;

pub type DecimalType = f64;
pub type IntegerType = i32;
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionValue {
    NaN,
    Decimal {
        value: DecimalType,  // value of the number
    },
    Integer {
        value: IntegerType,  // value of the number
    },
}
impl Display for ExpressionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionValue::NaN => f.write_str("NaN"),
            ExpressionValue::Decimal { value } => {
                f.write_fmt(format_args!("{}", value))
            },
            ExpressionValue::Integer { value } => {
                f.write_fmt(format_args!("{}", value))
            },
        }
    }
}

pub trait Power<Rhs = Self> {
    type Output;

    /// Performs the `^` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 ^ 2, 144);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn power(self, rhs: Rhs) -> Self::Output;
}

///
/// ExpressionValue.power(ExpressionValue) = ExpressionValue
///
impl Power for ExpressionValue {
    type Output = ExpressionValue;

    fn power(self, rhs: Self) -> Self::Output {
        match self {
            ExpressionValue::NaN => ExpressionValue::NaN,
            ExpressionValue::Decimal { value: left_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: left_value.powf(value) },
                ExpressionValue::Integer { value } => ExpressionValue::Decimal{ value: left_value.powf(value as DecimalType) },
            },
            ExpressionValue::Integer { value: left_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: (left_value as DecimalType).powf(value) },
                ExpressionValue::Integer { value } => ExpressionValue::Integer{ value: (left_value as DecimalType).powi(value) as IntegerType },
            },
        }
    }
}

///
/// ExpressionValue + ExpressionValue = ExpressionValue
///
impl std::ops::Add for &ExpressionValue {
    type Output = ExpressionValue;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ExpressionValue::NaN => ExpressionValue::NaN,
            ExpressionValue::Decimal { value: decimal_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: decimal_value + value },
                ExpressionValue::Integer { value } => ExpressionValue::Decimal{ value: decimal_value + (*value as DecimalType)},
            },
            ExpressionValue::Integer { value: integer_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: *integer_value as DecimalType + value },
                ExpressionValue::Integer { value } => ExpressionValue::Integer{ value: integer_value + value},
            },
        }
    }
}
impl std::ops::AddAssign for ExpressionValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs
    }
}

///
/// ExpressionValue - ExpressionValue = ExpressionValue
///
impl std::ops::Sub for &ExpressionValue {
    type Output = ExpressionValue;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            ExpressionValue::NaN => ExpressionValue::NaN,
            ExpressionValue::Decimal { value: decimal_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: decimal_value - value },
                ExpressionValue::Integer { value } => ExpressionValue::Decimal{ value: decimal_value - (*value as DecimalType)},
            },
            ExpressionValue::Integer { value: integer_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: *integer_value as DecimalType - value },
                ExpressionValue::Integer { value } => ExpressionValue::Integer{ value: integer_value - value},
            },
        }
    }
}
impl std::ops::SubAssign for ExpressionValue {
    fn sub_assign(&mut self, rhs: Self) {
        *self = &*self - &rhs
    }
}

///
/// ExpressionValue * ExpressionValue = ExpressionValue
///
impl std::ops::Mul for &ExpressionValue {
    type Output = ExpressionValue;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            ExpressionValue::NaN => ExpressionValue::NaN,
            ExpressionValue::Decimal { value: decimal_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: decimal_value * value },
                ExpressionValue::Integer { value } => ExpressionValue::Decimal{ value: decimal_value * (*value as DecimalType)},
            },
            ExpressionValue::Integer { value: integer_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: *integer_value as DecimalType * value },
                ExpressionValue::Integer { value } => ExpressionValue::Integer{ value: integer_value * value},
            },
        }
    }
}
impl std::ops::MulAssign for ExpressionValue {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * &rhs
    }
}

///
/// ExpressionValue / ExpressionValue = ExpressionValue
///
impl std::ops::Div for &ExpressionValue {
    type Output = ExpressionValue;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            ExpressionValue::NaN => ExpressionValue::NaN,
            ExpressionValue::Decimal { value: decimal_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } if *value == 0.0 => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: decimal_value / value },
                ExpressionValue::Integer { value: 0 } => ExpressionValue::NaN,
                ExpressionValue::Integer { value } => ExpressionValue::Decimal{ value: decimal_value / (*value as DecimalType)},
            },
            ExpressionValue::Integer { value: integer_value } => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } if *value == 0.0  => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal{ value: *integer_value as DecimalType / value },
                ExpressionValue::Integer { value: 0 } => ExpressionValue::NaN,
                ExpressionValue::Integer { value } => ExpressionValue::Integer{ value: integer_value / value},
            },
        }
    }
}
impl std::ops::DivAssign for ExpressionValue {
    fn div_assign(&mut self, rhs: Self) {
        *self = &*self / &rhs
    }
}

///
/// ExpressionValue * SignType = ExpressionValue
///
impl std::ops::Mul<SignType> for ExpressionValue {
    type Output = ExpressionValue;

    fn mul(self, rhs: SignType) -> Self::Output {
        match rhs {
            SignType::Negative => match self {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal { value: -value },
                ExpressionValue::Integer { value } => ExpressionValue::Integer { value: -value },
            },
            SignType::Positive => self,
        }
    }
}

///
/// &SignType * ExpressionValue = ExpressionValue
///
impl std::ops::Mul<ExpressionValue> for &SignType {
    type Output = ExpressionValue;

    fn mul(self, rhs: ExpressionValue) -> Self::Output {
        match self {
            SignType::Negative => match rhs {
                ExpressionValue::NaN => ExpressionValue::NaN,
                ExpressionValue::Decimal { value } => ExpressionValue::Decimal { value: -value },
                ExpressionValue::Integer { value } => ExpressionValue::Integer { value: -value },
            },
            SignType::Positive => rhs,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum SignType {
    Negative = -1,
    Positive = 1
}
///
/// true -> SignType::Positive
/// false -> SignType::Negative
///
impl From<bool> for SignType {
    fn from(value: bool) -> Self {
        if value {
            SignType::Positive
        } else {
            SignType::Negative
        }
    }
}
impl From<DecimalType> for SignType {
    fn from(value: DecimalType) -> Self {
        if value < 0.0 {
            SignType::Negative
        } else {
            SignType::Positive
        }
    }
}
impl From<IntegerType> for SignType {
    fn from(value: IntegerType) -> Self {
        if value < 0 {
            SignType::Negative
        } else {
            SignType::Positive
        }
    }
}
impl From<SignType> for DecimalType {
    fn from(value: SignType) -> Self {
        match value {
            SignType::Negative => -1 as DecimalType,
            SignType::Positive => 1 as DecimalType,
        }
    }
}
impl From<SignType> for IntegerType {
    fn from(value: SignType) -> Self {
        match value {
            SignType::Negative => -1 as IntegerType,
            SignType::Positive => 1 as IntegerType,
        }
    }
}

// TODO: port the parser test from https://github.com/Ezward/ExpressionCalculator/blob/master/test/com/lumpofcode/expression/ExpressionParserTest.java

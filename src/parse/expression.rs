use std::ops::Deref;

use super::{value::{ExpressionValue, DecimalType, IntegerType, SignType, Power}, position::ParsePosition};



pub trait Evaluate {
    fn evaluate(&self) -> ExpressionValue;
}
pub trait Position {
    fn position(&self) -> ParsePosition;
}


#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    NaN,
    Integer{ position: ParsePosition, value: IntegerType },
    Decimal{ position: ParsePosition, value: DecimalType },
    Parenthesis{ position: ParsePosition, sign: SignType, inner: Box<ExpressionNode> },
    Sum{ position: ParsePosition, operands: Vec<ExpressionNode> },
    Power{ position: ParsePosition, base: Box<ExpressionNode>, exponent: Box<ExpressionNode> },
}

impl Evaluate for ExpressionNode {
    fn evaluate(&self) -> ExpressionValue {
        match self {
            ExpressionNode::NaN => ExpressionValue::NaN,
            ExpressionNode::Integer { position: _, value } => ExpressionValue::Integer { value: *value },
            ExpressionNode::Decimal { position: _, value } => ExpressionValue::Decimal { value: *value },
            ExpressionNode::Parenthesis { position: _, sign, inner } => sign * inner.evaluate(),
            ExpressionNode::Sum { position: _, operands } => {
                let mut sum = operands[0].evaluate();
                for addend in operands[1..].iter() {
                    sum += addend.evaluate()
                }
                sum
            },
            ExpressionNode::Power { position: _, base, exponent } => {
                let base_value = base.evaluate();
                let exponent_value = exponent.evaluate();
                base_value.power(exponent_value)
            },
        }
    }
}

impl Position for ExpressionNode {
    fn position(&self) -> ParsePosition {
        match self {
            ExpressionNode::NaN => ParsePosition::default(),
            ExpressionNode::Integer { position, value: _ } => position.clone(),
            ExpressionNode::Decimal { position, value: _ } => position.clone(),
            ExpressionNode::Parenthesis { position, sign: _, inner: _ } => position.clone(),
            ExpressionNode::Sum { position, operands: _ } => position.clone(),
            ExpressionNode::Power { position, base: _, exponent: _ } => position.clone(),
        }
    }
}

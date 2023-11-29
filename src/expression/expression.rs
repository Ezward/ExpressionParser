use super::value::{ExpressionValue, DecimalType, IntegerType, SignType};



pub trait Expression {
    fn evaluate(&self) -> ExpressionValue;
}


#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Nil,
    Integer{ value: IntegerType },
    Decimal{ value: DecimalType },
    Parenthesis{ sign: SignType, inner: Box<ExpressionNode> },
    Sum{ operands: Vec<ExpressionNode> }
}

impl Expression for ExpressionNode {
    fn evaluate(&self) -> ExpressionValue {
        match self {
            ExpressionNode::Nil => todo!(),
            ExpressionNode::Integer { value } => ExpressionValue::Integer { value: *value },
            ExpressionNode::Decimal { value } => ExpressionValue::Decimal { value: *value },
            ExpressionNode::Parenthesis { sign, inner } => sign * inner.evaluate(),
            ExpressionNode::Sum { operands } => {
                let mut sum = operands[0].evaluate();
                for addend in operands[1..].iter() {
                    sum += addend.evaluate()
                }
                sum
            },
        }
    }
}

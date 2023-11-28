use super::value::{ExpressionValue, DecimalType, IntegerType, SignType};



pub trait Expression {
    fn evaluate(&self) -> ExpressionValue;
}


#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Nil,
    Integer{ value: IntegerType },
    Decimal{ value: DecimalType },
    Parenthesis{ sign: SignType, inner: Box<ExpressionNode> }
}

impl Expression for ExpressionNode {
    fn evaluate(&self) -> ExpressionValue {
        match self {
            ExpressionNode::Nil => todo!(),
            ExpressionNode::Integer { value } => ExpressionValue::Integer { value: *value },
            ExpressionNode::Decimal { value } => ExpressionValue::Decimal { value: *value },
            ExpressionNode::Parenthesis { sign, inner } => sign * inner.evaluate(),
        }
    }
}

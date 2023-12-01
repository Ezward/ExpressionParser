use crate::parse::position::ParsePosition;

use super::expression::Position;


#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParsingError {
    Unknown(ParsePosition),
    EndOfInput(ParsePosition),
    ExtraInput(ParsePosition),
    Number(ParsePosition),
}
impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::Unknown(position) => {
                f.write_fmt(format_args!("Unknown parsing error at {:?}", &position))
            },
            ParsingError::EndOfInput(position) => {
                f.write_fmt(format_args!("Unexpected end of input parsing expression at {:?}", &position))
            },
            ParsingError::ExtraInput(position) => {
                f.write_fmt(format_args!("Unexpected input after expression at {:?}", &position))
            },
            ParsingError::Number(position) => {
                f.write_fmt(format_args!("Error parsing number at {:?}", &position))
            },
        }
    }
}
impl std::error::Error for ParsingError {}

impl Position for ParsingError {
    fn position(&self) -> ParsePosition {
        match self {
            ParsingError::Unknown(position) => position.clone(),
            ParsingError::EndOfInput(position) => position.clone(),
            ParsingError::ExtraInput(position) => position.clone(),
            ParsingError::Number(position) => position.clone(),
        }
    }
}


#[derive(Debug, Clone)]
pub enum EvaluationError {
    Number{msg: String},
}
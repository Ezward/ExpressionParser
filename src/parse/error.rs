use crate::parse::position::ParsePosition;



#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParsingError {
    Unknown(ParsePosition),
    EndOfInput(ParsePosition),
    Number(ParsePosition),
}
impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::Unknown(position) => {
                f.write_fmt(format_args!("Unknown parsing error at {:?}", &position))
            },
            ParsingError::EndOfInput(position) => {
                f.write_fmt(format_args!("Unexpected end of input at {:?}", &position))
            },
            ParsingError::Number(position) => {
                f.write_fmt(format_args!("Error parsing number at {:?}", &position))
            },
        }
    }
}
impl std::error::Error for ParsingError {}


#[derive(Debug, Clone)]
pub enum EvaluationError {
    Number{msg: String},
}
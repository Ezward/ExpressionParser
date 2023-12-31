//!
//! A simple expression parser and evaluator for the 4 operations and exponentiation.
//!
//! @author Ezward
//!
//! NOTE: this grammar separates out sums from differences and products from quotients.
//!       Thus, it is not a traditional factor/term grammar.  The grammar is
//!       designed to separate out operations that are subject to the associative
//!       and commutative properties with the notion that the parse tree can
//!       then be more easily queried or manipulated using those mathematical properties.
//!
//! Parses the following PEG grammar:
//!
//! ```
//! digit ::= [0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9]
//! sign ::= '-'
//! integer ::= {sign} [digit]*
//! decimal ::= {sign} [digit]* '.' [digit]*
//! scientific ::= {sign} [digit]* {'.' [digit]*} ['e' | 'E'] {sign} [digit]*
//! number ::= [integer | decimal | scientific]
//! parenthesis ::= {sign} '(' expression ')'
//! value ::= [parenthesis | number]
//! power ::= value{'^'value}
//! quotient ::= power {['÷' | '/'] power}*
//! product ::= quotient {['×' | '*']  quotient}*
//! difference ::= product  {'-' product}*
//! sum ::= difference {'+' difference}*
//! expression ::= sum
//!
//! Key to PEG notation:
//! {} = optional, choose zero or one
//! {}* = optional, 0 or more
//! [] = required, choose one
//! []* = required, 1 or more
//!
//! Usage:
//!   let s = " (((10 + 5) * -6) - -20.0 / -2 * 3 + -((5*2)^2) - (-5 * -2 * 5)) ";
//!   let (_result_context, result_node) = parse(s, beginning()).unwrap();
//!   assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -270 as DecimalType});
//! ```
//!
use crate::expression::node::{Position, Evaluate};
use crate::scan::context::{
    ScanPosition,
    ScanContext,
    scan_one_or_more_chars,
    scan_literal,
    scan_zero_or_more_chars
};

use crate::expression::position::ParsePosition;
use crate::expression::error::ParsingError;

use super::node::ExpressionNode;
use super::value::SignType;



fn scan_whitespace(s: &str, context: ScanContext) -> ScanContext {
    scan_zero_or_more_chars(s, context, |ch| ch.is_ascii_whitespace())
}
fn scan_digits(s: &str, context: ScanContext) -> ScanContext {
    scan_one_or_more_chars(s, context, |ch| ch.is_ascii_digit())
}
fn scan_to_end(s: &str, context: ScanContext) -> ScanContext {
    scan_zero_or_more_chars(s, context, |_ch| true)  // scan to end of input
}

///
/// Check the scan context for a match.
/// If it is not a match then return an appropriate error.
/// If it is a match, pass through the matching scan context.
///
fn expect_match(s: &str, start_position: ScanPosition, context: ScanContext) -> Result<ScanContext, ParsingError> {
    let (matched, position) = context;
    if !matched {
        if position.byte_index >= s.len() {
            Err(ParsingError::EndOfInput(ParsePosition::new(&start_position, &position)))
        } else {
            Err(ParsingError::Number(ParsePosition::new(&start_position, &position)))
        }
    } else {
        Ok(context)
    }
}



fn parse_whitespace(s: &str, context: ScanContext) -> Result<ScanContext, ParsingError> {
    expect_match(s, context.1, scan_whitespace(s, context))
}

///
/// Exhaustively parse the string.
/// This will error is there are extra non-whitespace characters after the expression.
///
pub fn parse(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    match parse_expression(s, context) {
        Ok((expression_context, expression_node)) => {
            let (matched, position) = scan_whitespace(s, expression_context);
            if !matched || position.byte_index < s.len() {
                Err(ParsingError::ExtraInput(ParsePosition {
                    start: position,
                    end: scan_to_end(s, (matched, position)).1  // scan to end of input
                }))
            } else {
                Ok((expression_context, expression_node))
            }
        },
        Err(e) => Err(e),
    }
}

///
/// Parse the expression and return where it ends.
/// ```
/// expression ::= sum
/// ```
///
pub fn parse_expression(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    parse_sum(s, context)
}

pub fn print_expression_result(s: &str, context:ScanContext) {
    match parse_expression(s, context) {
        Ok((_context, expression_node)) => {
            println!("{} = {}", &s[expression_node.position().start.byte_index..expression_node.position().end.byte_index], expression_node.evaluate());
        },
        Err(e) => {
            println!("{}", s);
            if e.position().end.char_index - e.position().start.char_index > 1 {
                println!("{}^{}", " ".repeat(e.position().start.char_index), "^".repeat(e.position().end.char_index - e.position().start.char_index - 1));
            } else {
                println!("{}^", " ".repeat(e.position().start.char_index));
            }
            println!("{}", e);
        },
    }
}

pub fn print_result(s: &str, context:ScanContext) {
    match parse_expression(s, context) {
        Ok((_context, expression_node)) => {
            println!("{}", expression_node.evaluate());
        },
        Err(e) => {
            println!("{}", s);
            if e.position().end.char_index - e.position().start.char_index > 1 {
                println!("{}^{}", " ".repeat(e.position().start.char_index), "^".repeat(e.position().end.char_index - e.position().start.char_index - 1));
            } else {
                println!("{}^", " ".repeat(e.position().start.char_index));
            }
            println!("{}", e);
        },
    }
}

///
/// Parse a number.
///
/// ```
///  digit ::= [0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9]
///  sign ::= '-'
///  integer ::= {sign} [digit]*
///  decimal ::= {sign} [digit]* '.' [digit]*
///  scientific ::= {sign} [digit]* {'.' [digit]*} ['e' | 'E'] {sign} [digit]*
///  number ::= [integer | decimal | scientific]
/// ```
///
fn parse_number(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    //
    // skip any leading whitespace
    //
    let (mut _matched, start_position) = parse_whitespace(s, context)?;

    //
    // parse the optional negation
    //
    let (_is_negative, mut position) = scan_literal(s, (true, start_position), "-");

    //
    // scan the required integer part
    //
    (_matched, position) = expect_match(s, start_position, scan_digits(s, (true, position)))?;

    //
    // scan the optional decimal part
    //
    let is_decimal;
    (is_decimal, position) = scan_literal(s, (true, position), ".");
    if is_decimal {
        (_matched, position) = expect_match(s, start_position, scan_digits(s, (true, position)))?;
    }

    //
    // scan the optional exponent
    //
    let (mut has_exponent, mut exponent_position) = scan_literal(s, (true, position), "e");
    if !has_exponent {
        (has_exponent, exponent_position) = scan_literal(s, (true, position), "E");
    }
    if has_exponent {
        (_matched, position) = expect_match(s, start_position, scan_digits(s, (true, exponent_position)))?;
    }

    //
    // return the scanned value
    //
    Ok(((true, position), if is_decimal || has_exponent {
            ExpressionNode::Decimal{
                position: ParsePosition::new(&start_position, &position),
                value: s[start_position.byte_index..position.byte_index].parse::<f64>().map_err(|err| {
                    println!("Error converting decimal number at {:?}: {}", ParsePosition::new(&start_position, &position), &err);
                    ParsingError::Number(ParsePosition::new(&start_position, &position))
                })?
            }
        } else {
            // integer
            ExpressionNode::Integer{
                position: ParsePosition::new(&start_position, &position),
                value: s[start_position.byte_index..position.byte_index].parse::<i32>().map_err(|err| {
                    println!("Error converting integer at {:?}: {}", ParsePosition::new(&start_position, &position), &err);
                    ParsingError::Number(ParsePosition::new(&start_position, &position))
                })?
            }
        }
    ))
}

///
/// Parse a parenthesized expression.
///
/// ```
/// value ::= [parenthesis | number]
/// parenthesis ::= {sign} '(' expression ')'
/// ```
///
fn parse_value(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    //
    // skip any leading whitespace
    //
    let (mut matched, start_position) = parse_whitespace(s, context)?;

    //
    // parse the optional negation
    //
    let (is_negative, mut position) = scan_literal(s, (matched, start_position), "-");

    //
    // scan opening brace
    //
    (matched, position) = scan_literal(s, (matched, position), "(");
    if matched {
        //
        // parse the expression inside the parenthesis
        //
        let inner_node: ExpressionNode;

        ((matched, position), inner_node) = parse_expression(s, (matched, position))?;

        //
        // scan the required closing parenthesis
        //
        (matched, position) = expect_match(s, start_position, scan_literal(s, parse_whitespace(s, (matched, position))?, ")"))?;

        Ok(((matched, position), ExpressionNode::Parenthesis {
                position: ParsePosition::new(&start_position, &position),
                sign: SignType::from(!is_negative),
                inner: Box::new(inner_node),
            }
        ))

    } else {
        //
        // if it's not a parenthesis, then it must be a number.
        // start at the optional negation
        //
        parse_number(s, (true, start_position))
    }
}

///
/// Parse an exponentiation expression.
///
/// ```
/// power ::= value{'^'value}
/// ```
///
fn parse_power(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    const OPERATOR: &str = "^";

    //
    // skip any leading whitespace
    //
    let (matched, start_position) = parse_whitespace(s, context)?;


    let ((matched, left_position), left_node) = parse_value(s, (matched, start_position))?;

    //
    // scan operator
    //
    let (matched, position) = scan_literal(s, (matched, left_position), OPERATOR);
    if matched {
        // scan right side operand
        let ((_matched, right_position), right_node) = parse_value(s, (matched, position))?;

        Ok(((true, right_position), ExpressionNode::Power {
                position: ParsePosition::new(&start_position, &right_position),
                base: Box::new(left_node),
                exponent: Box::new(right_node)
            }
        ))
    } else {
        //
        // no operand, so just return the left expression
        //
        Ok(((true, left_position), left_node))
    }

}

///
/// Parse a series of addition operations.
///
/// ```
/// sum ::= difference {'+' difference}*
/// ```
///
fn parse_sum(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    const OPERATOR: &str = "+";

    //
    // skip any leading whitespace
    //
    let (matched, start_position) = parse_whitespace(s, context)?;


    let ((matched, mut operand_position), left_node) = parse_difference(s, (matched, start_position))?;
    let end_position = operand_position;

    //
    // scan operator
    //
    let (mut matched, mut position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
    if matched {
        //
        // collect up all addends.
        // - pull the expression node out of the Box in the ParseNode,
        // - put it into the vector
        // - put the vector into an sum expression node
        //
        let mut addends = vec!(left_node);
        while matched {
            let parse_node: ExpressionNode;

            // scan next operand
            ((matched, operand_position), parse_node) = parse_difference(s, (matched, position))?;

            // add it to the operands
            addends.push(parse_node);

            // scan next operator
            (matched, position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
        }

        Ok(((true, operand_position), ExpressionNode::Sum {
                position: ParsePosition::new(&start_position, &operand_position),
                operands: addends
            }
        ))


    } else {
        //
        // no operand, so just return the left expression
        //
        Ok(((true, end_position), left_node))
    }

}

///
/// Parse a series of subtraction operations.
///
/// ```
/// difference ::= product  {'-' product}*
/// ```
///
fn parse_difference(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    const OPERATOR: &str = "-";

    //
    // skip any leading whitespace
    //
    let (matched, start_position) = parse_whitespace(s, context)?;


    let ((matched, mut operand_position), left_node) = parse_product(s, (matched, start_position))?;
    let end_position = operand_position;

    //
    // scan operator
    //
    let (mut matched, mut position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
    if matched {
        //
        // collect up all operands.
        // - pull the expression node out of the Box in the ParseNode,
        // - put it into the vector
        // - put the vector into an sum expression node
        //
        let mut operands = vec!(left_node);
        while matched {
            let parse_node: ExpressionNode;

            // scan next operand
            ((matched, operand_position), parse_node) = parse_product(s, (matched, position))?;

            // add it to the operands
            operands.push(parse_node);

            // scan next operator
            (matched, position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
        }

        Ok(((true, operand_position), ExpressionNode::Difference {
                position: ParsePosition::new(&start_position, &operand_position),
                operands
            }
        ))


    } else {
        //
        // no operand, so just return the left expression
        //
        Ok(((true, end_position), left_node))
    }

}

///
/// Parse a series of multiplication operations.
///
/// ```
/// product ::= quotient {['×' | '*']  quotient}*
/// ```
///
fn parse_product(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    const OPERATOR: &str = "*";

    //
    // skip any leading whitespace
    //
    let (matched, start_position) = parse_whitespace(s, context)?;


    let ((matched, mut operand_position), left_node) = parse_quotient(s, (matched, start_position))?;
    let end_position = operand_position;

    //
    // scan operator
    //
    let (mut matched, mut position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
    if matched {
        //
        // collect up all operands.
        // - pull the expression node out of the Box in the ParseNode,
        // - put it into the vector
        // - put the vector into an sum expression node
        //
        let mut operands = vec!(left_node);
        while matched {
            let parse_node: ExpressionNode;

            // scan next operand
            ((matched, operand_position), parse_node) = parse_quotient(s, (matched, position))?;

            // add it to the operands
            operands.push(parse_node);

            // scan next operator
            (matched, position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
        }

        Ok(((true, operand_position), ExpressionNode::Product {
                position: ParsePosition::new(&start_position, &operand_position),
                operands
            }
        ))


    } else {
        //
        // no operand, so just return the left expression
        //
        Ok(((true, end_position), left_node))
    }

}

///
/// Parse a series of division operations.
///
/// ```
/// quotient ::= power {['÷' | '/'] power}*
/// ```
///
fn parse_quotient(s: &str, context: ScanContext) -> Result<(ScanContext, ExpressionNode), ParsingError> {
    const OPERATOR: &str = "/";

    //
    // skip any leading whitespace
    //
    let (matched, start_position) = parse_whitespace(s, context)?;


    let ((matched, mut operand_position), left_node) = parse_power(s, (matched, start_position))?;
    let end_position = operand_position;

    //
    // scan operator
    //
    let (mut matched, mut position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
    if matched {
        //
        // collect up all operands.
        // - pull the expression node out of the Box in the ParseNode,
        // - put it into the vector
        // - put the vector into an sum expression node
        //
        let mut operands = vec!(left_node);
        while matched {
            let parse_node: ExpressionNode;

            // scan next operand
            ((matched, operand_position), parse_node) = parse_power(s, (matched, position))?;

            // add it to the operands
            operands.push(parse_node);

            // scan next operator
            (matched, position) = scan_literal(s, parse_whitespace(s, (matched, operand_position))?, OPERATOR);
        }

        Ok(((true, operand_position), ExpressionNode::Quotient {
                position: ParsePosition::new(&start_position, &operand_position),
                operands
            }
        ))
    } else {
        //
        // no operand, so just return the left expression
        //
        Ok(((true, end_position), left_node))
    }
}


#[cfg(test)]
mod parse_tests {
    use crate::expression::value::{DecimalType, IntegerType, SignType};

    use super::*;

    #[test]
    fn test_parse_number_integer() {
        let s = "1234";
        let start = ScanPosition::default();
        let context = (true, start);

        let (result_context, result_node) = parse_number(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Integer{
            position: ParsePosition { start: start, end: expected_end },
            value: 1234
        }, result_node);
    }

    #[test]
    fn test_parse_number_decimal() {
        let s = "1234.0";
        let start = ScanPosition::default();
        let context = (true, start);

        let (result_context, result_node) = parse_number(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Decimal{
            position: ParsePosition { start: start, end: expected_end },
            value: 1234 as DecimalType
        }, result_node);
    }

    #[test]
    fn test_parse_number_scientific() {
        let s = "123.4e1";
        let start = ScanPosition::default();
        let context = (true, start);

        let (result_context, result_node) = parse_number(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Decimal{
            position: ParsePosition { start: start, end: expected_end },
            value: 1234 as DecimalType
        }, result_node);

        let s = "123.4E1";
        let start = ScanPosition::default();
        let context = (true, start);

        let (result_context, result_node) = parse_number(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Decimal{
            position: ParsePosition { start: start, end: expected_end },
            value: 1234 as DecimalType
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_integer() {
        let s = " ( 1234 ) ";
        let start = ScanPosition::new(1, 1, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Parenthesis{
            position: ParsePosition {
                start: start,
                end: expected_end
            },
            sign: SignType::Positive,
            inner: Box::new(ExpressionNode::Integer {
                position: ParsePosition {
                    start: ScanPosition::new(3, 3, 0, 0, 0),
                    end: ScanPosition::new(7, 7, 0, 0, 0)
                },
                value: 1234 as IntegerType
            })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_negative_integer() {
        let s = " ( -1234 ) ";
        let start = ScanPosition::new(1, 1, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Parenthesis{
            position: ParsePosition {
                start: start,
                end: expected_end
            },
            sign: SignType::Positive,
            inner: Box::new(ExpressionNode::Integer {
                position: ParsePosition {
                    start: ScanPosition::new(3, 3, 0, 0, 0),
                    end: ScanPosition::new(8, 8, 0, 0, 0)
                },
                value: -1234 as IntegerType
            })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_negative_decimal() {
        let s = " ( -1234.0 ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len()- 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Parenthesis{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            sign: SignType::Positive,
            inner: Box::new(ExpressionNode::Decimal {
                position: ParsePosition {
                    start: ScanPosition::new(3, 3, 0, 0, 0),
                    end: ScanPosition::new(10, 10, 0, 0, 0)
                },
                value: -1234 as DecimalType
            })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_negative() {
        let s = " -( 1234 ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Parenthesis{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end },
            sign: SignType::Negative,
            inner: Box::new(ExpressionNode::Integer {
                position: ParsePosition {
                    start: ScanPosition::new(4, 4, 0, 0, 0),
                    end: ScanPosition::new(8, 8, 0, 0, 0)
                },
                value: 1234 as IntegerType
            })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_nested() {
        let s = " -( -( 1234 ) ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Parenthesis{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            sign: SignType::Negative,
            inner: Box::new(ExpressionNode::Parenthesis {
                position: ParsePosition {
                    start: ScanPosition::new(4, 4, 0, 0, 0),
                    end: ScanPosition::new(13, 13, 0, 0, 0)
                },
                sign: SignType::Negative,
                inner: Box::new(ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(7, 7, 0, 0, 0),
                        end: ScanPosition::new(11, 11, 0, 0, 0)
                    },
                    value: 1234 as IntegerType
                })
            })
        }, result_node);
    }

    #[test]
    fn test_parse_sum() {
        let s = " 2 + 3 ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_sum(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Sum{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(2, 2, 0, 0, 0)
                    },
                    value: 2 as IntegerType
                },
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(5, 5, 0, 0, 0),
                        end: ScanPosition::new(6, 6, 0, 0, 0)
                    },
                    value: 3 as IntegerType
                }
            )
        }, result_node);
    }

    #[test]
    fn test_parse_sum_complex() {
        let s = " ( 1234 ) + -2^16 + -( 30.0^2 + 78.0  ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_sum(s, context).unwrap();
        // println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Sum{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(9, 9, 0, 0, 0)
                    },
                    sign: SignType::Positive,
                    inner: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(3, 3, 0, 0, 0),
                            end: ScanPosition::new(7, 7, 0, 0, 0)
                        },
                        value: 1234 as IntegerType
                    }),
                },
                ExpressionNode::Power {
                    position: ParsePosition {
                        start: ScanPosition::new(12, 12, 0, 0, 0),
                        end: ScanPosition::new(17, 17, 0, 0, 0)
                    },
                    base: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(12, 12, 0, 0, 0),
                            end: ScanPosition::new(14, 14, 0, 0, 0)
                        },
                        value: -2 as IntegerType
                    }),
                    exponent: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(15, 15, 0, 0, 0),
                            end: ScanPosition::new(17, 17, 0, 0, 0)
                        },
                        value: 16 as IntegerType
                    }),
                },
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(20, 20, 0, 0, 0),
                        end: ScanPosition::new(39, 39, 0, 0, 0)
                    },
                    sign: SignType::Negative,
                    inner: Box::new(ExpressionNode::Sum{
                        position: ParsePosition {
                            start: ScanPosition::new(23, 23, 0, 0, 0),
                            end: ScanPosition::new(36, 36, 0, 0, 0)
                        },
                        operands: vec!(
                            ExpressionNode::Power {
                                position: ParsePosition {
                                    start: ScanPosition::new(23, 23, 0, 0, 0),
                                    end: ScanPosition::new(29, 29, 0, 0, 0)
                                },
                                base: Box::new(ExpressionNode::Decimal {
                                    position: ParsePosition {
                                        start: ScanPosition::new(23, 23, 0, 0, 0),
                                        end: ScanPosition::new(27, 27, 0, 0, 0)
                                    },
                                    value: 30 as DecimalType
                                }),
                                exponent: Box::new(ExpressionNode::Integer {
                                    position: ParsePosition {
                                        start: ScanPosition::new(28, 28, 0, 0, 0),
                                        end: ScanPosition::new(29, 29, 0, 0, 0)
                                    },
                                    value: 2 as IntegerType
                                }),
                            },
                            ExpressionNode::Decimal {
                                position: ParsePosition {
                                    start: ScanPosition::new(32, 32, 0, 0, 0),
                                    end: ScanPosition::new(36, 36, 0, 0, 0)
                                },
                                value: 78 as DecimalType
                            },
                        ),
                    }),
                },
            )
        }, result_node);
    }

    #[test]
    fn test_parse_difference() {
        let s = " 2 - 3 ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_difference(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Difference{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(2, 2, 0, 0, 0)
                    },
                    value: 2 as IntegerType
                },
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(5, 5, 0, 0, 0),
                        end: ScanPosition::new(6, 6, 0, 0, 0)
                    },
                    value: 3 as IntegerType
                }
            )
        }, result_node);
    }

    #[test]
    fn test_parse_difference_complex() {
        let s = " ( 1234 ) - -2^16 - -( 30.0^2 - 78.0  ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_difference(s, context).unwrap();
        // println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Difference {
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(9, 9, 0, 0, 0)
                    },
                    sign: SignType::Positive,
                    inner: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(3, 3, 0, 0, 0),
                            end: ScanPosition::new(7, 7, 0, 0, 0)
                        },
                        value: 1234 as IntegerType
                    }),
                },
                ExpressionNode::Power {
                    position: ParsePosition {
                        start: ScanPosition::new(12, 12, 0, 0, 0),
                        end: ScanPosition::new(17, 17, 0, 0, 0)
                    },
                    base: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(12, 12, 0, 0, 0),
                            end: ScanPosition::new(14, 14, 0, 0, 0)
                        },
                        value: -2 as IntegerType
                    }),
                    exponent: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(15, 15, 0, 0, 0),
                            end: ScanPosition::new(17, 17, 0, 0, 0)
                        },
                        value: 16 as IntegerType
                    }),
                },
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(20, 20, 0, 0, 0),
                        end: ScanPosition::new(39, 39, 0, 0, 0)
                    },
                    sign: SignType::Negative,
                    inner: Box::new(ExpressionNode::Difference{
                        position: ParsePosition {
                            start: ScanPosition::new(23, 23, 0, 0, 0),
                            end: ScanPosition::new(36, 36, 0, 0, 0)
                        },
                        operands: vec!(
                            ExpressionNode::Power {
                                position: ParsePosition {
                                    start: ScanPosition::new(23, 23, 0, 0, 0),
                                    end: ScanPosition::new(29, 29, 0, 0, 0)
                                },
                                base: Box::new(ExpressionNode::Decimal {
                                    position: ParsePosition {
                                        start: ScanPosition::new(23, 23, 0, 0, 0),
                                        end: ScanPosition::new(27, 27, 0, 0, 0)
                                    },
                                    value: 30 as DecimalType
                                }),
                                exponent: Box::new(ExpressionNode::Integer {
                                    position: ParsePosition {
                                        start: ScanPosition::new(28, 28, 0, 0, 0),
                                        end: ScanPosition::new(29, 29, 0, 0, 0)
                                    },
                                    value: 2 as IntegerType
                                }),
                            },
                            ExpressionNode::Decimal {
                                position: ParsePosition {
                                    start: ScanPosition::new(32, 32, 0, 0, 0),
                                    end: ScanPosition::new(36, 36, 0, 0, 0)
                                },
                                value: 78 as DecimalType
                            },
                        ),
                    }),
                },
            )
        }, result_node);
    }

    #[test]
    fn test_parse_product() {
        let s = " 2 * 3 ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_product(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Product{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(2, 2, 0, 0, 0)
                    },
                    value: 2 as IntegerType
                },
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(5, 5, 0, 0, 0),
                        end: ScanPosition::new(6, 6, 0, 0, 0)
                    },
                    value: 3 as IntegerType
                }
            )
        }, result_node);
    }


    #[test]
    fn test_parse_quotient() {
        let s = " 2 / 3 ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_quotient(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Quotient{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(2, 2, 0, 0, 0)
                    },
                    value: 2 as IntegerType
                },
                ExpressionNode::Integer {
                    position: ParsePosition {
                        start: ScanPosition::new(5, 5, 0, 0, 0),
                        end: ScanPosition::new(6, 6, 0, 0, 0)
                    },
                    value: 3 as IntegerType
                }
            )
        }, result_node);
    }


    #[test]
    fn test_parse_power() {
        let s = " 2^3 ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_power(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Power{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            base: Box::new(ExpressionNode::Integer {
                position: ParsePosition {
                    start: ScanPosition::new(1, 1, 0, 0, 0),
                    end: ScanPosition::new(2, 2, 0, 0, 0)
                },
                value: 2 as IntegerType
            }),
            exponent: Box::new(ExpressionNode::Integer {
                position: ParsePosition {
                    start: ScanPosition::new(3, 3, 0, 0, 0),
                    end: ScanPosition::new(4, 4, 0, 0, 0)
                },
                value: 3 as IntegerType
            })
        }, result_node);
    }

    #[test]
    fn test_parse_power_complex() {
        let s = " (0.0+2)^(1.0+2) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_power(s, context).unwrap();
        println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Power{
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            base: Box::new(ExpressionNode::Parenthesis {
                position: ParsePosition {
                    start: ScanPosition::new(1, 1, 0, 0, 0),
                    end: ScanPosition::new(8, 8, 0, 0, 0)
                },
                sign: SignType::Positive,
                inner: Box::new(ExpressionNode::Sum{
                    position: ParsePosition {
                        start: ScanPosition::new(2, 2, 0, 0, 0),
                        end: ScanPosition::new(7, 7, 0, 0, 0)
                    },
                    operands: vec!(
                        ExpressionNode::Decimal {
                            position: ParsePosition {
                                start: ScanPosition::new(2, 2, 0, 0, 0),
                                end: ScanPosition::new(5, 5, 0, 0, 0)
                            },
                            value: 0 as DecimalType
                        },
                        ExpressionNode::Integer {
                            position: ParsePosition {
                                start: ScanPosition::new(6, 6, 0, 0, 0),
                                end: ScanPosition::new(7, 7, 0, 0, 0)
                            },
                            value: 2 as IntegerType
                        },
                    ),
                }),
            }),
            exponent: Box::new(ExpressionNode::Parenthesis {
                position: ParsePosition {
                    start: ScanPosition::new(9, 9, 0, 0, 0),
                    end: ScanPosition::new(16, 16, 0, 0, 0)
                },
                sign: SignType::Positive,
                inner: Box::new(ExpressionNode::Sum{
                    position: ParsePosition {
                        start: ScanPosition::new(10, 10, 0, 0, 0),
                        end: ScanPosition::new(15, 15, 0, 0, 0)
                    },
                    operands: vec!(
                        ExpressionNode::Decimal {
                            position: ParsePosition {
                                start: ScanPosition::new(10, 10, 0, 0, 0),
                                end: ScanPosition::new(13, 13, 0, 0, 0)
                            },
                            value: 1 as DecimalType
                        },
                        ExpressionNode::Integer {
                            position: ParsePosition {
                                start: ScanPosition::new(14, 14, 0, 0, 0),
                                end: ScanPosition::new(15, 15, 0, 0, 0)
                            },
                            value: 2 as IntegerType
                        },
                    ),
                }),
            }),
        }, result_node);
    }

    #[test]
    fn test_parse_expression() {
        let s = " ( 1234 ) - -2^16 - -( 30.0^2 + 78.0  ) ";
        let start = ScanPosition::new(0, 0, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_expression(s, context).unwrap();
        // println!("{:?}", result_node);
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ExpressionNode::Difference {
            position: ParsePosition {
                start: ScanPosition::new(1, 1, 0, 0, 0),
                end: expected_end
            },
            operands: vec!(
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(1, 1, 0, 0, 0),
                        end: ScanPosition::new(9, 9, 0, 0, 0)
                    },
                    sign: SignType::Positive,
                    inner: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(3, 3, 0, 0, 0),
                            end: ScanPosition::new(7, 7, 0, 0, 0)
                        },
                        value: 1234 as IntegerType
                    }),
                },
                ExpressionNode::Power {
                    position: ParsePosition {
                        start: ScanPosition::new(12, 12, 0, 0, 0),
                        end: ScanPosition::new(17, 17, 0, 0, 0)
                    },
                    base: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(12, 12, 0, 0, 0),
                            end: ScanPosition::new(14, 14, 0, 0, 0)
                        },
                        value: -2 as IntegerType
                    }),
                    exponent: Box::new(ExpressionNode::Integer {
                        position: ParsePosition {
                            start: ScanPosition::new(15, 15, 0, 0, 0),
                            end: ScanPosition::new(17, 17, 0, 0, 0)
                        },
                        value: 16 as IntegerType
                    }),
                },
                ExpressionNode::Parenthesis {
                    position: ParsePosition {
                        start: ScanPosition::new(20, 20, 0, 0, 0),
                        end: ScanPosition::new(39, 39, 0, 0, 0)
                    },
                    sign: SignType::Negative,
                    inner: Box::new(ExpressionNode::Sum{
                        position: ParsePosition {
                            start: ScanPosition::new(23, 23, 0, 0, 0),
                            end: ScanPosition::new(36, 36, 0, 0, 0)
                        },
                        operands: vec!(
                            ExpressionNode::Power {
                                position: ParsePosition {
                                    start: ScanPosition::new(23, 23, 0, 0, 0),
                                    end: ScanPosition::new(29, 29, 0, 0, 0)
                                },
                                base: Box::new(ExpressionNode::Decimal {
                                    position: ParsePosition {
                                        start: ScanPosition::new(23, 23, 0, 0, 0),
                                        end: ScanPosition::new(27, 27, 0, 0, 0)
                                    },
                                    value: 30 as DecimalType
                                }),
                                exponent: Box::new(ExpressionNode::Integer {
                                    position: ParsePosition {
                                        start: ScanPosition::new(28, 28, 0, 0, 0),
                                        end: ScanPosition::new(29, 29, 0, 0, 0)
                                    },
                                    value: 2 as IntegerType
                                }),
                            },
                            ExpressionNode::Decimal {
                                position: ParsePosition {
                                    start: ScanPosition::new(32, 32, 0, 0, 0),
                                    end: ScanPosition::new(36, 36, 0, 0, 0)
                                },
                                value: 78 as DecimalType
                            },
                        ),
                    }),
                },
            )
        }, result_node);
    }

}
#[cfg(test)]
mod evaluation_tests {
    use crate::{expression::{value::{DecimalType, ExpressionValue}, node::Evaluate}, scan::context::beginning};

    use super::*;

    #[test]
    fn test_evaluate_integer() {
        let s = "1234";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 1234 });
    }

    #[test]
    fn test_evaluate_negative_integer() {
        let s = "-1234";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: -1234 });
    }

    #[test]
    fn test_evaluate_decimal() {
        let s = "1234.0";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 1234 as DecimalType });
    }

    #[test]
    fn test_evaluate_negative_decimal() {
        let s = "-1234.0";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -1234 as DecimalType });
    }

    #[test]
    fn test_evaluate_scientific() {
        let s = "1234e0";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 1234 as DecimalType });
    }

    #[test]
    fn test_evaluate_negative_scientific() {
        let s = "-1234E0";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -1234 as DecimalType });
    }

    #[test]
    fn test_evaluate_parenthesis() {
        let s = "(1234)";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 1234 });
    }

    #[test]
    fn test_evaluate_negative_parenthesis() {
        let s = "-(1234)";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: -1234 });
    }

    #[test]
    fn test_evaluate_integer_sum() {
        let s = " 1 + 2 + 3 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 6 });
    }

    #[test]
    fn test_evaluate_decimal_sum() {
        let s = " 1 + 2 + 3.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 6 as DecimalType });
    }

    #[test]
    fn test_evaluate_integer_difference() {
        let s = " 1 - 2 - 3 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: -4 });
    }

    #[test]
    fn test_evaluate_decimal_difference() {
        let s = " 1 - 2 - 3.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -4 as DecimalType });
    }

    #[test]
    fn test_evaluate_integer_product() {
        let s = " 1 * 2 * 3 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 6 });
    }

    #[test]
    fn test_evaluate_decimal_product() {
        let s = " 1 * 2 * 3.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 6 as DecimalType });
    }

    #[test]
    fn test_evaluate_integer_quotient() {
        let s = " 3 / 2 / 1";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 1 });
    }

    #[test]
    fn test_evaluate_decimal_quotient() {
        let s = " 3.0 / 2 / 1.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 1.5 as DecimalType });
    }

    #[test]
    fn test_evaluate_divide_by_zero() {
        let s = " 3 / 0 / 1 ";
        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::NaN);

        let s = " 3.0 / 0.0 / 1.0 ";
        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::NaN);
    }

    #[test]
    fn test_evaluate_integer_power() {
        let s = " 3^2";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 9 });

        let s = " 3^0";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 1 });

        let s = " 3^-1";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: 0 });
    }

    #[test]
    fn test_evaluate_decimal_power() {
        let s = " 3.0^2 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 9 as DecimalType });

        let s = " 3.0^2.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 9 as DecimalType });

        let s = " 3^2.0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 9 as DecimalType });

        let s = " 3.0^0 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 1 as DecimalType });

        let s = " 2.0^-1 ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: 0.5 as DecimalType });
    }

    #[test]
    fn test_evaluate_integer_expression() {
        let s = " (((10 + 5) * -6) - -20 / -2 * 3 + -((5*2)^2) - (-5 * -2 * 5)) ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Integer { value: -270 });
    }

    #[test]
    fn test_evaluate_decimal_expression() {
        let s = " (((10 + 5) * -6) - -20.0 / -2 * 3 + -((5*2)^2) - (-5 * -2 * 5)) ";

        print_expression_result(s, beginning());
        let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
        assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -270 as DecimalType});
    }
}


use crate::scan::context::{
    ScanPosition,
    ScanContext,
    scan_one_or_more_chars,
    scan_literal,
    scan_zero_or_more_chars
};

use crate::expression::position::ParsePosition;
use crate::expression::error::ParsingError;

use super::expression::ExpressionNode;
use super::value::SignType;

/**
 * @author Ezward
 *
 * NOTE: this grammar separates out sums from differences and products from quotients.
 *       Thus, it is not a traditional factor/term grammar.  The grammar is
 *       designed to separate out operations that are subject to the associative
 *       and commutative properties with the notion that the parse tree can
 *       then be more easily queried or manipulated using those mathematical properties.
 *
 * Singleton class to parse, evaluate and pretty-print simple 4-operator expressions
 * that use the following PEG grammar;
 *
 * digit ::= [0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9]
 * sign ::= '-'
 * integer ::= {sign} [digit]*
 * decimal ::= {sign} [digit]* '.' [digit]*
 * scientific ::= {sign} [digit]* {'.' [digit]*} ['e' | 'E'] {sign} [digit]*
 * number ::= [integer | decimal | scientific]
 * parenthesis ::= {sign} '(' expression ')'
 * value ::= [parenthesis | number]
 * power ::= value{'^'value}
 * quotient ::= power {['÷' | '/'] power}*
 * product ::= quotient {['×' | '*']  quotient}*
 * difference ::= product  {'-' product}*
 * sum ::= difference {'+' difference}*
 * expression ::= sum
 *
 * Key to PEG notation:
 * {} = optional, choose zero or one
 * {}* = optional, 0 or more
 * [] = required, choose one
 * []* = required, 1 or more
 *
 * Usage:
 * 		ExpressionEvaluator.Expression theExpression = ExpressionEvaluator.parse("(((10 + 5) x -6) - -20 / -2 x 3 + -100 - 50)");
 *		if(null == theExpression) throw new RuntimeException("Parse error");
 *		double theValue = theExpression.evaluate();
 *		String thePrettyPrint = theExpression.format();
 *
 */

fn scan_whitespace(s: &str, context: ScanContext) -> ScanContext {
    scan_zero_or_more_chars(s, context, |ch| ch.is_ascii_whitespace())
}
fn scan_digits(s: &str, context: ScanContext) -> ScanContext {
    scan_one_or_more_chars(s, context, |ch| ch.is_ascii_digit())
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

#[derive(Debug, Clone, PartialEq)]
pub struct ParseNode {
    pub position: ParsePosition, // position in source
    pub value: Box<ExpressionNode>
}



fn parse_whitespace(s: &str, context: ScanContext) -> Result<ScanContext, ParsingError> {
    expect_match(s, context.1, scan_whitespace(s, context))
}


fn parse_number(s: &str, context: ScanContext) -> Result<(ScanContext, ParseNode), ParsingError> {
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
    let mut is_decimal = false;
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
    Ok(((true, position), ParseNode {
        position: ParsePosition::new(&start_position, &position),
        value: if (is_decimal || has_exponent) {
            Box::new(ExpressionNode::Decimal{
                value: s[start_position.byte_index..position.byte_index].parse::<f64>().map_err(|err| {
                    println!("Error converting decimal number at {:?}: {}", ParsePosition::new(&start_position, &position), &err);
                    ParsingError::Number(ParsePosition::new(&start_position, &position))
                })?
            })
        } else {
            // integer
            Box::new(ExpressionNode::Integer{
                value: s[start_position.byte_index..position.byte_index].parse::<i32>().map_err(|err| {
                    println!("Error converting integer at {:?}: {}", ParsePosition::new(&start_position, &position), &err);
                    ParsingError::Number(ParsePosition::new(&start_position, &position))
                })?
            })
        }
    }))
}

fn parse_value(s: &str, context: ScanContext) -> Result<(ScanContext, ParseNode), ParsingError> {
    //
    // skip any leading whitespace
    //
    let (mut matched, start_position) = parse_whitespace(s, context)?;

    //
    // parse the optional negation
    //
    let (is_negative, mut position) = scan_literal(s, (true, start_position), "-");

    //
    // scan opening brace
    //
    (matched, position) = scan_literal(s, (true, position), "(");
    if matched {
        //
        // TODO: temporarily just expect a number inside
        //
        let number_node: ParseNode;
        ((matched, position), number_node) = parse_number(s, (true, position))?;

        //
        // scan the required closing parenthesis
        //
        (matched, position) = expect_match(s, start_position, scan_literal(s, parse_whitespace(s, (true, position))?, ")"))?;

        Ok(((true, position), ParseNode {
            position: ParsePosition::new(&start_position, &position),
            value: Box::new(ExpressionNode::Parenthesis { sign: SignType::from(!is_negative), inner: number_node.value })
        }))

    } else {
        //
        // if it's not a parenthesis, then it must be a number.
        // start at the optional negation
        //
        parse_number(s, (true, start_position))
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
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Integer { value: 1234 })
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
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Decimal { value: 1234 as DecimalType })
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
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Decimal { value: 1234 as DecimalType })
        }, result_node);

        let s = "123.4E1";
        let start = ScanPosition::default();
        let context = (true, start);

        let (result_context, result_node) = parse_number(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len(), s.chars().count(), 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Decimal { value: 1234 as DecimalType })
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
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Parenthesis { sign: SignType::Positive, inner: Box::new(ExpressionNode::Integer { value: 1234 as IntegerType }) })
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
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Parenthesis { sign: SignType::Positive, inner: Box::new(ExpressionNode::Integer { value: -1234 as IntegerType }) })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_negative_decimal() {
        let s = " ( -1234.0 ) ";
        let start = ScanPosition::new(1, 1, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len()- 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Parenthesis { sign: SignType::Positive, inner: Box::new(ExpressionNode::Decimal { value: -1234 as DecimalType }) })
        }, result_node);
    }

    #[test]
    fn test_parse_parenthesis_negative() {
        let s = " -( 1234 ) ";
        let start = ScanPosition::new(1, 1, 0, 0, 0);
        let context = (true, start);

        let (result_context, result_node) = parse_value(s, context).unwrap();
        let expected_end = ScanPosition::new(s.len() - 1, s.chars().count() - 1, 0, 0, 0);
        assert_eq!((true, expected_end), result_context);
        assert_eq!(ParseNode{
            position: ParsePosition { start: start, end: expected_end },
            value: Box::new(ExpressionNode::Parenthesis { sign: SignType::Negative, inner: Box::new(ExpressionNode::Integer { value: 1234 as IntegerType }) })
        }, result_node);
    }


}
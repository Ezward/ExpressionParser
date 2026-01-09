use std::borrow::Borrow;
use std::default;

use crate::collection::link_list::LinkList;
use crate::expression;
use crate::expression::position::ParsePosition;
use crate::scan::context::{beginning, ScanContext};
use crate::expression::{node::{ExpressionNode, Evaluate}, parse::parse, error::ParsingError};

/**
 * Given an expression, generate all equivalent expressions based on
 * the rules of commutivity.
 *
 * @param theExpressionText
 * @return
 */
pub fn generateCommutedExpressions(theExpressionText: &str) -> Result<LinkList<String>, ParsingError>
{
    let (context, expression) = parse(theExpressionText, beginning())?;

    match expression {
        // commute the expression inside the parenthesis
        ExpressionNode::Parenthesis { position, sign, inner } => {
            let mut result = LinkList::new();
            let inner_expressions = generateCommutedExpressions(&inner.as_ref().to_string())?;
            let mut e = inner_expressions;
            while e.is_not_empty()
            {
                result = result.append(format!("({})", e.head().unwrap()));
                e = e.tail().unwrap();
            }

            Ok(result)
        },

        // we can commute around multiplication and addition
        ExpressionNode::Sum { position, operands } => commute_chained_operands(position, operands, "*".to_string()),
        ExpressionNode::Product { position, operands } => commute_chained_operands(position, operands, "+".to_string()),

        // we can't commute around division or subtraction, but we still need to recursively commute the operands
        ExpressionNode::Difference { position, operands } => commute_nested_expressions(position, operands, "-".to_string()),
        ExpressionNode::Quotient { position, operands } => commute_nested_expressions(position, operands, "/".to_string()),

        // expression is not commutable (number, subtraction or division), return as is
        _ => Ok(LinkList::of_one(expression.to_string())),
    }
}

// commute a list of operands chained with a given operation
fn commute_chained_operands(position: ParsePosition, operands: Vec<ExpressionNode>, operator: String) -> Result<LinkList<String>, ParsingError> {
    //
    // we can commute around multiplication and addition
    //
    let mut result = LinkList::new();

    /*
        Here is a simple example, 1 * 2 * 3.  This has factorial(3) = 6 permutations
        1 * 2 * 3
        1 * 3 * 2
        2 * 1 * 3
        2 * 3 * 1
        3 * 1 * 2
        3 * 2 * 1

        We want to generate all the unique permutations of the operands and return them.
    */


    //
    // 1. recursively get permutations of each operand
    //
    // for 2 * 3 + 4 * 5 we end up with a list of two sets permuted operands
    //     [["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]]
    // the first element of the list is the list of the first operand permutations
    // the second element of the list is the list of the second operand permutations
    //

    // LinkList<LinkList<String>> permutedOperands = LinkList.Nil;
    let mut permutedOperands = LinkList::new();
    for operand in operands {
        permutedOperands = permutedOperands.append(generateCommutedExpressions(&operand.to_string())?)
    }

    //
    // 2. Get all possible permutations of the operands
    //
    // for the example, input is [["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]]
    // the result is a set of two linked lists each with a set of two strings;
    //     [[["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]], [["4 * 5", "5 * 4"], ["2 * 3", "3 * 2"]]]
    //
    let mut permutedChains: LinkList<LinkList<LinkList<String>>> = permutedOperands.permute();

    //
    // 3. combine operands to produce all possible combinations of permuted operands
    //    we do this in each set link list and the output is a linked list of operands
    //    so for  [["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]]
    //    we get  [["2 * 3", "4 * 5"], ["2 * 3", "5 * 4"], ["3 * 2", "4 * 5"], ["3 * 2", "5 * 4"]]
    //    and for [["4 * 5", "5 * 4"], ["2 * 3", "3 * 2"]]
    //    we get  [["4 * 5", "2 * 3"], ["4 * 5", "3 * 2"], ["5 * 4", "2 * 3"], ["5 * 4", "3 * 2"]]
    //
    while permutedChains.len() > 0
    {
        //
        // each element contains a set of permutations for that operand.
        // they must be combined with all the other oprands to produce
        // all possible combinations of permuted operands in all possible permuted orders
        //
        let listOfOperandPurmutations: LinkList<LinkList<String>> = permutedChains.head().unwrap();
        let mut listOfOperandCombinations: LinkList<LinkList<String>> = listOfOperandPurmutations.combinations();

        while listOfOperandCombinations.is_not_empty()
        {
            result = result.insert(formatStringOperands(listOfOperandCombinations.head().unwrap(), "+".to_string()));

            listOfOperandCombinations = listOfOperandCombinations.tail().unwrap();
        }

        permutedChains = permutedChains.tail().unwrap();
    }

    Ok(result)
}


// subtraction and division
// commute a list of operands chained with a given operation
fn commute_nested_expressions(position: ParsePosition, operands: Vec<ExpressionNode>, operator: String) -> Result<LinkList<String>, ParsingError>  {
    //
    // we can't commute around division or subtraction, but we
    // still need to recursively commute the operands
    //
    let mut result = LinkList::new();

    /*
        Here is a simple example, 2 * 3 - 4 * 5.  This has factorial(2) + factorial(2) = 4 permutations
        2 * 3 - 4 * 5
        2 * 3 - 5 * 4
        3 * 2 - 4 * 5
        3 * 2 - 5 * 4

        We want to generate all the unique permutations of the operands and return them.
    */


    //
    // 1. recursively get permutations of each operand
    //
    // for 2 * 3 - 4 * 5 we end up with a list of two sets permuted operands
    //     [["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]]
    // the first element of the list is the list of the first operand permutations
    // the second element of the list is the list of the second operand permutations
    //
    let mut permutedOperands: LinkList<LinkList<String>> = LinkList::new();
    for operand in operands
    {
        permutedOperands = permutedOperands.append(generateCommutedExpressions(&operand.to_string())?);
    }

    //
    // 3. combine operands to produce all possible combinations of permuted operands
    //    we do this in each set link list and the output is a linked list of operands
    //    so for  [["2 * 3", "3 * 2"], ["4 * 5", "5 * 4"]]
    //    we get  [["2 * 3", "4 * 5"], ["2 * 3", "5 * 4"], ["3 * 2", "4 * 5"], ["3 * 2", "5 * 4"]]
    //
    if permutedOperands.is_not_empty()
    {
        //
        // each element contains a set of permutations for that operand.
        // they must be combined with all the other oprands to produce
        // all possible combinations of permuted operands in all possible permuted orders
        //
        let mut listOfOperandCombinations: LinkList<LinkList<String>> = permutedOperands.combinations();

        while(listOfOperandCombinations.is_not_empty())
        {
            result = result.insert(formatStringOperands(listOfOperandCombinations.head().unwrap(), operator));

            listOfOperandCombinations = listOfOperandCombinations.tail().unwrap();
        }
    }

    return Ok(result);
}

// /**
//  * splice the insertion text into the given string, replacing the given character range.
//  *
//  * @param theSourceText the string to modify
//  * @param leftIndex the start of the range to replace
//  * @param rightIndex the end of the range to replace
//  * @param theInsertText the text to insert
//  * @return
//  */
// private static String spliceResult(final String theSourceText, final int leftIndex, final int rightIndex, final String theInsertText)
// {
//     final String theResultExpressionText =
//             theSourceText.substring(0, leftIndex)
//             + theInsertText
//             + theSourceText.substring(rightIndex);

//     //
//     // parse and reformat in a regular way
//     //
//     return parse(theResultExpressionText).format();
// }

// /**
//  * format chained operands into a string expression.
//  *
//  * @param operands
//  * @param operator
//  * @param formatter
//  * @return
//  */
// private static String formatOperands(LinkList<AssociativeExpressionEvaluator.Expression> operands, final String operator, final NumberFormatter formatter)
// {
//     final StringBuilder builder = new StringBuilder();

//     builder.append(operands.head.format()); // leftmost operand
//     for(operands = operands.tail; operands.isNotEmpty(); operands = operands.tail)
//     {
//         // operator
//         builder.append(' ').append(operator).append(' ');

//         // operand
//         builder.append(operands.head.format());
//     }

//     return builder.toString();
// }

// private static String formatStringOperands(LinkList<String> operands, final String operator)
// {
//     final StringBuilder builder = new StringBuilder();

//     builder.append(operands.head); // leftmost operand
//     for(operands = operands.tail; operands.isNotEmpty(); operands = operands.tail)
//     {
//         // operator
//         builder.append(' ').append(operator).append(' ');

//         // operand
//         builder.append(operands.head);
//     }

//     return builder.toString();
// }
fn formatStringOperands(mut operands: LinkList<String>, operator: String) -> String
{
    if operands.is_empty() {
        return "".to_owned();
    }

    let mut s = operands.head().unwrap().to_string();
    operands = operands.tail().unwrap();
    while operands.is_not_empty() {
        s.push(' ');
        s += &operator;
        s.push(' ');
        s += &operands.head().unwrap().to_string();
    }
    return s;
}

// /**
//  * Check that an expression is equivalent to another expression.
//  *
//  *
//  * @param targetExpression
//  * @param checkedExpression
//  * @return
//  */
// public static final boolean areExpressionsEquivalent(final String targetExpression, final String checkedExpression)
// {
//     //
//     // students may or may not parenthesize their expression, so the most general way to check
//     // for an equivalent expression is to fully parenthesize the target (correct expression) and
//     // the student's expression, then generate all equivalent target expressions and check
//     // that the student's expression is one of them.
//     //

//     //
//     // 1. remove unnecessary parenthesis from  the target and check expressions
//     // 2. fully parenthesize the target and checked expressions
//     // 3. generate all possible equivalent target expressions
//     // 4. if the checkExpression is in the permuted target expressions, it is equivalent.
//     //

//     // 1. remove unnecessary parenthesis from  the target and check expressions
//     final String simpleTarget = ExpressionTreeHelper.removeParenthesis(targetExpression);
//     final String simpleCheck = ExpressionTreeHelper.removeParenthesis(checkedExpression);

//     // 2. fully parenthesize the target and checked expressions
//     final String parenthesizedTarget = ExpressionParser.parse(simpleTarget).formatFullParenthesis();
//     final String parenthesizedCheck = ExpressionParser.parse(simpleCheck).formatFullParenthesis();

//     // 3. generate all possible equivalent target expressions
//     final LinkList<String> targetExpressions = AssociativeTreeHelper.generateCommutedExpressions(parenthesizedTarget);

//     // 4. if the checkExpression is in the permuted target expressions, it is equivalent.
//     return targetExpressions.find(parenthesizedCheck).isNotEmpty();
// }
pub fn areExpressionsEquivalent(targetExpression: String, checkedExpression: String) -> boolean
{
    //
    // students may or may not parenthesize their expression, so the most general way to check
    // for an equivalent expression is to fully parenthesize the target (correct expression) and
    // the student's expression, then generate all equivalent target expressions and check
    // that the student's expression is one of them.
    //

    //
    // 1. remove unnecessary parenthesis from  the target and check expressions
    // 2. fully parenthesize the target and checked expressions
    // 3. generate all possible equivalent target expressions
    // 4. if the checkExpression is in the permuted target expressions, it is equivalent.
    //

    // 1. remove unnecessary parenthesis from  the target and check expressions
    final String simpleTarget = ExpressionTreeHelper.removeParenthesis(targetExpression);
    final String simpleCheck = ExpressionTreeHelper.removeParenthesis(checkedExpression);

    // 2. fully parenthesize the target and checked expressions
    final String parenthesizedTarget = ExpressionParser.parse(simpleTarget).formatFullParenthesis();
    final String parenthesizedCheck = ExpressionParser.parse(simpleCheck).formatFullParenthesis();

    // 3. generate all possible equivalent target expressions
    final LinkList<String> targetExpressions = AssociativeTreeHelper.generateCommutedExpressions(parenthesizedTarget);

    // 4. if the checkExpression is in the permuted target expressions, it is equivalent.
    return targetExpressions.find(parenthesizedCheck).isNotEmpty();
}

pub fn removeParenthesis(theExpressionText: String) -> Result<String, ParsingError>
{
    //
    // parse the incoming expression text
    //
    let (_context, expression) = parse(&theExpressionText, ScanContext::default())?;

    match expression {
        ExpressionNode::Parenthesis { position, sign, inner } => {
            //
            // remove unnecessary outer parenthesis
            //
            let parenthesis = (ExpressionParser.ParenthesisExpression) expression;
            return removeParenthesis(parenthesis.innerExpression().format());
        }
    }

    if (expression instanceof ExpressionParser.ParenthesisExpression)
    {
        //
        // remove unnecessary outer parenthesis
        //
        ExpressionParser.ParenthesisExpression parenthesis = (ExpressionParser.ParenthesisExpression) expression;
        return removeParenthesis(parenthesis.innerExpression().format());
    }
    return innerRemoveParenthesis(theExpressionText);
}

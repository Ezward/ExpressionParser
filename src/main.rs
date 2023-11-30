use scan::context::beginning;

use crate::parse::parse::parse_expression;
use crate::parse::expression::Evaluate;

pub mod scan;
pub mod parse;

fn main() {
    let s = std::env::args().nth(1).expect("no expression provided");
    match parse_expression(&s, beginning()) {
        Ok((_position, expression)) => {
            println!("{}", expression.evaluate())
        }
        Err(err) => {
            println!("Failure parsing expression {}", &err)
        }
    }
}

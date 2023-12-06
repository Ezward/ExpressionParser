use expression::{parse::parse, error::ParsingError};
use scan::context::beginning;

use crate::expression::node::{Position, Evaluate};

pub mod scan;
pub mod expression;

fn main() -> Result<(), ParsingError> {
    let s = std::env::args().nth(1).expect("no expression provided");
    match parse(&s, beginning()) {
        Ok((_position, expression)) => {
            println!("{}", expression.evaluate());
            Ok(())
        }
        Err(e) => {
            println!("{}", s);
            if e.position().end.char_index - e.position().start.char_index > 1 {
                println!("{}^{}", " ".repeat(e.position().start.char_index), "^".repeat(e.position().end.char_index - e.position().start.char_index - 1));
            } else {
                println!("{}^", " ".repeat(e.position().start.char_index));
            }
            println!("{}", e);
            Err(e)
        }
    }
}

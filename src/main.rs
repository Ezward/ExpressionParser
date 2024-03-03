use std::process;

use expression::{parse::parse, error::ParsingError};
use scan::context::beginning;

use crate::expression::node::{Evaluate, Position};

pub mod scan;
pub mod expression;
// pub mod commute;
// pub mod helpers;
pub mod collection;

fn main() -> Result<(), ParsingError> {
    if let Some(s) = std::env::args().nth(1) {
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
    } else {
        eprintln!(r#"Oops, no expression was provided.  Try "1 + 10^(2 * 3) * 5""#);
        process::exit(1)
    }
}

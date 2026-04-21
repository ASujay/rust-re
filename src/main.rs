/*
        
    so i want to do something like this
    re <regular expression> <file | directory(all the files in director)>
    
    
*/

mod lexer;
mod error;
mod parser;
mod token;
mod fa;

use std::{env, process};
use lexer::{RegExLexer};
use error::{RegExError};
use parser::{RegExParser};
use fa::{Nfa};

fn main() -> Result<(), RegExError> {
    // check the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: re <regular expression> <file | directory>");
        process::exit(0);
    }

    let mut lexer = RegExLexer::init(args[1].chars().collect());
    let tokens = lexer.emit_tokens();
    let  mut parser = RegExParser::new(tokens);
    let ast = parser.parse_expr()?;
    let mut nfa = Nfa::new();
    let fragment = nfa.build(&ast);
    Ok(())
}

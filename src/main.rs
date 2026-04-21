/*
        
    so i want to do something like this
    re <regular expression> <file | directory(all the files in director)>
    
    
*/


use std::{env, process};

#[derive(Debug, PartialEq)]
enum RegExTokenType {
    LeftParan,
    RightParan,
    Pipe,
    KleeneStar,
    Plus,
    Question,
    BSlash,
    Literal,
}

#[derive(Debug)]
struct RegExToken {
    token_type: RegExTokenType,
    x_pos: usize,
    literal: char,
}

impl RegExToken {
    fn new(token_type: RegExTokenType, x_pos: usize, literal: char) -> RegExToken {
        RegExToken {
            token_type,
            x_pos,
            literal,
        }
    }
}

struct RegExLexer {
    input_string: Vec<char>,
    start: usize,
    current: usize,
}

#[derive(Debug)]
enum RegExNode {
    Character(char),
    Concat(Box<RegExNode>, Box<RegExNode>),
    Alternation(Box<RegExNode>, Box<RegExNode>),
    Star(Box<RegExNode>),
    Plus(Box<RegExNode>),
    Question(Box<RegExNode>),
}

#[derive(Debug)]
enum RegExError {
    UnexpectedToken,
    ExpectedExpression,
    ExpectedRightParan,
}

struct RegExParser {
    tokens: Vec<RegExToken>,
    current: usize,
}

impl RegExParser {
    fn new(tokens: Vec<RegExToken>) -> RegExParser {
        RegExParser {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> Option<&RegExToken> {
        if !self.is_at_end() {
            let token = &self.tokens[self.current];
            self.current += 1;
            return Some(token);
        }
        None
    }

    fn peek(&self) -> Option<&RegExToken> {
        if !self.is_at_end() {
            return Some(&self.tokens[self.current]);
        }
        None
    }

    /*
            expr    →  term ('|' term)*
            term    →  factor factor*
            factor  →  atom ('*' | '+' | '?')?
            atom    →  CHAR | '(' expr ')'
    */

    fn parse_expr(&mut self) -> Result<RegExNode, RegExError> {
        let mut left_node = self.parse_term()?;
        loop {
            if let Some(tkn) = self.peek() && tkn.token_type == RegExTokenType::Pipe {
                // check if pipe is present
                _ = self.advance();
                let right_node = self.parse_term()?;
                left_node = RegExNode::Alternation(Box::new(left_node), Box::new(right_node));
            } else {
                break;
            }
        }

        Ok(left_node)
    }

    fn parse_term(&mut self) -> Result<RegExNode, RegExError> {
        let mut left = self.parse_factor()?;
        loop {
            if let Some(tkn) = self.peek() {
                match tkn.token_type {
                    RegExTokenType::LeftParan => {
                        let right = self.parse_factor()?;
                        left = RegExNode::Concat(Box::new(left), Box::new(right));
                    },
                    RegExTokenType::Literal => {
                        let right = self.parse_factor()?;
                        left = RegExNode::Concat(Box::new(left), Box::new(right));
                    },
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }
 
    fn parse_factor(&mut self) -> Result<RegExNode, RegExError> {
        let atom_node = self.parse_atom()?;
        if let Some(next) = self.peek() {
            match next.token_type {
                RegExTokenType::Plus => {
                    _ = self.advance();
                    Ok(RegExNode::Plus(Box::new(atom_node)))
    
                },
                RegExTokenType::KleeneStar => {
                    _ = self.advance();
                    Ok(RegExNode::Star(Box::new(atom_node)))
    
                },
                RegExTokenType:: Question => {
                    _ = self.advance();
                    Ok(RegExNode::Question(Box::new(atom_node)))
                },
                _ => Ok(atom_node),
            }    
        } else {
            Ok(atom_node)
        }
        
    }

    fn parse_atom(&mut self) -> Result<RegExNode, RegExError> {
        let token = self.peek().ok_or(RegExError::UnexpectedToken)?;
        match token.token_type {
            RegExTokenType::Literal => {
                let token = self.advance().ok_or(RegExError::UnexpectedToken)?;
                Ok(RegExNode::Character(token.literal))
            },
            RegExTokenType::LeftParan => {
                _ = self.advance();
                let expr_node = self.parse_expr()?;
                let close = self.advance().ok_or(RegExError::ExpectedRightParan)?;
                if close.token_type != RegExTokenType::RightParan {
                    return Err(RegExError::ExpectedRightParan);
                }
                Ok(expr_node)
            },
            _ => Err(RegExError::UnexpectedToken)
        }
    }
}

impl RegExLexer {
    fn init(input_string: Vec<char>) -> RegExLexer {
        RegExLexer {
            input_string,
            start: 0,
            current: 0,
        }
    }


    fn advance(&mut self) -> Option<char> {
        if self.current < self.input_string.len() {
            let chr = Some(self.input_string[self.current]);
            self.current += 1;
            return chr;
        }

        None
    }

    fn emit_token(&mut self, chr: char) -> RegExToken {
        return match chr {
            '(' => RegExToken::new(RegExTokenType::LeftParan, self.start, chr),
            ')' => RegExToken::new(RegExTokenType::RightParan, self.start, chr),
            '+' => RegExToken::new(RegExTokenType::Plus, self.start, chr),
            '*' => RegExToken::new(RegExTokenType::KleeneStar, self.start, chr),
            '|' => RegExToken::new(RegExTokenType::Pipe, self.start, chr),
            '?' => RegExToken::new(RegExTokenType::Question, self.start, chr),
            '\\' => RegExToken::new(RegExTokenType::BSlash, self.start, chr),
            _ => RegExToken::new(RegExTokenType::Literal, self.start, chr),
        };
    }

    fn emit_tokens(&mut self) -> Vec<RegExToken> {
        let mut tokens = Vec::<RegExToken>::new(); 
        while let Some(chr) = self.advance() {
            let token = self.emit_token(chr);
            tokens.push(token);
            self.start = self.current;
        }
        tokens
    }
}





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
    println!("{:?}", ast);
    Ok(())
}

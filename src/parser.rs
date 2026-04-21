use crate::lexer::{RegExLexer};
use crate::error::{RegExError};
use crate::token::{RegExToken, RegExTokenType};

#[derive(Debug)]
pub enum RegExNode {
    Character(char),
    Concat(Box<RegExNode>, Box<RegExNode>),
    Alternation(Box<RegExNode>, Box<RegExNode>),
    Star(Box<RegExNode>),
    Plus(Box<RegExNode>),
    Question(Box<RegExNode>),
}



pub struct RegExParser {
    pub tokens: Vec<RegExToken>,
    pub current: usize,
}

impl RegExParser {
    pub fn new(tokens: Vec<RegExToken>) -> RegExParser {
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

    pub fn parse_expr(&mut self) -> Result<RegExNode, RegExError> {
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
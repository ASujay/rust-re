use crate::token::{RegExToken, RegExTokenType};

pub struct RegExLexer {
    input_string: Vec<char>,
    start: usize,
    current: usize,
}

impl RegExLexer {
    pub fn init(input_string: Vec<char>) -> RegExLexer {
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

    pub fn emit_tokens(&mut self) -> Vec<RegExToken> {
        let mut tokens = Vec::<RegExToken>::new(); 
        while let Some(chr) = self.advance() {
            let token = self.emit_token(chr);
            tokens.push(token);
            self.start = self.current;
        }
        tokens
    }
}



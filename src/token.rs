#[derive(Debug, PartialEq)]
pub enum RegExTokenType {
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
pub struct RegExToken {
    pub token_type: RegExTokenType,
    pub x_pos: usize,
    pub literal: char,
}

impl RegExToken {
    pub fn new(token_type: RegExTokenType, x_pos: usize, literal: char) -> RegExToken {
        RegExToken {
            token_type,
            x_pos,
            literal,
        }
    }
}
use std::io;

#[derive(Debug)]
pub enum RegExError {
    UnexpectedToken,
    ExpectedExpression,
    ExpectedRightParan,
    CouldNotReadSearchFile,
}

impl From<io::Error> for RegExError {
    fn from(value: io::Error) -> Self {
        _ = value;
        RegExError::CouldNotReadSearchFile
    }
}
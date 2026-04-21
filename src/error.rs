#[derive(Debug)]
pub enum RegExError {
    UnexpectedToken,
    ExpectedExpression,
    ExpectedRightParan,
}
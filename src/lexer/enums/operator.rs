#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide, Modulo,
    Equal, LessThan, GreaterThan, LessThanEqual, GreaterThanEqual, NotEqual,
    And, Or,
}


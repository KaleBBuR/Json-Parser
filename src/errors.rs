use std::fmt;

#[derive(Debug)]
pub enum TokenError<'a> {
    UnexpectedValue(&'a str),
    ExpectedValue(Vec<&'a str>, usize, usize)
}

impl<'a> fmt::Display for TokenError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::UnexpectedValue(ref value) => write!(f, "Unexpected Value! `{}`", value),
            TokenError::ExpectedValue(ref expected_values, _line, _char) => {
                write!(f, "Expected Values: => {:?} LINE: {}, CHAR: {}", expected_values, _line, _char)
            }
        }
    }
}
use std::fmt;

#[derive(Debug)]
pub enum TokenError<'a> {
    UnexpectedValue(&'a str),
    ExpectedValue(Vec<&'a str>)
}

impl<'a> fmt::Display for TokenError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::UnexpectedValue(ref value) => write!(f, "Unexpected Value! `{}`", value),
            TokenError::ExpectedValue(ref expected_values) => {
                write!(f, "Expected Values: => {:?}", expected_values)
            }
        }
    }
}
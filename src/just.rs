//! # Parser
//!
//! A struct Parser which is used to take in the incoming str full of json data and convert it to a JSON type

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::json::JSON;

pub struct JUST<'a> {
    pub tokens: Box<dyn Iterator<Item = &'a Token> + 'a>,
}

#[derive(Debug, Clone)]
pub enum Token {
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    OpeningBracket,
    ClosingBracket,
    Colon,
    Comma,
    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Null
}

#[derive(Debug)]
pub enum TokenError {
    FloatTokenError(String),
    IntTokenError(String),
    ValueTokenError(String),
    UnknownChar(String),
    Msg(String)
}

pub trait Tokenize<'a>: Sized {
    type TokenVec;

    fn tokens(&mut self) -> Result<Self::TokenVec, TokenError>;

    fn tokenize_string(&mut self) -> Result<Token, TokenError>;
    fn tokenize_number(&mut self, first_char: char) -> Result<Token, TokenError>;
    fn tokenize_value(&mut self, first_char: char) -> Result<Token, TokenError>;
}

impl<'a> JUST<'a> {
    pub fn parse(&mut self) -> Result<JSON, TokenError> {
        match self.tokens.next() {
            Some(token) => {
                match token {
                    Token::OpeningCurlyBrace => Ok(JSON::Object(self.parse_object()?)),
                    Token::OpeningBracket => Ok(JSON::Array(self.parse_array()?)),
                    _ => Err(TokenError::Msg("Unexpected Start of File.\nShould be `{{` or `[`".to_string()))
                }
            },
            None => Err(TokenError::Msg("Could not tokenize JSON".to_string()))
        }
    }

    fn parse_array(&mut self) -> Result<Vec<JSON>, TokenError> {
        let mut arr= Vec::new();
        let mut next_val = vec!["arr value", "]"];

        while let Some(token) = self.tokens.next() {
            match token {
                Token::OpeningCurlyBrace =>
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                        arr.push(JSON::Object(self.parse_object()?))
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::ClosingCurlyBrace => return Err(TokenError::Msg("Unexpected Value `}`".to_string())),
                Token::ClosingBracket =>
                    if next_val.contains(&"]") {
                        break
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)));
                    },
                Token::OpeningBracket => {
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::Array(self.parse_array()?))
                },
                Token::String(ref string_val) => {
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::String(string_val.to_owned()))
                },
                Token::Boolean(bool_val) => {
                    if next_val.contains(&"arr_value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::Boolean(*bool_val))
                },
                Token::Integer(int_val) => {
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::Integer(*int_val))
                },
                Token::Float(float_val) => {
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::Float(*float_val))
                },
                Token::Null => {
                    if next_val.contains(&"arr value") {
                        next_val = vec![",", "]"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                    arr.push(JSON::Null)
                },
                Token::Comma => {
                    if next_val.contains(&",") {
                        next_val = vec!["arr value"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
                },
                Token::Colon => return Err(TokenError::Msg("Unexpected Value -> `:`".to_string()))
            };
        }

        Ok(arr)
    }

    fn parse_object(&mut self) -> Result<HashMap<String, JSON>, TokenError> {
        let mut object: HashMap<String, JSON> = HashMap::new();
        let mut current_keyword = String::new();
        let mut next_val = vec!["obj keyword", "}"];

        while let Some(token) = self.tokens.next() {
            match token {
                Token::OpeningCurlyBrace =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Object(self.parse_object()?));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::ClosingCurlyBrace =>
                    if next_val.contains(&"}") {
                        break
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::ClosingBracket => return Err(TokenError::Msg("Unexpected Value -> `}`".to_string())),
                Token::OpeningBracket =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Array(self.parse_array()?));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::String(string_val) =>
                    if next_val.contains(&"obj keyword") {
                        next_val = vec![":"];
                        current_keyword = string_val.to_owned()
                    } else if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::String(string_val.to_owned()));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Boolean(bool_val) =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Boolean(*bool_val));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Integer(int_val) =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Integer(*int_val));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Float(float_val) =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Float(*float_val));
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Null =>
                    if next_val.contains(&"obj value") {
                        next_val = vec![",", "}"];
                        object.insert(current_keyword.clone(), JSON::Null);
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Comma =>
                    if next_val.contains(&",") {
                        next_val = vec!["obj keyword"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    },
                Token::Colon =>
                    if next_val.contains(&":") {
                        next_val = vec!["obj value"];
                    } else {
                        return Err(TokenError::Msg(format!("Expected Value -> `{:?}`", next_val)))
                    }
            }
        }

        Ok(object)
    }
}

impl<'a> Tokenize<'a> for Peekable<Chars<'a>> {
    type TokenVec = Vec<Token>;

    fn tokens(&mut self) -> Result<Self::TokenVec, TokenError> {
        let mut token_vec: Vec<Token> = Vec::new();

        'tokenizer: while let Some(character) = self.next() {
            match character {
                '{' => token_vec.push(Token::OpeningCurlyBrace),
                '}' => token_vec.push(Token::ClosingCurlyBrace),
                '[' => token_vec.push(Token::OpeningBracket),
                ']' => token_vec.push(Token::ClosingBracket),
                ',' => token_vec.push(Token::Comma),
                ':' => token_vec.push(Token::Colon),
                '"' => token_vec.push(self.tokenize_string()?),
                '0'..='9' => token_vec.push(self.tokenize_number(character)?),
                'a'..='z' => token_vec.push(self.tokenize_value(character)?),
                _ =>
                    if character.is_whitespace() {
                        continue 'tokenizer
                    } else {
                        return Err(TokenError::UnknownChar(character.to_string()))
                    }
            };
        }

        Ok(token_vec.clone())
    }

    fn tokenize_number(&mut self, first_char: char) -> Result<Token, TokenError> {
        let mut number: String = first_char.to_string();
        let mut is_float = false;
        while let Some(character) = self.peek() {
            match character {
                '0'..='9' => number.push(*character),
                '.' => { is_float = true; number.push(*character); },
                _ => break
            }
            self.next();
        }

        match is_float {
            true => {
                let float = number.parse::<f64>();
                match float {
                    Ok(num) => Ok(Token::Float(num)),
                    Err(err) => Err(TokenError::FloatTokenError(err.to_string()))
                }
            },
            false => {
                let int = number.parse::<i64>();
                match int {
                    Ok(num) => Ok(Token::Integer(num)),
                    Err(err) => Err(TokenError::IntTokenError(err.to_string()))
                }
            },
        }
    }

    fn tokenize_string(&mut self) -> Result<Token, TokenError> {
        let mut string: String = String::new();
        let mut slash = false;
        while let Some(character) = self.next() {
            match character {
                '"' => {
                    match slash {
                        true => { slash = false; string.push(character); },
                        false => break
                    }
                },
                '\\' => {
                    string.push(character);
                    slash = true;
                },
                _ => string.push(character)
            };
        }

        Ok(Token::String(string))
    }

    fn tokenize_value(&mut self, first_char: char) -> Result<Token, TokenError> {
        let mut value: String = first_char.to_string();
        while let Some(character) = self.peek() {
            match character {
                'a'..='z' => value.push(*character),
                _ => break
            }
        }

        match value.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Err(TokenError::ValueTokenError(value))
        }
    }
}
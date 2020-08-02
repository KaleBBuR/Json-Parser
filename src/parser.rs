use std::collections::HashMap;

use crate::tokenizer::{Tokenizer, Tokens};
use crate::errors::TokenError;

#[derive(Clone, Debug, PartialEq)]
pub enum JSON {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<JSON>),
    Object(HashMap<String, JSON>),
    Null
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(json: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(json),
        }
    }

    pub fn parse(&mut self) -> JSON {
        match self.tokenizer.next().expect("No value!") {
            Tokens::OpeningCurlyBrace => JSON::Object(self.parse_object()),
            Tokens::OpeningBracket => JSON::Array(self.parse_array()),
            _ => panic!("Unexpected Beginning of file. Expected '{{' or '[' ")
        }
    }

    fn step(&mut self) -> Option<Tokens> {
        let step = self.tokenizer.next();
        if step.is_some() {
            Some(step.unwrap())
        } else {
            None
        }
    }

    fn parse_array(&mut self) -> Vec<JSON> {
        let mut arr= Vec::new();
        let mut next_val = vec!["arr value", "]"];

        'arr: loop {
            match self.step() {
                Some(token) => {
                    match token {
                        Tokens::OpeningCurlyBrace => if next_val.contains(&"arr value") {
                            next_val = vec![",", "]"];
                            arr.push(JSON::Object(self.parse_object()))
                        } else {
                            panic!("{}", TokenError::ExpectedValue(next_val));
                        },
                        Tokens::ClosingCurlyBrace => panic!("{}", TokenError::UnexpectedValue(&"}")),
                        Tokens::ClosingBracket => if next_val.contains(&"]") { break 'arr } else { panic!("{}", TokenError::ExpectedValue(next_val)); },
                        Tokens::OpeningBracket => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::Array(self.parse_array()))
                        },
                        Tokens::String(ref string_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::String(string_val.to_owned()))
                        },
                        Tokens::Boolean(bool_val) => {
                            if next_val.contains(&"arr_value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::Boolean(bool_val))
                        },
                        Tokens::Integer(int_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::Integer(int_val))
                        },
                        Tokens::Float(float_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::Float(float_val))
                        },
                        Tokens::Null => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            arr.push(JSON::Null)
                        },
                        Tokens::Comma => {
                            if next_val.contains(&",") { next_val = vec!["arr value"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                            continue 'arr
                        },
                        Tokens::Colon => panic!("{}", TokenError::UnexpectedValue(":"))
                    }
                },
                None => break 'arr
            }
        }

        arr
    }

    fn parse_object(&mut self) -> HashMap<String, JSON> {
        let mut object: HashMap<String, JSON> = HashMap::new();
        let mut current_keyword = String::new();
        let mut next_val = vec!["obj keyword", "}"];

        'obj: loop {
            match self.step() {
                Some(token) => {
                    match token {
                        Tokens::OpeningCurlyBrace =>  {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Object(self.parse_object()));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::ClosingCurlyBrace => if next_val.contains(&"}") { break 'obj } else { panic!(TokenError::ExpectedValue(next_val)); },
                        Tokens::ClosingBracket => panic!("{}", TokenError::UnexpectedValue(&"]")),
                        Tokens::OpeningBracket => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Array(self.parse_array()));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::String(ref string_val) => {
                            if next_val.contains(&"obj keyword") {
                                next_val = vec![":"];
                                current_keyword = string_val.to_owned();
                                continue 'obj;
                            }

                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::String(string_val.to_owned()));
                                continue 'obj;
                            }

                            panic!("{}", TokenError::ExpectedValue(next_val));
                        },
                        Tokens::Boolean(bool_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Boolean(bool_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::Integer(int_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Integer(int_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::Float(float_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Float(float_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::Null => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Null);
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val));
                            }
                        },
                        Tokens::Comma => {
                            if next_val.contains(&",") { next_val = vec!["obj keyword"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                        },
                        Tokens::Colon => if next_val.contains(&":") { next_val = vec!["obj value"]; } else { panic!("{}", TokenError::ExpectedValue(next_val)); }
                    }
                },
                None => break 'obj
            }
        }

        object
    }
}
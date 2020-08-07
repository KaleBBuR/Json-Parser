//! # Parser
//!
//! A struct Parser which is used to take in the incoming str full of json data and convert it to a JSON type

use std::collections::HashMap;

use crate::tokenizer::{Tokenizer, Tokens};
use crate::errors::TokenError;
use crate::json::JSON;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    /// This is how you would get a new parser! To let you start parsing your json data. You need it to be a str, not a String
    ///
    /// # Example
    ///
    /// ```
    /// let mut file = File::open("json_file").unwrap();
    /// let mut contents = String::new();
    /// file.read_to_string(&mut contents).unwrap();
    /// let parser = parser::Parser::new(contents.as_str())
    /// ```
    pub fn new(json: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(json),
        }
    }

    /// With the parser you can now use the parse method, which finally turn your json string into usable json data!
    /// This also returns to a JSON Enum, which will have all your values
    ///
    /// # Example
    ///
    /// Json file
    /// ```
    /// {
    ///     "colors": [
    ///         {
    ///             "color": "black",
    ///             "category": "hue",
    ///             "type": "primary",
    ///             "code": {
    ///                 "rgba": [255,255,255,1],
    ///                 "hex": "#000"
    ///             }
    ///         },
    ///         {
    ///             "color": "white",
    ///             "category": "value",
    ///             "code": {
    ///                 "rgba": [0,0,0,1],
    ///                 "hex": "#FFF"
    ///             }
    ///         },
    ///         {
    ///             "color": "red",
    ///             "category": "hue",
    ///             "type": "primary",
    ///             "code": {
    ///                 "rgba": [255,0,0,1],
    ///                 "hex": "#FF0"
    ///             }
    ///         },
    ///         {
    ///             "color": "blue",
    ///             "category": "hue",
    ///             "type": "primary",
    ///             "code": {
    ///                 "rgba": [0,0,255,1],
    ///                 "hex": "#00F"
    ///             }
    ///         },
    ///         {
    ///             "color": "yellow",
    ///             "category": "hue",
    ///             "type": "primary",
    ///             "code": {
    ///                 "rgba": [255,255,0,1],
    ///                 "hex": "#FF0"
    ///             }
    ///         },
    ///         {
    ///             "color": "green",
    ///             "category": "hue",
    ///             "type": "secondary",
    ///             "code": {
    ///                 "rgba": [0,255,0,1],
    ///                 "hex": "#0F0"
    ///             }
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// ```
    /// let mut file = File::open("src/file.json").unwrap();
    /// let mut contents = String::new();
    /// file.read_to_string(&mut contents).unwrap();
    /// let json = parser::Parser::new(contents.as_str()).parse();
    /// assert_eq!(json,
    ///     object!{
    ///         "colors" => array![
    ///             object!{
    ///                 "color" => "black",
    ///                 "category" => "hue",
    ///                 "type" => "primary",
    ///                 "code" => object!{
    ///                     "rgba" => array![255, 255, 255, 1],
    ///                     "hex" => "#000"
    ///                 }
    ///             },
    ///             object!{
    ///                 "color" => "white",
    ///                 "category" => "value",
    ///                 "code" => object!{
    ///                     "rgba" => array![0, 0, 0, 1],
    ///                     "hex" => "#FFF"
    ///                 }
    ///             },
    ///             object!{
    ///                 "color" => "red",
    ///                 "category" => "hue",
    ///                 "type" => "primary",
    ///                 "code" => object!{
    ///                     "rgba" => array![255, 0, 0, 1],
    ///                     "hex" => "#FF0"
    ///                 }
    ///             },
    ///             object!{
    ///                 "color" => "blue",
    ///                 "category" => "hue",
    ///                 "type" => "primary",
    ///                 "code" => object!{
    ///                     "rgba" => array![0, 0, 255, 1],
    ///                     "hex" => "#00F"
    ///                 }
    ///             },
    ///             object!{
    ///                 "color" => "yellow",
    ///                 "category" => "hue",
    ///                 "type" => "primary",
    ///                 "code" => object!{
    ///                     "rgba" => array![255, 255, 0, 1],
    ///                     "hex" => "#FF0"
    ///                 }
    ///             },
    ///             object!{
    ///                 "color" => "green",
    ///                 "category" => "hue",
    ///                 "type" => "secondary",
    ///                 "code" => object!{
    ///                     "rgba" => array![0, 255, 0, 1],
    ///                     "hex" => "#0F0"
    ///                 }
    ///             }
    ///         ]
    ///     }
    /// );
    /// ```
    pub fn parse(&mut self) -> JSON {
        let token_data = self.tokenizer.next().expect("No value!");
        match token_data.0 {
            Tokens::OpeningCurlyBrace => JSON::Object(self.parse_object()),
            Tokens::OpeningBracket => JSON::Array(self.parse_array()),
            _ => panic!("Unexpected Beginning of file. Expected '{{' or '[' ")
        }
    }

    fn step(&mut self) -> Option<(Tokens, usize, usize)> {
        let token_data = self.tokenizer.next();
        if token_data.is_some() {
            Some(token_data.unwrap())
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
                    match token.0 {
                        Tokens::OpeningCurlyBrace => if next_val.contains(&"arr value") {
                            next_val = vec![",", "]"];
                            arr.push(JSON::Object(self.parse_object()))
                        } else {
                            panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                        },
                        Tokens::ClosingCurlyBrace => panic!("{}", TokenError::UnexpectedValue(&"}")),
                        Tokens::ClosingBracket => if next_val.contains(&"]") { break 'arr } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); },
                        Tokens::OpeningBracket => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::Array(self.parse_array()))
                        },
                        Tokens::String(ref string_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::String(string_val.to_owned()))
                        },
                        Tokens::Boolean(bool_val) => {
                            if next_val.contains(&"arr_value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::Boolean(bool_val))
                        },
                        Tokens::Integer(int_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::Integer(int_val))
                        },
                        Tokens::Float(float_val) => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::Float(float_val))
                        },
                        Tokens::Null => {
                            if next_val.contains(&"arr value") { next_val = vec![",", "]"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                            arr.push(JSON::Null)
                        },
                        Tokens::Comma => {
                            if next_val.contains(&",") { next_val = vec!["arr value"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
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
                    match token.0 {
                        Tokens::OpeningCurlyBrace =>  {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Object(self.parse_object()));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                            }
                        },
                        Tokens::ClosingCurlyBrace => if next_val.contains(&"}") { break 'obj } else { panic!(TokenError::ExpectedValue(next_val, token.1, token.2)); },
                        Tokens::ClosingBracket => panic!("{}", TokenError::UnexpectedValue(&"]")),
                        Tokens::OpeningBracket => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Array(self.parse_array()));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
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

                            panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                        },
                        Tokens::Boolean(bool_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Boolean(bool_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                            }
                        },
                        Tokens::Integer(int_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Integer(int_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                            }
                        },
                        Tokens::Float(float_val) => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Float(float_val));
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                            }
                        },
                        Tokens::Null => {
                            if next_val.contains(&"obj value") {
                                next_val = vec![",", "}"];
                                object.insert(current_keyword.clone(), JSON::Null);
                            } else {
                                panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2));
                            }
                        },
                        Tokens::Comma => {
                            if next_val.contains(&",") { next_val = vec!["obj keyword"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                        },
                        Tokens::Colon => if next_val.contains(&":") { next_val = vec!["obj value"]; } else { panic!("{}", TokenError::ExpectedValue(next_val, token.1, token.2)); }
                    }
                },
                None => break 'obj
            }
        }

        object
    }
}
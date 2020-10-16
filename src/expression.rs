use std::iter::Peekable;
use std::str::Chars;

use crate::json::JSON;

#[doc(hidden)]
pub struct ParseExpression<'a> {
    parse: Expressions<'a>,
    json: &'a JSON,
}

impl<'a> ParseExpression<'a> {
    pub fn new(expression: &'a str, json: &'a JSON) -> Self {
        Self {
            parse: Expressions::new(expression),
            json
        }
    }

    pub fn parse_expression(&mut self) -> JSON {
        let mut hashtag = false;
        let mut arr: Vec<JSON> = Vec::new();
        let mut json = self.json.to_owned();
        'expression_parse: while let Some(expression) = self.step() {
            match expression {
                Expression::Dot => continue 'expression_parse,
                Expression::Key(ref key) => {
                    match hashtag {
                        true => {
                            let mut return_arr: Vec<JSON> = Vec::new();
                            for obj in arr.iter() {
                                match obj.get_obj() {
                                    Some(ref json_obj) => {
                                        return_arr.push(json_obj[key].clone());
                                    },
                                    None => panic!("Expected Object for `{}`!", key)
                                }
                            }

                            match self.parse.expression.peek() {
                                Some(ref item) => {
                                    match item {
                                        '.' => {
                                            arr = return_arr;
                                            continue 'expression_parse
                                        },
                                        _ => panic!("Expected `.` after `{}`", key)
                                    }
                                },
                                None => return JSON::Array(return_arr)
                            }
                        },
                        false => {
                            json = json[key.clone()].clone();
                            match self.parse.expression.peek() {
                                Some(ref item) => {
                                    match item {
                                        '.' => continue 'expression_parse,
                                        _ => panic!("Expected `.` after `{}`", key)
                                    }
                                },
                                None => return json
                            }
                        }
                    }
                },
                Expression::Index(ref index) => {
                    json = json[index.clone()].clone();
                    match self.parse.expression.peek() {
                        Some(ref item) => {
                            match item {
                                '.' => continue 'expression_parse,
                                _ => panic!("Expected `.` after `{}`", index)
                            }
                        },
                        None => return json
                    }
                },
                Expression::Hashtag => {
                    match self.parse.expression.peek() {
                        Some(ref item) => {
                            match item {
                                '.' => {
                                    hashtag = true;
                                    match json.get_arr() {
                                        Some(json_arr) => arr = json_arr,
                                        None => panic!("Expected Array!")
                                    };
                                    self.step();
                                    continue 'expression_parse
                                },
                                _ => panic!("Expected `.` after #")
                            }
                        },
                        None => {
                            match json.get_arr() {
                                Some(arr) => return JSON::Integer(arr.len() as i64),
                                None => panic!("Expected Array!")
                            }
                        }
                    }
                },
                Expression::Query(ref query) => {
                    let mut comparison: &str = "";
                    let mut query_return_arr: Vec<JSON> = Vec::new();
                    if *query.last().unwrap() == Expression::Hashtag {
                        match json.get_arr() {
                            Some(json_arr) => {
                                arr = json_arr.clone();
                                query_return_arr = json_arr.clone();
                            },
                            None => panic!("Expected array for query!")
                        }
                        hashtag = true;
                    }

                    for (i, expression) in query.iter().enumerate() {
                        match *expression {
                            Expression::Dot | Expression::Hashtag | Expression::OpenParantheses | Expression::ClosingParantheses => continue,
                            Expression::Key(ref key) => {
                                eprintln!("KEY: {}", key);
                                match hashtag {
                                    true => {
                                        let mut return_arr: Vec<JSON> = Vec::new();
                                        eprintln!("ARR: {:?}", arr);
                                        for obj in arr.iter() {
                                            match obj.get_obj() {
                                                Some(ref json_obj) => {
                                                    return_arr.push(json_obj[key].clone());
                                                },
                                                None => panic!("Expected Object for `{}`!", key)
                                            }
                                        }

                                        match query.get(i+1) {
                                            Some(ref item) => {
                                                match item {
                                                    Expression::Dot | Expression::EqualTo | Expression::NotEqualTo | Expression::GreaterThan | Expression::GreaterThanEqual | Expression::LessThan | Expression::LessThanEqual => {
                                                        arr = return_arr;
                                                        continue
                                                    },
                                                    _ => panic!("Unxpected character: `{:?}` after `{}`", item, key)
                                                }
                                            },
                                            None => panic!("Expected more after `{}`!", key)
                                        }
                                    },
                                    false => {
                                        json = json[key.clone()].clone();
                                    }
                                }
                            },
                            Expression::Index(ref index) => {
                                json = json[index.clone()].clone();
                            },
                            Expression::GreaterThan => {
                                comparison = ">";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` after `>`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` after `>`")
                                }
                            },
                            Expression::GreaterThanEqual => {
                                comparison = ">=";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` after `>=`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` after `>=`")
                                }
                            },
                            Expression::LessThan => {
                                comparison = "<";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` after `<`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` after `<`")
                                }
                            },
                            Expression::LessThanEqual => {
                                comparison = "<=";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` after `<=`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` after `<=`")
                                }
                            },
                            Expression::EqualTo => {
                                comparison = "==";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) | Expression::String(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` or `String` after `==`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` or `String` after `==`")
                                }
                            },
                            Expression::NotEqualTo => {
                                comparison = "!=";
                                match query.get(i+1) {
                                    Some(ref item) => {
                                        match item {
                                            Expression::Float(_) | Expression::Int(_) | Expression::String(_) => continue,
                                            _ => panic!("Expected `Int` or `Float` or `String` after `==`")
                                        }
                                    },
                                    None => panic!("Expected `Int` or `Float` or `String` after `==`")
                                }
                            },
                            Expression::Float(ref float_val) => {
                                let mut return_arr = Vec::new();
                                match comparison {
                                    "==" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val == gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "!=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val != gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "<=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val <= gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "<" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val < gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    ">" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val > gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    ">=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_float() {
                                                Some(gotten_float) => {
                                                    if *float_val >= gotten_float {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    _ => panic!("Unknown comparison! `{}`", comparison)
                                }

                                arr = return_arr;
                            },
                            Expression::Int(ref int_val) => {
                                let mut return_arr = Vec::new();
                                match comparison {
                                    "==" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val == gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "!=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val != gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "<=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val <= gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    "<" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val < gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    ">" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val > gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    ">=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_int() {
                                                Some(gotten_int) => {
                                                    if *int_val >= gotten_int {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected Integer!")
                                            }
                                        }
                                    },
                                    _ => panic!("Unknown comparison! `{}`", comparison)
                                }

                                arr = return_arr;
                            },
                            Expression::String(ref string_val) => {
                                let mut return_arr = Vec::new();
                                match comparison {
                                    "==" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_string() {
                                                Some(gotten_string) => {
                                                    if *string_val == gotten_string {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected String!")
                                            }
                                        }
                                    },
                                    "!=" => {
                                        for (i, item) in arr.iter().enumerate() {
                                            match item.get_string() {
                                                Some(gotten_string) => {
                                                    if *string_val != gotten_string {
                                                        return_arr.push(query_return_arr[i].clone());
                                                    }
                                                },
                                                None => panic!("Expected String!")
                                            }
                                        }
                                    },
                                    _ => panic!("Unexpected comparison! `{}`", comparison)
                                }

                                arr = return_arr;
                            }
                            _ => panic!("Unknown expression -> `{:?}`", expression)
                        }
                    }
                }
                _ => panic!("Unknown expression -> `{:?}`", expression)
            }
        }

        json
    }

    #[inline]
    fn step(&mut self) -> Option<Expression> {
        let expression_type = self.parse.next();
        if expression_type.is_some() {
            Some(expression_type.unwrap())
        } else {
            None
        }
    }
}

#[doc(hidden)]
struct Expressions<'a> {
    expression: Peekable<Chars<'a>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[doc(hidden)]
pub enum Expression {
    Hashtag,
    EqualTo,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    OpenParantheses,
    ClosingParantheses,
    Dot,
    Query(Vec<Expression>),
    Key(String),
    Index(usize),
    String(String),
    Int(i64),
    Float(f64),
}

impl<'a> Expressions<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            expression: expression.chars().peekable(),
        }
    }

    fn get_index(&mut self, first_char: char) -> Expression {
        let mut index = String::new();
        index.push(first_char);
        'index: while let Some(character) = self.expression.peek() {
            match character {
                '0'..='9' => {
                    index.push(*character);
                    self.expression.next();
                },
                _ => break 'index
            }
        }

        let index = index.parse::<isize>().unwrap();
        if index < 0 {
            panic!("Index must be greater than or equal to Zero!")
        }

        Expression::Index(index as usize)
    }

    fn get_key(&mut self, first_char: char) -> Expression {
        let mut key = String::new();
        key.push(first_char);
        'key: while let Some(character) = self.expression.peek() {
            match character {
                'a'..='z' | 'A'..='Z' => {
                    key.push(*character);
                    self.expression.next();
                },
                _ => break 'key
            }
        }

        Expression::Key(key)
    }

    fn get_string(&mut self) -> Expression {
        let mut slash =  false;
        let mut value = String::new();

        'string: while let Some(character) = self.expression.next() {
            match character {
                '\\' => {
                    value.push(character);
                    slash = true;
                },
                '\'' => {
                    if slash {
                        value.push(character);
                        slash = false;
                        continue 'string;
                    }

                    break 'string
                },
                _ => {
                    value.push(character);
                    slash = false;
                }
            }
        }

        Expression::String(value)
    }

    fn get_number(&mut self, first_char: char) -> Expression {
        let mut number = String::new();

        number.push(first_char);
        'number: while let Some(character) = self.expression.peek() {
            match character {
                '0'..='9'| '.' => {
                    number.push(*character);
                    self.expression.next();
                },
                _ => break 'number ,
            }
        }

        match number.contains(".") {
            true => Expression::Float(number.parse::<f64>().unwrap()),
            false => Expression::Int(number.parse::<i64>().unwrap())
        }
    }

    fn parse_hash(&mut self) -> Option<Expression> {
        let future_char = self.expression.peek();
        match future_char {
            Some(item) => {
                match item {
                    '(' => {},
                    '.' => return None,
                    _ => panic!("Unexpected Character! -> `{}`", item)
                };
            },
            None => return Some(Expression::Hashtag)
        };

        let mut query: Vec<Expression> = Vec::new();
        query.push(Expression::Hashtag);
        'query: while let Some(character) = self.expression.next() {
            match character {
                '.' => query.push(Expression::Dot),
                '(' => query.push(Expression::OpenParantheses),
                ')' => {
                    query.push(Expression::ClosingParantheses);
                    let next_char = self.expression.peek();
                    match next_char {
                        Some(item) => {
                            match item {
                                '#' => {
                                    query.push(Expression::Hashtag);
                                    self.expression.next();
                                    break 'query
                                },
                                '.' => break 'query,
                                _ => panic!("Unexpected Character! -> `{}`", item)
                            }
                        },
                        None => break 'query
                    }
                },
                'a'..='z' => query.push(self.get_key(character)),
                '0'..='9' => query.push(self.get_number(character)),
                '\'' => query.push(self.get_string()),
                '#' => query.push(Expression::Hashtag),
                '=' => {
                    let next_char = self.expression.peek();
                    match next_char {
                        Some(item) => {
                            match item {
                                '=' => {
                                    query.push(Expression::EqualTo);
                                    self.expression.next();
                                },
                                _ => panic!("Unexpected Character! -> `{}`", item)
                            }
                        },
                        None => panic!("Should be more after {}", character)
                    }
                },
                '!' => {
                    let next_char = self.expression.peek();
                    match next_char {
                        Some(item) => {
                            match item {
                                '=' => {
                                    query.push(Expression::NotEqualTo);
                                    self.expression.next();
                                },
                                _ => panic!("Unexpected Character! -> `{}`", item)
                            }
                        },
                        None => panic!("Should be more after {}", character)
                    }
                },
                '<' => {
                    let next_char = self.expression.peek();
                    match next_char {
                        Some(item) => {
                            match item {
                                '=' => {
                                    query.push(Expression::LessThanEqual);
                                    self.expression.next();
                                },
                                '0'..='9' => {
                                    query.push(Expression::LessThan);
                                    continue 'query
                                },
                                _ => panic!("Unexpected Character! -> `{}`", item)
                            }
                        },
                        None => panic!("Should be more after {}", character)
                    }
                },
                '>' => {
                    let next_char = self.expression.peek();
                    match next_char {
                        Some(item) => {
                            match item {
                                '=' => {
                                    query.push(Expression::GreaterThanEqual);
                                    self.expression.next();
                                },
                                '0'..='9' => {
                                    query.push(Expression::GreaterThan);
                                    continue 'query
                                },
                                _ => panic!("Unexpected Character! -> `{}`", item)
                            }
                        },
                        None => panic!("Should be more after {}", character)
                    }
                }
                _ => panic!("Unexpected Character! -> `{}`", character)
            };
        }

        Some(Expression::Query(query))
    }
}

impl<'a> Iterator for Expressions<'a> {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        'expression: while let Some(character) = self.expression.next() {
            return Some(match character {
                'a'..='z' => self.get_key(character),
                '0'..='9' => self.get_index(character),
                '#' => {
                    let parsed_hash = self.parse_hash();
                    match parsed_hash {
                        Some(express) => {
                            express
                        },
                        None => {
                            Expression::Hashtag
                        }
                    }
                },
                '.' => Expression::Dot,
                _ => break 'expression
            })
        }

        None
    }
}
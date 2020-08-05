use std::iter::Peekable;
use std::str::Chars;

use crate::parser::JSON;


pub struct ParseExpression<'a> {
    parse: Expressions<'a>,
    json: &'a JSON,
}

impl<'a> ParseExpression<'a> {
    pub fn new(expression: &'a str, json: &'a JSON) -> Self {
        Self {
            parse: Expressions::new(expression, json),
            json
        }
    }

    // pub fn parse_expression(&mut self) -> JSON {

    // }

    pub fn get_expression(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = Vec::new();
        while let Some(expression) = self.step() {
            expressions.push(expression);
        }

        expressions
    }

    fn step(&mut self) -> Option<Expression> {
        let expression_type = self.parse.next();
        if expression_type.is_some() {
            Some(expression_type.unwrap())
        } else {
            None
        }
    }
}

struct Expressions<'a> {
    expression: Peekable<Chars<'a>>,
    json: &'a JSON
}

#[derive(Debug)]
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
    pub fn new(expression: &'a str, json: &'a JSON) -> Self {
        Self {
            expression: expression.chars().peekable(),
            json
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
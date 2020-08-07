use std::iter::Peekable;
use std::str::Chars;

use crate::errors::TokenError;

#[doc(hidden)]
pub enum Tokens {
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

#[doc(hidden)]
pub struct Tokenizer<'a> {
    pub json_source: Peekable<Chars<'a>>,
    _line: usize,
    _char: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            json_source: s.chars().peekable(),
            _line: 1,
            _char: 0
        }
    }

    fn parse_value(&mut self, first_char: char) -> Option<Tokens> {
        let mut value = String::new();

        value.push(first_char);
        'value: while let Some(character) = self.json_source.peek() {
            self._char += 1;
            match character {
                'a'..='z' => {
                    value.push(*character);
                    self.json_source.next();
                }
                _ => {
                    break 'value
                }
            }
        }

        match value.as_str() {
            "true" => Some(Tokens::Boolean(true)),
            "false" => Some(Tokens::Boolean(false)),
            "null" => Some(Tokens::Null),
            _ => panic!("ERROR: {}", TokenError::UnexpectedValue(value.as_str()))
        }
    }

    fn parse_string(&mut self) -> Option<Tokens> {
        let mut slash =  false;
        let mut value = String::new();

        'string: while let Some(character) = self.json_source.next() {
            self._char += 1;
            match character {
                '\\' => {
                    value.push(character);
                    slash = true;
                },
                '"' => {
                    if slash {
                        value.push(character);
                        slash = false;
                        continue 'string;
                    }

                    return Some(Tokens::String(value));
                },
                _ => {
                    value.push(character);
                    slash = false;
                }
            }
        }

        None
    }

    fn parse_number(&mut self, first_char: char) -> Option<Tokens> {
        let mut number = String::new();

        number.push(first_char);
        'number: while let Some(character) = self.json_source.peek() {
            self._char += 1;
            match character {
                '0'..='9'| '.' => {
                    number.push(*character);
                    self.json_source.next();
                },
                _ => break 'number ,
            }
        }

        match number.contains(".") {
            true => Some(Tokens::Float(number.parse::<f64>().unwrap())),
            false => Some(Tokens::Integer(number.parse::<i64>().unwrap()))
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (Tokens, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        'tokenizer: while let Some(character) = self.json_source.next() {
            match character {
                '\n' => { self._line += 1; self._char = 0; },
                _ => self._char += 1,
            };

            return Some(match character {
                '{' => (Tokens::OpeningCurlyBrace, self._line, self._char),
                '}' => (Tokens::ClosingCurlyBrace, self._line, self._char),
                '[' => (Tokens::OpeningBracket, self._line, self._char),
                ']' => (Tokens::ClosingBracket, self._line, self._char),
                ':' => (Tokens::Colon, self._line, self._char),
                ',' => (Tokens::Comma, self._line, self._char),
                '"' => (self.parse_string()?, self._line, self._char),
                '0'..='9' => (self.parse_number(character)?, self._line, self._char),
                'a'..='z' => (self.parse_value(character)?, self._line, self._char),
                _ => {
                    if character.is_whitespace() { continue 'tokenizer; } else { panic!("Unknown Character: {}", character) }
                }
            })
        }

        None
    }
}
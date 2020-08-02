// use std::iter::Peekable;
// use std::str::Chars;

// pub struct Expressions<'a> {
//     expression: Peekable<Chars<'a>>
// }

// enum Operator {

// }

// impl<'a> Expressions<'a> {
//     pub fn new<S> (expression: S) -> Self
//     where
//         S: Into<&'a str>
//     {
//         Self {
//             expression: expression.into().chars().peekable()
//         }
//     }

//     pub fn parse_expression(&mut self) -> JSON {

//     }
// }

// impl<'a> Iterator for Expressions<'a> {
//     type Item = Operator;

//     fn next(&mut self) -> Option<Self::Item> {
//         'expression while let Some(char) = self.expression.
//     }
// }
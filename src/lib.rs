//! JSON Parser
//!
//! This JSON Parser was made to learn about rust and how to use it in different ways.
//! Also to make JSON parsing look nicer. In my opinion, but overall just made it to make it.
//!
//! This is an example JSON file
//! ```
//! [
//!     1, 2, 3, 4, 5.5,
//!     {
//!         "Simple Arr": [1, 2, 3, 4]
//!     },
//!     10, 11, 12, 13, 14, 15
//! ]
//! ```
//!
//! Here's how you would parse the JSON with my Parser.
//! There are better ones out there, this is just for a learning experiment!
//! ```
//! let mut file = File::open("json_file").unwrap();
//! let mut contents = String::new();
//! file.read_to_string(&mut contents).unwrap();
//! let arr = parser::Parser::new(contents.as_str()).parse();
//! assert_eq!(arr,
//! array![
//!     1, 2, 3, 4, 5.5,
//!     object!{
//!         "Simple Arr" => array![1, 2, 3, 4]
//!     },
//!     10, 11, 12, 13, 14, 15
//! ]);
//! ```

pub mod json;

pub mod just;

#[macro_use]
mod macros;

mod expression;
mod tests;
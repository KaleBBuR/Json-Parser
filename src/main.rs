#[allow(dead_code)]
#[allow(non_snake_case)]

use std::fs::File;
use std::io::Read;

pub mod errors;
pub mod parser;
pub mod tokenizer;

use crate::parser::Parser;

fn main() {
    println!("JSON Parser!");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_test1() {
        let mut test1 = File::open("src/test1.json").unwrap();
        let mut contents = String::new();
        test1.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(contents.as_str());
        let json = parser.parse();
        eprintln!("{:?}", json);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_test2() {
        let mut test2 = File::open("src/test2.json").unwrap();
        let mut contents = String::new();
        test2.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(contents.as_str());
        let json = parser.parse();
        eprintln!("{:?}", json);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_test3() {
        let mut test3 = File::open("src/test3.json").unwrap();
        let mut contents = String::new();
        test3.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(contents.as_str());
        let json = parser.parse();
        eprintln!("{:?}", json);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_json_arr() {
        let mut test4 = File::open("src/test4.json").unwrap();
        let mut contents = String::new();
        test4.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(contents.as_str());
        let arr = parser.parse();
        eprintln!("{:?}", arr);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_json_obj() {
        let mut test5 = File::open("src/test5.json").unwrap();
        let mut contents = String::new();
        test5.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(contents.as_str());
        let obj = parser.parse();
        eprintln!("{:?}", obj);
        assert_eq!(1, 1);
    }
}

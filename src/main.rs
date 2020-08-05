
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::Read;

pub mod errors;
pub mod parser;
pub mod tokenizer;
pub mod get;
pub mod macros;
pub mod expression;

#[allow(unused_imports)]
use crate::parser::Parser;

fn main() {
    println!("JSON Parser!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expressions() {
        let mut test1 = File::open("src/test1.json").unwrap();
        let mut contents = String::new();
        test1.read_to_string(&mut contents).unwrap();
        let json = Parser::new(contents.as_str()).parse();
        let expression_1 = json.test_get_expressions("pageInfo.resultsPerPage");
        let expression_2 = json.test_get_expressions("items.#.kind");
        let expression_3 = json.test_get_expressions("items.#.#(id.kind=='youtube#video')");
        let expression_4 = json.test_get_expressions("#(pageInfo.totalResults>4000)");
        let expression_5 = json.test_get_expressions("items.#");
        let expression_6 = json.test_get_expressions("items.2");
        eprintln!(
        "EXPRESSION 1: {:?}\n\nEXPRESSION 2: {:?}\n\nEXPRESSION 3: {:?}\n\nEXPRESSION 4: {:?}\n\nEXPRESSION 5: {:?}\n\nEXPRESSION 6: {:?}\n\n",
        expression_1,
        expression_2,
        expression_3,
        expression_4,
        expression_5,
        expression_6
        );
    }

    #[test]
    fn test_index_arr() {
        let arr = array![true, 10, "hello", 11.5];
        assert!(arr[0] == true);
        assert!(arr[1] == 10);
        assert!(arr[2] == "hello");
        assert!(arr[3] == 11.5);

        let arr_val = arr[3].get_float().unwrap();
        eprintln!("{}", arr_val);
    }

    #[test]
    fn test_index_obj() {
        let obj = object!{
            "Cringe" => true,
            "Ugly" => 10,
            "Death" => "Please"
        };

        assert!(obj["Cringe"] == true);
        assert!(obj["Ugly"] == 10);
        assert!(obj["Death"] == "Please");

        let cringe = obj["Cringe"].get_bool().unwrap();
        if cringe {
            eprintln!("Cringe");
        }
    }

    #[test]
    fn test_test1() {
        let mut test1 = File::open("src/test1.json").unwrap();
        let mut contents = String::new();
        test1.read_to_string(&mut contents).unwrap();
        let json = Parser::new(contents.as_str()).parse();
        eprintln!("\n{:?}\n", json);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_test2() {
        let mut test2 = File::open("src/test2.json").unwrap();
        let mut contents = String::new();
        test2.read_to_string(&mut contents).unwrap();
        let json = Parser::new(contents.as_str()).parse();
        eprintln!("\n{:?}\n", json);
        assert!(json[0]["entities"]["hashtags"][0]["text"] == "Angular");
    }

    #[test]
    fn test_test3() {
        let mut test3 = File::open("src/test3.json").unwrap();
        let mut contents = String::new();
        test3.read_to_string(&mut contents).unwrap();
        let json = Parser::new(contents.as_str()).parse();
        eprintln!("\n{:?}\n", json);
        assert_eq!(json,
            object!{
                "colors" => array![
                    object!{
                        "color" => "black",
                        "category" => "hue",
                        "type" => "primary",
                        "code" => object!{
                            "rgba" => array![255, 255, 255, 1],
                            "hex" => "#000"
                        }
                    },
                    object!{
                        "color" => "white",
                        "category" => "value",
                        "code" => object!{
                            "rgba" => array![0, 0, 0, 1],
                            "hex" => "#FFF"
                        }
                    },
                    object!{
                        "color" => "red",
                        "category" => "hue",
                        "type" => "primary",
                        "code" => object!{
                            "rgba" => array![255, 0, 0, 1],
                            "hex" => "#FF0"
                        }
                    },
                    object!{
                        "color" => "blue",
                        "category" => "hue",
                        "type" => "primary",
                        "code" => object!{
                            "rgba" => array![0, 0, 255, 1],
                            "hex" => "#00F"
                        }
                    },
                    object!{
                        "color" => "yellow",
                        "category" => "hue",
                        "type" => "primary",
                        "code" => object!{
                            "rgba" => array![255, 255, 0, 1],
                            "hex" => "#FF0"
                        }
                    },
                    object!{
                        "color" => "green",
                        "category" => "hue",
                        "type" => "secondary",
                        "code" => object!{
                            "rgba" => array![0, 255, 0, 1],
                            "hex" => "#0F0"
                        }
                    }
                ]
            }
        );
    }

    #[test]
    fn test_json_arr() {
        let mut test4 = File::open("src/test4.json").unwrap();
        let mut contents = String::new();
        test4.read_to_string(&mut contents).unwrap();
        let arr = Parser::new(contents.as_str()).parse();
        eprintln!("\n{:?}\n", arr);
        assert_eq!(arr, array![
            1, 2, 3, 4, 5.5, object!{
                "Simple Arr" => array![1, 2, 3, 4]
            }, 10, 11, 12, 13, 14, 15
        ]);
    }

    #[test]
    fn test_json_obj() {
        let mut test5 = File::open("src/test5.json").unwrap();
        let mut contents = String::new();
        test5.read_to_string(&mut contents).unwrap();
        let obj = Parser::new(contents.as_str()).parse();
        eprintln!("\n{:?}\n", obj);
        assert_eq!(obj, object!{
            "Hello" => "hi",
            "Goodbye" => "bye",
            "Why" => object!{
                "Because" => 1000,
                "Can" => true
            }
        });
    }
}

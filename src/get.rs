use std::collections::HashMap;
#[allow(unused_imports)]
use std::fmt;
use std::ops;

use crate::parser::JSON;
use crate::expression::{Expression, ParseExpression};

impl JSON {
    pub fn new_arr() -> JSON {
        JSON::Array(Vec::new())
    }

    pub fn new_obj() -> JSON {
        JSON::Object(HashMap::new())
    }

    pub fn get<'a, S> (&self, expression: S) -> JSON
    where
        S: Into<&'a str>
    {
        let expression = expression.into();
        ParseExpression::new(expression, self).parse_expression()
    }

    pub fn test_get_expressions(&self, expression: &str) -> Vec<Expression> {
        ParseExpression::new(expression, &self).get_expressions()
    }

    pub fn get_arr(&self) -> Option<Vec<JSON>> {
        match self {
            JSON::Array(json_arr) => Some(json_arr.to_vec()),
            _ => None
        }
    }

    pub fn get_obj(&self) -> Option<HashMap<String, JSON>> {
        match self {
            JSON::Object(json_obj) => Some(json_obj.to_owned()),
            _ => None
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        match *self {
            JSON::Integer(int_val) => Some(int_val),
            _ => None
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        match *self {
            JSON::Float(float) => Some(float),
            _ => None
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match *self {
            JSON::String(ref string) => Some(string.to_owned()),
            _ => None
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match *self {
            JSON::Boolean(boolean) => Some(boolean),
            _ => None
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            JSON::Null => true,
            _ => false,
        }
    }
}

impl ops::Index<usize> for JSON {
    type Output = JSON;

    fn index(&self, index: usize) -> &Self::Output {
        match *self {
            JSON::Array(ref json_arr) => {
                match json_arr.get(index) {
                    Some(arr_item) => {
                        arr_item
                    },
                    None => panic!("Expected Array!")
                }
            },
            _ => panic!("Expected Array!")
        }
    }
}

impl<'a> ops::Index<&'a str> for JSON {
    type Output = JSON;

    fn index(&self, index: &str) -> &Self::Output {
        match *self {
            JSON::Object(ref json_obj) => {
                match json_obj.get(index) {
                    Some(obj_item) => {
                        obj_item
                    },
                    None => panic!("Expected Object!")
                }
            }
             _ => panic!("Expected Object!")
        }
    }
}

impl ops::Index<String> for JSON {
    type Output = JSON;

    fn index(&self, index: String) -> &Self::Output {
        match *self {
            JSON::Object(ref json_obj) => {
                match json_obj.get(&index) {
                    Some(obj_item) => {
                        obj_item
                    },
                    None => panic!("Expected Object!")
                }
            }
             _ => panic!("Expected Object!")
        }
    }
}

impl From<i64> for JSON {
    fn from(item: i64) -> Self {
        JSON::Integer(item)
    }
}

impl From<f64> for JSON {
    fn from(item: f64) -> Self {
        JSON::Float(item)
    }
}

impl From<bool> for JSON {
    fn from(item: bool) -> Self {
        JSON::Boolean(item)
    }
}

impl From<String> for JSON {
    fn from(item: String) -> Self {
        JSON::String(item)
    }
}

impl From<Vec<JSON>> for JSON {
    fn from(items: Vec<JSON>) -> Self {
        let mut arr = Vec::new();
        for item in items {
            arr.push(item.into());
        }

        JSON::Array(arr)
    }
}

impl From<HashMap<String, JSON>> for JSON {
    fn from(map: HashMap<String, JSON>) -> Self {
        let mut hm = HashMap::new();
        for (key, val) in map {
            hm.insert(key, val.into());
        }

        JSON::Object(hm)
    }
}

impl<'a> From<&'a str> for JSON {
    fn from(item: &str) -> Self {
        JSON::String(item.to_string())
    }
}

impl PartialEq<i64> for JSON {
    fn eq(&self, other: &i64) -> bool {
        match *self {
            JSON::Integer(ref value) => value == other,
            _ => false
        }
    }
}

impl PartialEq<f64> for JSON {
    fn eq(&self, other: &f64) -> bool {
        match *self {
            JSON::Float(ref value) => value == other,
            _ => false
        }
    }
}

impl<'a> PartialEq<&'a str> for JSON {
    fn eq(&self, other: &&'a str) -> bool {
        match *self {
            JSON::String(ref value) => &other.to_string() == value,
            _ => false
        }
    }
}

impl PartialEq<String> for JSON {
    fn eq(&self, other: &String) -> bool {
        match *self {
            JSON::String(ref value) => value == other,
            _ => false
        }
    }
}

impl PartialEq<bool> for JSON {
    fn eq(&self, other: &bool) -> bool {
        match *self {
            JSON::Boolean(ref value) => value == other,
            _ => false
        }
    }
}

impl PartialEq<Vec<JSON>> for JSON {
    fn eq(&self, other: &Vec<JSON>) -> bool {
        match *self {
            JSON::Array(ref value) => value == other,
            _ => false
        }
    }
}

impl PartialEq<HashMap<String, JSON>> for JSON {
    fn eq(&self, other: &HashMap<String, JSON>) -> bool {
        match *self {
            JSON::Object(ref value) => value == other,
            _ => false
        }
    }
}

// impl fmt::Display for JSON {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             JSON::Boolean(ref bool_val) => if *bool_val { write!(f, "true") } else { write!(f, "false") },
//             JSON::Integer(ref int_val) => write!(f, "{}", int_val),
//             JSON::Float(ref float_val) => write!(f, "{}", float_val),
//             JSON::String(ref string_val) => write!(f, "{}", string_val),
//             JSON::Null => write!(f, "null"),
//             JSON::Array(ref arr_vals) => {
//                 let mut arr_str = String::new();
//                 for (i, item) in arr_vals.clone().iter().enumerate() {
//                     if i < (arr_vals.len()-1) {
//                         arr_str.push_str(format!("{}, ", item).as_str())
//                     } else {
//                         arr_str.push_str(format!("{}", item).as_str())
//                     }
//                 }
//                 write!(f, "[{}]", arr_str)
//             },
//             JSON::Object(ref obj_vals) => {
//                 let mut obj = HashMap::new();

//                 for (key, value) in obj_vals.clone().iter() {
//                     obj.insert(format!("{}", key), format!("{}", value));
//                 }

//                 write!(f, "{:?}", obj)
//             },
//         }
//     }
// }


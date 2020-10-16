//! # JSON
//!
//! This is all the functions JSON struct can do.

use std::{
    ops,
    collections::HashMap,
    str::FromStr
};

use crate::expression::ParseExpression;
use crate::just::{TokenError, JUST, Tokenize};

/// Represents all the JSON values
#[derive(Debug, Clone, PartialEq)]
pub enum JSON {
    /// JSON Boolean
    Boolean(bool),

    /// JSON Integer
    Integer(i64),

    /// JSON Float
    Float(f64),

    /// JSON String
    String(String),

    /// JSON Array
    Array(Vec<JSON>),

    /// JSON Object
    Object(HashMap<String, JSON>),

    /// JSON Null
    Null,
}

impl JSON {
     /// Allows you get items from a JSON
    ///
    /// # Examples
    ///
    /// ```
    /// let json: JSON = object!{
    ///     "kind" => "youtube#searchListResponse",
    ///     "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/PaiEDiVxOyCWelLPuuwa9LKz3Gk",
    ///     "nextPageToken" => "CAUQAA",
    ///     "regionCode" => "KE",
    ///     "pageInfo" => object!{
    ///         "totalResults" => 4249,
    ///         "resultsPerPage" => 5
    ///     },
    ///     "items" => array![
    ///         object!{
    ///             "kind" => "youtube#searchResult",
    ///             "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/QpOIr3QKlV5EUlzfFcVvDiJT0hw",
    ///             "id" => object!{
    ///                 "kind" => "youtube#channel",
    ///                 "videoId" => "UCJowOS1R0FnhipXVqEnYU1A"
    ///             }
    ///          },
    ///         object!{
    ///             "kind" => "youtube#searchResult",
    ///             "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/AWutzVOt_5p1iLVifyBdfoSTf9E",
    ///             "id" => object!{
    ///                 "kind" => "youtube#video",
    ///                 "videoId" => "Eqa2nAAhHN0"
    ///             }
    ///         },
    ///         object!{
    ///             "kind" => "youtube#searchResult",
    ///             "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/2dIR9BTfr7QphpBuY3hPU-h5u-4",
    ///             "id" => object!{
    ///                 "kind" => "youtube#video",
    ///                 "videoId" => "IirngItQuVs"
    ///             }
    ///         }
    ///     ]
    /// };
    ///
    /// assert_eq!(json.get("pageInfo.totalResults"), JSON::Integer(4249));
    /// assert_eq!(json.get("pageInfo.totalResults").get_int(), Some(4249));
    /// assert_eq!(json.get("items.#"), JSON::Integer(3));
    /// assert_eq!(json.get("items.#(id.kind=='youtube#video')#.etag"),
    /// array![
    ///     object!{
    ///         "kind" => "youtube#searchResult",
    ///         "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/AWutzVOt_5p1iLVifyBdfoSTf9E",
    ///         "id" => object!{
    ///            "kind" => "youtube#video",
    ///            "videoId" => "Eqa2nAAhHN0"
    ///         }
    ///     },
    ///     object!{
    ///         "kind" => "youtube#searchResult",
    ///         "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/2dIR9BTfr7QphpBuY3hPU-h5u-4",
    ///         "id" => object!{
    ///             "kind" => "youtube#video",
    ///             "videoId" => "IirngItQuVs"
    ///         }
    ///     }
    /// ]);
    /// assert_eq!(json.get("items.2"),
    /// object!{
    ///     "kind" => "youtube#searchResult",
    ///     "etag" => "m2yskBQFythfE4irbTIeOgYYfBU/2dIR9BTfr7QphpBuY3hPU-h5u-4",
    ///     "id" => object!{
    ///        "kind" => "youtube#video",
    ///        "videoId" => "IirngItQuVs"
    ///     }
    /// });
    /// ```
    pub fn get<'a, S> (&self, expression: S) -> JSON
    where
        S: Into<&'a str>
    {
        let expression = expression.into();
        ParseExpression::new(expression, self).parse_expression()
    }

    /// Returns an Option Array (Vec)
    ///
    /// # Examples
    ///
    /// ```
    /// let array = array![5, 10, true, false, array![1, 2, 3]];
    /// assert_eq!(array[4].get_arr(), Some(array![1, 2, 3]));
    ///
    /// let obj = object!{
    ///     "how are you" => "Good",
    ///     "words" => array![
    ///         1, 3, true, "hello"
    ///     ]
    /// };
    /// assert_eq!(object["words"].get_arr(), Some(array![1, 3, true, "hello"]));
    /// ```
    #[inline]
    pub fn get_arr(&self) -> Option<Vec<JSON>> {
        match self {
            JSON::Array(json_arr) => Some(json_arr.to_vec()),
            _ => None
        }
    }

    /// Returns an Option Object (Hashmap)
    ///
    /// # Examples
    ///
    /// ```
    /// let array = array!["hello", "bye", object!{"you" => "cute"}];
    /// assert_eq!(array[2], Some(object!{"you" => "cute"}));
    ///
    /// let obj = object!{
    ///     "cringe" => "no you",
    ///     "cities" => object {
    ///         "california" => "San diego",
    ///         "texas" => "Dallas"
    ///     }
    /// };
    /// assert_eq!(obj["cities"].get_obj(), Some(object!{"california" => "San diego", "texas" => "Dallas"}));
    #[inline]
    pub fn get_obj(&self) -> Option<HashMap<String, JSON>> {
        match self {
            JSON::Object(json_obj) => Some(json_obj.to_owned()),
            _ => None
        }
    }

    /// Returns an Option Integer 64
    ///
    /// # Example
    ///
    /// ```
    /// let array = array![1, 2.5, true, 10, false, array![1, 2, 3], 5.6, "hello", obj{"cringe" => "you"}, "no u", JSON::Null];
    /// assert_eq!(array[0].get_int(), Some(1));
    /// assert_eq!(array[3].get_int(), Some(10));
    /// ```
    #[inline]
    pub fn get_int(&self) -> Option<i64> {
        match *self {
            JSON::Integer(int_val) => Some(int_val),
            _ => None
        }
    }

    /// Returns an Option Float 64
    ///
    /// # Example
    ///
    /// ```
    /// let array = array![1, 2.5, true, 10, false, array![1, 2, 3], 5.6, "hello", obj{"cringe" => "you"}, "no u", JSON::Null];
    /// assert_eq!(array[1].get_float(), Some(2.5));
    /// assert_eq!(array[6].get_float(), Some(5.6));
    /// ```
    #[inline]
    pub fn get_float(&self) -> Option<f64> {
        match *self {
            JSON::Float(float) => Some(float),
            _ => None
        }
    }

    /// Returns an Option String
    ///
    /// # Example
    ///
    /// ```
    /// let array = array![1, 2.5, true, 10, false, array![1, 2, 3], 5.6, "hello", obj{"cringe" => "you"}, "no u", JSON::Null];
    /// assert_eq!(array[7].get_string(), Some(String::from("hello")));
    /// assert_eq!(array[9].get_string(), Some(String::from("no u")));
    /// ```
    #[inline]
    pub fn get_string(&self) -> Option<String> {
        match *self {
            JSON::String(ref string) => Some(string.to_owned()),
            _ => None
        }
    }

    /// Returns an Option Boolean
    ///
    /// # Example
    ///
    /// ```
    /// let array = array![1, 2.5, true, 10, false, array![1, 2, 3], 5.6, "hello", obj{"cringe" => "you"}, "no u", JSON::Null];
    /// assert_eq!(array[2].get_bool(), Some(true));
    /// assert_eq!(array[4].get_bool(), Some(false));
    /// ```
    #[inline]
    pub fn get_bool(&self) -> Option<bool> {
        match *self {
            JSON::Boolean(boolean) => Some(boolean),
            _ => None
        }
    }

    /// Returns an Boolean, and checks if it's a JSON::Null type
    ///
    /// # Example
    ///
    /// ```
    /// let array = array![1, 2.5, true, 10, false, array![1, 2, 3], 5.6, "hello", obj{"cringe" => "you"}, "no u", JSON::Null];
    /// assert_eq!(array[8].is_null(), Some(false));
    /// assert_eq!(array[10].is_null(), Some(true));
    /// ```
    #[inline]
    pub fn is_null(&self) -> bool {
        match *self {
            JSON::Null => true,
            _ => false,
        }
    }
}

impl FromStr for JSON {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.chars().peekable().tokens()?;
        let mut just = JUST {
            tokens: Box::new(x.iter())
        };

        Ok(just.parse()?)
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
                println!("{}", index);
                match json_obj.get(&format!("{}", index)) {
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
                println!("{:?}", json_obj);
                match json_obj.get(&format!("{}", index)) {
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


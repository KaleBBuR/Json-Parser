use std::collections::HashMap;

#[path = "lex.rs"] mod lex;

#[derive(Debug)]
pub enum ParserError {
    IncorrectStartingValue,
    MissingQuotes,
}

/*
 * Did take the number idea from Serde JSON, heh... If you want a really good parser. Check them out -> https://docs.serde.rs/serde_json/
 */
#[derive(Debug)]
pub struct Number {
    n: N
}

#[derive(Debug)]
enum N {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum JSON {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<JSON>),
    Object(HashMap<String, JSON>)
}

pub fn to_map_from_object(json: &String) -> HashMap<String, JSON>{
    let lex = lex::lexical_analysis(json);
    check_first_char(&lex[0], lex::BEGIN_OBJ);
    parse_object(&lex, 0)
}

pub fn to_map_from_vec(json: &String) -> Vec<JSON> {
    let lex = lex::lexical_analysis(json);
    check_first_char(&lex[0], lex::BEGIN_ARR);
    parse_array(&lex, 0)
}


pub fn parse_value(lexvalue: &String, lex: &Vec<String>, index: usize) -> JSON {
    let value = lexvalue.as_str();

    if value.ends_with("\"") && value.starts_with("\"") {
        return JSON::String(value.to_string().replace("\"", ""))
    }

    if !value.starts_with("\"") && value.ends_with("\"") || value.starts_with("\"") && !value.ends_with("\"") {
        panic!(format!("Missing Quotation mark! {}: {:?}", value, ParserError::MissingQuotes));
    }

    if value.contains(".") {
        let float_value = value.parse::<f64>();
        if float_value.is_ok() {
            return JSON::Number(Number{n: N::Float(float_value.unwrap())})
        }
    }

    let int_value = value.parse::<i64>();
    if int_value.is_ok() {
        return JSON::Number(Number{n: N::Int(int_value.unwrap())})
    }

    let bool_value = value.parse::<bool>();
    if bool_value.is_ok() {
        return JSON::Bool(bool_value.unwrap());
    }

    if value == lex::BEGIN_ARR {
        let arr = parse_array(lex, index);
        return JSON::Array(arr)
    }

    if value == lex::BEGIN_OBJ {
        let obj = parse_object(lex, index);
        return JSON::Object(obj)
    }

    JSON::Null
}

fn parse_keyword(keyword: &String) -> String {
    if keyword.as_str().starts_with("\"") && keyword.as_str().starts_with("\"") {
        return keyword.to_owned().replace("\"", "");
    }

    panic!(format!("Missing Quotation mark! {}: {:?}", keyword, ParserError::MissingQuotes));
}

pub fn parse_object(lex: &Vec<String>, index: usize) -> HashMap<String, JSON> {
    let mut object_index = 0;
    let mut map: HashMap<String, JSON> = HashMap::new();
    for (i, token) in lex[index..lex.len()].iter().enumerate() {
        let str_token = token.as_str();
        if str_token == lex::END_OBJ || str_token == lex::END_ARR {
            object_index -= 1;
            if object_index == 0 {
                break;
            }
            continue;
        }

        if str_token == lex::BEGIN_OBJ || str_token == lex::BEGIN_ARR {
            object_index += 1;
            continue;
        }

        if object_index > 1 {
            continue;
        }

        if str_token == lex::NAME_SEP {
            let keyword = parse_keyword(&lex.clone()[index+i-1]);
            let value = parse_value(&lex.clone()[index+i+1], &lex, index+i+1);
            map.insert(keyword, value);
        }
    }

    map
}

pub fn parse_array(lex: &Vec<String>, index: usize) -> Vec<JSON> {
    let mut arr_index = 0;
    let mut json_arr: Vec<JSON> = Vec::new();
    for (i, token) in lex[index..lex.len()].iter().enumerate() {
        let str_token = token.as_str();
        if str_token == lex::VALUE_SEP { continue; }
        if str_token == lex::END_ARR || str_token == lex::END_OBJ {
            arr_index -= 1;
            if arr_index == 0 {
                break;
            }
            continue;
        }

        if str_token == lex::BEGIN_ARR {
            if arr_index == 1 {
                let arr = parse_array(lex, index+i);
                json_arr.push(JSON::Array(arr));
            }
            arr_index += 1;
            continue;
        }

        if str_token == lex::BEGIN_OBJ {
            if arr_index == 1 {
                let object = parse_object(lex, index+i);
                json_arr.push(JSON::Object(object));
            }
            arr_index += 1;
            continue;
        }

        if arr_index > 1 {
            continue;
        }

        let value = parse_value(token, lex, index+i);
        json_arr.push(value);
    }

    json_arr
}

fn check_first_char(first_char: &String, check_char: &str) {
    if first_char.as_str() != check_char {
        panic!(format!("ERROR: Must start with -> {} : {:?}", check_char, ParserError::IncorrectStartingValue));
    }
}
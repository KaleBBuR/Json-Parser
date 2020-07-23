pub const BEGIN_ARR: &str = "[";
pub const BEGIN_OBJ: &str = "{";
pub const END_ARR: &str = "]";
pub const END_OBJ: &str = "}";
pub const NAME_SEP: &str = ":";
pub const VALUE_SEP: &str = ",";

pub fn lexical_analysis(JSON: &String) -> Vec<String> {
    let mut json_type = String::new();
    let mut lexed_json: Vec<String> = Vec::new();
    let json = JSON.to_owned().replace("\t", "").replace("\r", "").replace("\n", "");
    let arr_chars = json.as_str().chars();
    let mut item_found = false;
    let mut string_found = false;
    for (i, x) in arr_chars.clone().enumerate() {
        if x == '"' && arr_chars.clone().nth(i-1).unwrap() != '\\' {
            json_type.push('"');
            match item_found {
                true => {
                    string_found = false;
                    item_found = false;
                    lexed_json.push(json_type.trim_end().trim_start().to_string());
                    json_type = String::new();
                },
                false => {
                    string_found = true;
                    item_found = true;
                }
            }
            continue;
        }

        if !check_token(x.to_string().as_str()) && !item_found && !x.is_whitespace(){
            item_found = true;
        } else if check_token(x.to_string().as_str()) && !string_found {
            if item_found {
                item_found = false;
                lexed_json.push(json_type.trim_end().trim_start().to_string());
                json_type = String::new();
            }
            lexed_json.push(x.to_string());
            continue;
        }

        if item_found {
            json_type.push(x);
        }
    };

    lexed_json
}

pub fn check_token(item: &str) -> bool {
    if item == NAME_SEP { return true; }
    if item == BEGIN_ARR { return true; }
    if item == BEGIN_OBJ { return true; }
    if item == END_ARR { return true; }
    if item == END_OBJ { return true; }
    if item == VALUE_SEP { return true; }

    false
}
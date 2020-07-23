#[allow(dead_code)]
#[allow(non_snake_case)]

use std::fs;

#[path = "lex.rs"] mod lex;
#[path = "parser.rs"] mod parser;

fn main() {
    println!("JSON Parser!");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        let json1 = fs::read_to_string("./src/test1.json").expect("Cannot open test1.json");
        println!("{:?}\n", lex::lexical_analysis(&json1));
        assert_eq!(lex::lexical_analysis(&json1),
            vec!["{", "\"kind\"", ":", "\"youtube#searchListResponse\"", ",", "\"etag\"", ":", "\"m2yskBQFythfE4irbTIeOgYYfBU/PaiEDiVxOyCWelLPuuwa9LKz3Gk\"", ",", "\"nextPageToken\"", ":", "\"CAUQAA\"", ",", "\"regionCode\"", ":", "\"KE\"", ",", "\"pageInfo\"", ":", "{", "\"totalResults\"", ":", "4249", ",", "\"resultsPerPage\"", ":", "5", "}", ",", "\"items\"", ":", "[", "{", "\"kind\"", ":", "\"youtube#searchResult\"", ",", "\"etag\"", ":", "\"m2yskBQFythfE4irbTIeOgYYfBU/QpOIr3QKlV5EUlzfFcVvDiJT0hw\"", ",", "\"id\"", ":", "{", "\"kind\"", ":", "\"youtube#video\"", ",", "\"videoID\"", ":", "\"UCJowOS1R0FnhipXVqEnYU1A\"", "}", "}", ",", "{", "\"kind\"", ":", "\"youtube#searchResult\"", ",", "\"etag\"", ":", "\"m2yskBQFythfE4irbTIeOgYYfBU/AWutzVOt_5p1iLVifyBdfoSTf9E\"", ",", "\"id\"", ":", "{", "\"kind\"", ":", "\"youtube#video\"", ",", "\"videoId\"", ":", "\"Eqa2nAAhHN0\"", "}", "}", ",", "{", "\"kind\"", ":", "\"youtube#searchResult\"", ",", "\"etag\"", ":", "\"m2yskBQFythfE4irbTIeOgYYfBU/2dIR9BTfr7QphpBuY3hPU-h5u-4\"", ",", "\"id\"", ":", "{", "\"kind\"", ":", "\"youtube#video\"", ",", "\"videoId\"", ":", "\"IirngItQuVs\"", "}", "}", "]", "}"]);
    }

    #[test]
    fn test_parse_map_test1() {
        let json1 = fs::read_to_string("./src/test1.json").expect("Cannot open test1.json");
        let map = parser::to_map_from_object(&json1);
        println!("{:?}\n\n", map);
        println!("KIND: {:?}\n", map["kind"]);
        println!("ETAG: {:?}\n", map["etag"]);
        println!("NEXT PAGE TOKEN: {:?}\n", map["nextPageToken"]);
        println!("REGION CODE: {:?}\n", map["regionCode"]);
        println!("PAGE INFO: {:?}\n", map["pageInfo"]);
        println!("ITEMS: {:?}", map["items"]);
    }

    #[test]
    fn test_parse_map_test2() {
        let json2 = fs::read_to_string("./src/test2.json").expect("Cannot open test2.json");
        let map = parser::to_map_from_vec(&json2);
        println!("{:?}\n\n", map);
        println!("{:?}\n", map[0]);
    }

    #[test]
    fn test_parse_map_test3() {
        let json3 = fs::read_to_string("./src/test3.json").expect("Cannot open test3.json");
        let map = parser::to_map_from_object(&json3);
        println!("{:?}", map);
        println!("{:?}", map["colors"]);
    }

    #[test]
    fn test_parse_array_4() {
        let arr_json = fs::read_to_string("./src/test4.json").expect("Cannot open test4.json");
        let lex = lex::lexical_analysis(&arr_json);
        let _arr = parser::parse_array(&lex, 0);
        println!("{:?}", _arr);
        for i in _arr.iter() {
            println!("{:?}", i);
        }
    }

    #[test]
    fn test_parse_object_5() {
        let _obj_json= fs::read_to_string("./src/test5.json").expect("Cannot open test5.json");
        let lex = lex::lexical_analysis(&_obj_json);
        let _obj = parser::parse_object(&lex, 0);
        println!("{:?}", _obj);
        println!("WHY: {:?}", _obj["Why"]);
        println!("GOODBYE: {:?}", _obj["Goodbye"]);
        println!("HELLO: {:?}", _obj["Hello"]);
    }
}

#[macro_export]
macro_rules! array {
    [] => {
        {
            $crate::parser::JSON::new_arr()
        }
    };

    [ $( $item:expr ),+ ] => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($item.into());
            )*
            $crate::parser::JSON::Array(vec)
        }
    };
}

#[macro_export]
macro_rules! object {
    {} => {
        {
            $crate::parser::JSON::new_obj()
        }
    };

    { $( $key:expr => $value:expr ),+ } => {
        {
            let mut hm = std::collections::HashMap::new();
            $(
                hm.insert(String::from($key), $value.into());
            )*
            $crate::parser::JSON::Object(hm)
        }
    }
}
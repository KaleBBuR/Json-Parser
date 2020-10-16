/// # Examples
///
/// ```
/// let arr1 = array![true, 10, "hello", 11.5];
/// assert!(arr1[0] == true);
/// assert!(arr1[1] == 10);
/// assert!(arr1[2] == "hello");
/// assert!(arr1[3] == 11.5);
///
/// let mut arr2 = array![];
/// arr2.insert(0, 0.into());
/// arr2.insert(1, 1.into());
///
/// assert!(arr[0] == 0);
/// assert!(arr[1] == 1);
///
/// let arr_val = arr1[3].get_float().unwrap();
/// assert_eq(arr_val, String::from("hello");
/// ```

#[macro_export]
macro_rules! array {
    [] => {
        {
            let vec: Vec<$crate::json::JSON> = Vec::new();
            vec
        }
    };

    [ $( $item:expr ),+ ] => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($item.into());
            )*
            $crate::json::JSON::Array(vec)
        }
    };
}


/// # Examples
///
/// ```
/// let obj = object!{
///     "Cringe" => true,
///     "Ugly" => 10,
///     "Death" => "Please"
/// };

/// assert!(obj["Cringe"] == true);
/// assert!(obj["Ugly"] == 10);
/// assert!(obj["Death"] == "Please");

/// let cringe = obj["Cringe"].get_bool().unwrap();
/// assert_eq!(cringe, true);
/// ```
#[macro_export]
macro_rules! object {
    {} => {
        {
            let hm: HashMap<String, $crate::json::JSON> = std::collections::HashMap::new();
            hm
        }
    };

    { $( $key:expr => $value:expr ),+ } => {
        {
            let mut hm = std::collections::HashMap::new();
            $(
                hm.insert(String::from($key), $value.into());
            )*
            $crate::json::JSON::Object(hm)
        }
    }
}
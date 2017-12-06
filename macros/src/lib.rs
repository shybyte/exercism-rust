#[macro_export]
macro_rules! hashmap {
    // special empty rule to avoid "unused_mut" warnings
    () => {
        HashMap::new()
    };
    // https://elliotekj.com/2017/07/03/handling-optional-trailing-commas-macro_rules/
    ( $( $key:expr => $value:expr ),* $(,)* ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

//use std::collections::HashMap;
#[macro_export]
macro_rules! hashmap {
    ( , ) =>{
        {
            compile_error!("Hashmap macro: comma argument without hash values is invalid")
        }
    };
    ( $( $a:expr => $b:expr ),* $(,)? ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($a, $b);
            )*
            hm
        }
    };
}
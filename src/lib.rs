//use std::collections::HashMap;
#[macro_export]

/// This macro allows building out a hash map similar to the way vec! works.
/// Each set of values is listed as a key => value pair separated by commas.
/// ```
/// #[macro_use] extern crate macros;
/// use std::collections::HashMap;
/// let hm1 = hashmap!(23 => 623, 34 => 21);
/// let hm2 = hashmap![
///         'h' => 89,
///         'a' => 1,
///         's' => 19,
///         'h' => 8,
///     ];
/// ```
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

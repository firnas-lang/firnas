#[macro_export]
macro_rules! either {
    ($test: expr => $true_expr: expr; $false_expr: expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

#[macro_export]
macro_rules! string_key_hashmap {
    ($($key: expr => $val: expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key.to_string(), $val);)*
        map
    }}
}

/// A block that executes only when `"dbg"` feature is enabled
///
/// No need for semicolons inside this block.
#[macro_export]
macro_rules! dbg_exec {
    (@inner $s:stmt) => {
        #[cfg(feature = "dbg")] {
            $s
        }
    };
    ( $($s:stmt)+ ) => {
        (||{
            $(dbg_exec!{@inner $s})+
        })();
    }
}

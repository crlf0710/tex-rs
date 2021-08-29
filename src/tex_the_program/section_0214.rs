//! @ Here is a common way to make the current list grow:
//
// @d tail_append(#)==begin link(tail):=#; tail:=link(tail);
macro_rules! tail_append {
    ($globals:expr, $ptr:expr) => {
        link!($globals, tail!($globals)) = $ptr;
        tail!($globals) = link!($globals, tail!($globals));
        // end
    }
}

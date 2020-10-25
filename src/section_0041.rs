//! @ The length of the current string is called |cur_length|:
//
// @d cur_length == (pool_ptr - str_start[str_ptr])
macro_rules! cur_length {
    ($globals:expr) => {
        ($globals.pool_ptr - $globals.str_start[$globals.str_ptr])
    };
}

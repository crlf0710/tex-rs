//! @ Next we consider changes to \TeX's numeric registers.
//
// @<Assignments@>=
macro_rules! Assignments_1235 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // register,advance,multiply,divide: do_register_command(a);
        if $cur_cmd == register || $cur_cmd == advance || $cur_cmd == multiply ||
            $cur_cmd == divide {
            do_register_command($globals, small_number::from($a as u8));
            use crate::section_1236::do_register_command;
            use crate::section_0101::small_number;
            true
        } else {
            false
        }
    }}
}

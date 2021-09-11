//! @ Next we consider changes to \TeX's numeric registers.
//
// @<Assignments@>=
pub(crate) macro Assignments_1235($globals:expr, $cur_cmd:expr, $a:expr) {{
    // register,advance,multiply,divide: do_register_command(a);
    let processed = if $cur_cmd == register
        || $cur_cmd == advance
        || $cur_cmd == multiply
        || $cur_cmd == divide
    {
        do_register_command($globals, small_number::from($a as u8))?;
        use crate::section_0101::small_number;
        use crate::section_1236::do_register_command;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}

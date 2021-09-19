//! @ And of course what goes up must come down.
//
// @d pop_input==@t@> {leave an input level, re-enter the old}
/// leave an input level, re-enter the old
pub(crate) macro pop_input($globals:expr) {{
    // begin decr(input_ptr); cur_input:=input_stack[input_ptr];
    decr!($globals.input_ptr);
    crate::trace_expr_verbose!("input_ptr = {:?}", $globals.input_ptr);
    $globals.cur_input = $globals.input_stack[$globals.input_ptr];
    // end
    use crate::section_0016::decr;
}}

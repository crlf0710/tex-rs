//! @ And of course what goes up must come down.
//
// @d pop_input==@t@> {leave an input level, re-enter the old}
/// leave an input level, re-enter the old
macro_rules! pop_input {
    ($globals:expr) => {
        // begin decr(input_ptr); cur_input:=input_stack[input_ptr];
        decr!($globals.input_ptr);
        $globals.cur_input = $globals.input_stack[$globals.input_ptr];
        // end
    };
}

//! ` `
// @<Go into ordinary math mode@>=
macro_rules! Go_into_ordinary_math_mode {
    ($globals:expr) => {{
        // begin push_math(math_shift_group); eq_word_define(int_base+cur_fam_code,-1);
        push_math($globals, math_shift_group.into());
        eq_word_define($globals, int_base as pointer + cur_fam_code as pointer, -1);
        // if every_math<>null then begin_token_list(every_math,every_math_text);
        if every_math!($globals) != null {
            begin_token_list($globals, every_math!($globals), every_math_text);
        }
        // end
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0230::int_base;
        use crate::section_0236::cur_fam_code;
        use crate::section_0269::math_shift_group;
        use crate::section_0278::eq_word_define;
        use crate::section_0307::every_math_text;
        use crate::section_0323::begin_token_list;
        use crate::section_1136::push_math;
    }}
}


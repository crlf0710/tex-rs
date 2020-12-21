//! ` `

// @<Character |s| is the current new-line character@>=s=new_line_char
macro_rules! Character_s_is_the_current_new_line_character {
    ($globals:expr, $s:expr) => {
        $s.numeric_value() == new_line_char!($globals) as crate::section_0018::ASCII_code_repr
    }
}


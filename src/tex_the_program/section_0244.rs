//! ` `

// @<Character |s| is the current new-line character@>=s=new_line_char
pub(crate) macro Character_s_is_the_current_new_line_character($globals:expr, $s:expr) {
    $s.numeric_value()
        == crate::section_0236::new_line_char!($globals) as crate::section_0018::ASCII_code_repr
}

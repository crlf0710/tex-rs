//! @ The following code is performed only when |cur_cmd=spacer|.
//
// @<Enter |skip_blanks| state, emit a space@>=
pub(crate) macro Enter_skip_blanks_state__emit_a_space($globals:expr) {
    // begin state:=skip_blanks; cur_chr:=" ";
    state!($globals) = skip_blanks;
    $globals.cur_chr = ASCII_code_literal!(b' ').into();
    // end
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0302::state;
    use crate::section_0303::*;
}

//! ` `
// @<Create a character node |q| for the next...@>=
pub(crate) macro Create_a_character_node_q_for_the_next_character__but_set_q_to_null_if_problem_arise($globals:expr, $q:expr, $f:expr) {{
    // q:=null; f:=cur_font;
    $q = null;
    $f = cur_font!($globals).into();
    // if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given) then
    if $globals.cur_cmd == letter
        || $globals.cur_cmd == other_char
        || $globals.cur_cmd == char_given
    {
        // q:=new_character(f,cur_chr)
        $q = new_character($globals, $f, $globals.cur_chr.into());
    }
    // else if cur_cmd=char_num then
    else if $globals.cur_cmd == char_num {
        // begin scan_char_num; q:=new_character(f,cur_val);
        scan_char_num($globals, true)?;
        $q = new_character($globals, $f, ASCII_code::from($globals.cur_val));
        // end
    }
    // else back_input
    else {
        back_input($globals);
    }
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_0230::cur_font;
    use crate::section_0325::back_input;
    use crate::section_0434::scan_char_num;
    use crate::section_0582::new_character;
}}

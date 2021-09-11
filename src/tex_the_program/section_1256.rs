//! @ Here is where the information for a new font gets loaded.
//
// @<Assignments@>=
pub(crate) macro Assignments_1256($globals:expr, $cur_cmd:expr, $a:expr) {{
    // def_font: new_font(a);
    let processed = if $cur_cmd == def_font {
        new_font($globals, small_number::from($a as u8))?;
        use crate::section_0101::small_number;
        use crate::section_1257::new_font;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}

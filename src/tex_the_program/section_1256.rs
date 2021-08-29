//! @ Here is where the information for a new font gets loaded.
//
// @<Assignments@>=
macro_rules! Assignments_1256 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // def_font: new_font(a);
        if $cur_cmd == def_font {
            new_font($globals, small_number::from($a as u8))?;
            use crate::section_1257::new_font;
            use crate::section_0101::small_number;
            true
        } else {
            false
        }
    }}
}

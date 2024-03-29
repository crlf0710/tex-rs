//! @ Here's an example of the way many of the following routines operate.
//! (Unfortunately, they aren't all as simple as this.)
//
// @<Assignments@>=
pub(crate) macro Assignments_1217($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_font: define(cur_font_loc,data,cur_chr);
    let processed = if $cur_cmd == set_font {
        define!(
            $globals,
            $a,
            cur_font_loc as _,
            data,
            $globals.cur_chr.get() as _
        );
        use crate::section_0230::cur_font_loc;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    use crate::section_0210::*;
    use crate::section_1214::define;
    processed
}}

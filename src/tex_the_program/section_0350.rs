//! ` `
// @<Finish line, |goto switch|@>=
pub(crate) macro Finish_line__goto_switch($globals:expr, $lbl_switch:lifetime) {{
    // begin loc:=limit+1; goto switch;
    loc!($globals) = limit!($globals) + 1;
    crate::goto_backward_label!($lbl_switch);
    // end
    use crate::section_0036::loc;
    use crate::section_0302::limit;
}}

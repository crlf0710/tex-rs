//! ` `
// @<Scan an optional space@>=
macro_rules! Scan_an_optional_space {
    ($globals:expr) => {{
        // begin get_x_token; if cur_cmd<>spacer then back_input;
        get_x_token($globals)?;
        if $globals.cur_cmd != spacer {
            back_input($globals);
        }
        // end
        use crate::section_0380::get_x_token;
        use crate::section_0207::spacer;
        use crate::section_0325::back_input;
    }}
}
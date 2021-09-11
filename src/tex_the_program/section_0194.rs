//! ` `
// @<Display penalty |p|@>=
pub(crate) macro Display_penalty_p($globals:expr, $p:expr) {{
    // begin print_esc("penalty "); print_int(penalty(p));
    print_esc($globals, crate::strpool_str!("penalty "));
    print_int($globals, penalty!($globals, $p as pointer));
    // end
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0115::pointer;
    use crate::section_0157::penalty;
}}

//! ` `
// @<Assignments@>=
pub(crate) macro Assignments_1264($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_interaction: new_interaction;
    let processed = if $cur_cmd == set_interaction {
        new_interaction($globals);
        use crate::section_1265::new_interaction;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}

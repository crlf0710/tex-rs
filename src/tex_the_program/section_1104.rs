//! ` `
//!
//! The |remove_item| command removes a penalty, kern, or glue node if it
//! appears at the tail of the current list, using a brute-force linear scan.
//! Like \.{\\lastbox}, this command is not allowed in vertical mode (except
//! internal vertical mode), since the current list in vertical mode is sent
//! to the page builder.  But if we happen to be able to implement it in
//! vertical mode, we do.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1104($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // any_mode(remove_item): delete_last;
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, remove_item as u16) {
            delete_last($globals)?;
            true
        } else {
            false
        };
    use crate::section_0208::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    use crate::section_1105::delete_last;
    processed
}}

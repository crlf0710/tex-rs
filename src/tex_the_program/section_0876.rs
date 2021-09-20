//! @ Once the best sequence of breakpoints has been found (hurray), we call on the
//! procedure |post_line_break| to finish the remainder of the work.
//! (By introducing this subprocedure, we are able to keep |line_break|
//! from getting extremely long.)
//
// @<Break the paragraph at the chosen...@>=
pub(crate) macro Break_the_paragraph_at_the_chosen_breakpoints__justify_the_resulting_lines_to_the_correct_widths__and_append_them_to_the_current_vertical_list($globals:expr, $final_widow_penalty:expr) {{
    crate::trace_span!("Break the paragraph at the chosen...");
    // post_line_break(final_widow_penalty)
    post_line_break($globals, $final_widow_penalty)?;
    use crate::section_0877::post_line_break;
}}

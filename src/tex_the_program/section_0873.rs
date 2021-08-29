//! ` `
// @<Try the final line break at the end of the paragraph...@>=
macro_rules! Try_the_final_line_break_at_the_end_of_the_paragraph__and_goto_done_if_the_desired_breakpoints_have_been_found {
    ($globals:expr, $lbl_done:lifetime) => {{
        // begin try_break(eject_penalty,hyphenated);
        try_break($globals, eject_penalty, hyphenated.into())?;
        // if link(active)<>last_active then
        if link!($globals, active) != last_active!() {
            // begin @<Find an active node with fewest demerits@>;
            Find_an_active_node_with_fewest_demerits!($globals);
            // if looseness=0 then goto done;
            if looseness!($globals) == 0 {
                goto_forward_label!($lbl_done);
            }
            // @<Find the best active node for the desired looseness@>;
            Find_the_best_active_node_for_the_desired_looseness!($globals);
            // if (actual_looseness=looseness)or final_pass then goto done;
            if $globals.actual_looseness == looseness!($globals) || $globals.final_pass {
                goto_forward_label!($lbl_done);
            }
            // end;
        }
        // end
        use crate::section_0157::eject_penalty;
        use crate::section_0819::hyphenated;
        use crate::section_0829::try_break;
    }}
}

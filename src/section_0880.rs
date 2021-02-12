//! @ The current line to be justified appears in a horizontal list starting
//! at |link(temp_head)| and ending at |cur_break(cur_p)|. If |cur_break(cur_p)| is
//! a glue node, we reset the glue to equal the |right_skip| glue; otherwise
//! we append the |right_skip| glue at the right. If |cur_break(cur_p)| is a
//! discretionary node, we modify the list so that the discretionary break
//! is compulsory, and we set |disc_break| to |true|. We also append
//! the |left_skip| glue at the left of the line, unless it is zero.
//
// @<Justify the line ending at breakpoint |cur_p|, and append it...@>=
macro_rules! Justify_the_line_ending_at_breakpoint_cur_p__and_append_it_to_the_current_vertical_list__together_with_associated_penalties_and_other_insertions {
    ($globals:expr, $cur_line:expr, $disc_break:expr, $post_disc_break:expr, $final_widow_penalty:expr) => {{
        /// temporary registers for list manipulation
        let mut q: pointer;
        // @<Modify the end of the line to reflect the nature of the break and to include
        //   \.{\\rightskip}; also set the proper value of |disc_break|@>;
        Modify_the_end_of_the_line_to_reflect_the_nature_of_the_break_and_to_include_rightskip__also_set_the_proper_value_of_disc_break!
            ($globals, q, $disc_break, $post_disc_break);
        // @<Put the \(l)\.{\\leftskip} glue at the left and detach this line@>;
        Put_the_leftskip_glue_at_the_left_and_detach_this_line!($globals, q);
        // @<Call the packaging subroutine, setting |just_box| to the justified box@>;
        Call_the_packaging_subroutine__setting_just_box_to_the_justified_box!($globals, q, $cur_line);
        // @<Append the new box to the current vertical list, followed by the list of
        //   special nodes taken out of the box by the packager@>;
        Append_the_new_box_to_the_current_vertical_list__followed_by_the_list_of_special_nodes_taken_out_of_the_box_by_the_packager!
            ($globals);
        // @<Append a penalty node, if a nonzero penalty is appropriate@>
        Append_a_penalty_node__if_a_nonzero_penalty_is_appropriate!
            ($globals, $cur_line, $disc_break, $final_widow_penalty);

        use crate::section_0115::pointer;
    }}
}

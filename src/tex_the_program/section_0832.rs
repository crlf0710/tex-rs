//! @ The following code uses the fact that |type(last_active)<>delta_node|.
//
// @d update_width(#)==@|
//   cur_active_width[#]:=cur_active_width[#]+mem[r+#].sc
pub(crate) macro update_width($globals:expr, $idx:expr, $r:expr) {{
    $globals.cur_active_width[$idx] = $globals.cur_active_width[$idx]
        + $globals.mem[$r + $idx][crate::section_0101::MEMORY_WORD_SC]
}}

// @<If node |r|...@>=
pub(crate) macro If_node_r_is_of_type_delta_node__update_cur_active_width__set_prev_r_and_prev_prev_r__then_goto_continue {
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr, $lbl_continue:lifetime) => {{
        crate::trace_span!("If node `r`...");
        // @^inner loop@>
        // if type(r)=delta_node then
        if r#type!($globals, $r) == delta_node {
            // begin do_all_six(update_width);
            do_all_six!(update_width !; @globals = $globals; $r);
            // prev_prev_r:=prev_r; prev_r:=r; goto continue;
            $prev_prev_r = $prev_r;
            $prev_r = $r;
            crate::goto_backward_label!($lbl_continue);
            // end
        }
        use crate::section_0133::r#type;
        use crate::section_0822::delta_node;
        use crate::section_0823::do_all_six;
    }}
}

//! @ An alignment entry ends when a tab or \.{\\cr} occurs, provided that the
//! current level of braces is the same as the level that was present at the
//! beginning of that alignment entry; i.e., provided that |align_state| has
//! returned to the value it had after the \<u_j> template for that entry.
//! @^inner loop@>
//
// @<If an alignment entry has just ended, take appropriate action@>=
macro_rules! If_an_alignment_entry_has_just_ended_take_appropriate_action {
    ($globals:expr, $lbl_restart:lifetime) => {{
        trace_span!("If an alignment entry...");
        // if cur_cmd<=car_ret then if cur_cmd>=tab_mark then if align_state=0 then
        if $globals.cur_cmd <= car_ret {
            if $globals.cur_cmd >= tab_mark {
                if $globals.align_state == 0 {
                    // @<Insert the \(v)\<v_j> template and |goto restart|@>
                    Insert_the_v_j_template_and_goto_restart!($globals, $lbl_restart);
                }
            }
        }
        use crate::section_0207::car_ret;
        use crate::section_0207::tab_mark;
    }}
}

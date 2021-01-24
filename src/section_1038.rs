//! @ The result of \.{\\char} can participate in a ligature or kern, so we must
//! look ahead for it.
//
// @<Look ahead for another character...@>=
macro_rules! Look_ahead_for_another_character__or_leave_lig_stack_empty_if_there_s_none_there {
    ($globals:expr) => {{
        // get_next; {set only |cur_cmd| and |cur_chr|, for speed}
        /// set only `cur_cmd` and `cur_chr`, for speed
        get_next($globals)?;
        // if cur_cmd=letter then goto main_loop_lookahead+1;
        // if cur_cmd=other_char then goto main_loop_lookahead+1;
        // if cur_cmd=char_given then goto main_loop_lookahead+1;
        // x_token; {now expand and set |cur_cmd|, |cur_chr|, |cur_tok|}
        // if cur_cmd=letter then goto main_loop_lookahead+1;
        // if cur_cmd=other_char then goto main_loop_lookahead+1;
        // if cur_cmd=char_given then goto main_loop_lookahead+1;
        // if cur_cmd=char_num then
        //   begin scan_char_num; cur_chr:=cur_val; goto main_loop_lookahead+1;
        //   end;
        // if cur_cmd=no_boundary then bchar:=non_char;
        // cur_r:=bchar; lig_stack:=null; goto main_lig_loop;
        // main_loop_lookahead+1: adjust_space_factor;
        // fast_get_avail(lig_stack); font(lig_stack):=main_f;
        // cur_r:=qi(cur_chr); character(lig_stack):=cur_r;
        // if cur_r=false_bchar then cur_r:=non_char {this prevents spurious ligatures}
        trace_error_expr!("cur_cmd = {}", $globals.cur_cmd);
        todo!("lookahead");
        use crate::section_0341::get_next;
    }}
}

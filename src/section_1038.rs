//! @ The result of \.{\\char} can participate in a ligature or kern, so we must
//! look ahead for it.
//
// @<Look ahead for another character...@>=
macro_rules! Look_ahead_for_another_character__or_leave_lig_stack_empty_if_there_s_none_there {
    ($globals:expr, $cur_part_idx:expr, $lbl_main_loop_cycle:lifetime, $main_loop_status:expr) => {{
        trace_span!("Look ahead for another character...");
        region_multipart_autoincr! {
            ('main_loop_lookahead_inner, $cur_part_idx) {
                0 =>
                {
                    // get_next; {set only |cur_cmd| and |cur_chr|, for speed}
                    /// set only `cur_cmd` and `cur_chr`, for speed
                    get_next($globals)?;
                    // if cur_cmd=letter then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == letter {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // if cur_cmd=other_char then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == other_char {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // if cur_cmd=char_given then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == char_given {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // x_token; {now expand and set |cur_cmd|, |cur_chr|, |cur_tok|}
                    /// now expand and set `cur_cmd`, `cur_chr`, `cur_tok`
                    x_token($globals)?;
                    // if cur_cmd=letter then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == letter {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // if cur_cmd=other_char then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == other_char {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // if cur_cmd=char_given then goto main_loop_lookahead+1;
                    if $globals.cur_cmd == char_given {
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                    }
                    // if cur_cmd=char_num then
                    if $globals.cur_cmd == char_num {
                        // begin scan_char_num; cur_chr:=cur_val; goto main_loop_lookahead+1;
                        scan_char_num($globals, true)?;
                        $globals.cur_chr = chr_code_type::new($globals.cur_val as _);
                        goto_part_label!('main_loop_lookahead_inner, $cur_part_idx, 1);
                        // end;
                    }
                    // if cur_cmd=no_boundary then bchar:=non_char;
                    if $globals.cur_cmd == no_boundary {
                        $globals.bchar = non_char;
                    }
                    // cur_r:=bchar; lig_stack:=null; goto main_lig_loop;
                    $globals.cur_r = $globals.bchar;
                    $globals.lig_stack = null;
                    goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_lig_loop(0));
                },
                1 =>
                {
                    // main_loop_lookahead+1: adjust_space_factor;
                    adjust_space_factor!($globals);
                    // fast_get_avail(lig_stack); font(lig_stack):=main_f;
                    fast_get_avail!($globals, $globals.lig_stack);
                    let f = $globals.main_f;
                    // cur_r:=qi(cur_chr); character(lig_stack):=cur_r;
                    $globals.cur_r = $globals.cur_chr.get();
                    let c = ASCII_code::from($globals.cur_r as integer);
                    assign_font_and_character!($globals, $globals.lig_stack, f, c);
                    // if cur_r=false_bchar then cur_r:=non_char {this prevents spurious ligatures}
                    if $globals.cur_r == $globals.false_bchar {
                        $globals.cur_r = non_char;
                        /// this prevents spurious ligatures
                        const _ : () = ();
                    }
                    /// fall through to `main_lig_loop`
                    goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_lig_loop(0));
                },
            }
        }
        use crate::section_0297::chr_code_type;
        use crate::section_0341::get_next;
        use crate::section_0381::x_token;
        use crate::section_0434::scan_char_num;
    }}
}

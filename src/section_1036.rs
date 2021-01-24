//! ` `

// @<If the cursor is immediately followed by the right boundary...@>=
macro_rules! If_the_cursor_is_immediately_followed_by_the_right_boundary_goto_reswitch__if_its_followed_by_an_invalid_character__goto_big_switch__otherwise_move_the_cursor_one_step_to_the_right_and_goto_main_lig_loop {
    ($globals:expr, $part_idx:expr, $lbl_main_loop_cycle: lifetime, $main_loop_status:expr, $lbl_reswitch:lifetime, $lbl_big_switch:lifetime) => {{
        region_multipart! {
            ('main_loop_move, $part_idx) {
                // @^inner loop@>
                0 => {
                    // if lig_stack=null then goto reswitch;
                    if $globals.lig_stack == null {
                        goto_backward_label!($lbl_reswitch);
                    }
                    // cur_q:=tail; cur_l:=character(lig_stack);
                    $globals.cur_q = tail!($globals);
                    $globals.cur_l = character!($globals, $globals.lig_stack).numeric_value();
                    $part_idx = 1;
                },
                // main_loop_move+1:if not is_char_node(lig_stack) then goto main_loop_move_lig;
                1 => {
                    if !is_char_node!($globals, $globals.lig_stack) {
                        goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_move_lig);
                    }
                    $part_idx = 2;
                },
                // main_loop_move+2:if(cur_chr<font_bc[main_f])or(cur_chr>font_ec[main_f]) then
                2 => {
                    let chr: ASCII_code = $globals.cur_chr.into();
                    if !font_code_range_contains_char($globals, $globals.main_f, chr) {
                        // begin char_warning(main_f,cur_chr); free_avail(lig_stack); goto big_switch;
                        char_warning($globals, $globals.main_f, chr);
                        free_avail!($globals, $globals.lig_stack);
                        goto_backward_label!($lbl_big_switch);
                        // end;
                    }
                    // main_i:=char_info(main_f)(cur_l);
                    $globals.main_i = char_info!($globals, $globals.main_f, $globals.cur_l);
                    // if not char_exists(main_i) then
                    if !$globals.main_i.char_exists() {
                        // begin char_warning(main_f,cur_chr); free_avail(lig_stack); goto big_switch;
                        char_warning($globals, $globals.main_f, chr);
                        free_avail!($globals, $globals.lig_stack);
                        goto_backward_label!($lbl_big_switch);
                        // end;
                    }
                    // link(tail):=lig_stack; tail:=lig_stack {|main_loop_lookahead| is next}
                    link!($globals, tail!($globals)) = $globals.lig_stack;
                    tail!($globals) = $globals.lig_stack;
                    /// fall through to `main_loop_lookahead`
                    goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_lookahead(0));
                },
            }
        }
        use crate::section_0018::ASCII_code;
        use crate::section_0115::null;
        use crate::section_0549::font_code_range_contains_char;
        use crate::section_0581::char_warning;
    }}
}

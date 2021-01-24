//! @ Here we are at |main_loop_move_lig|.
//! When we begin this code we have |cur_q=tail| and |cur_l=character(lig_stack)|.
//
// @<Move the cursor past a pseudo-ligature...@>=
macro_rules! Move_the_cursor_past_a_pseudo_ligature__then_goto_main_loop_lookahead_or_main_lig_loop {
    ($globals:expr, $lbl_main_loop_cycle:lifetime, $main_loop_status:expr) => {{
        // main_p:=lig_ptr(lig_stack);
        $globals.main_p = lig_ptr!($globals, $globals.lig_stack);
        // if main_p>null then tail_append(main_p); {append a single character}
        if $globals.main_p > null {
            /// append a single character
            tail_append!($globals, $globals.main_p);
        }
        // temp_ptr:=lig_stack; lig_stack:=link(temp_ptr);
        $globals.temp_ptr = $globals.lig_stack;
        $globals.lig_stack = link!($globals, $globals.temp_ptr);
        // free_node(temp_ptr,small_node_size);
        free_node($globals, $globals.temp_ptr, small_node_size.into());
        // main_i:=char_info(main_f)(cur_l); ligature_present:=true;
        $globals.main_i = char_info!($globals, $globals.main_f, $globals.cur_l);
        $globals.ligature_present = true;
        // if lig_stack=null then
        if $globals.lig_stack == null {
            // if main_p>null then goto main_loop_lookahead
            if $globals.main_p > null {
                goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_lookahead(0));
            }
            // else cur_r:=bchar
            else {
                $globals.cur_r = $globals.bchar;
            }
        }
        // else cur_r:=character(lig_stack);
        else {
            $globals.cur_r = character!($globals, $globals.lig_stack).numeric_value();
        }
        // goto main_lig_loop
        goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_lig_loop(0));
        use crate::section_0130::free_node;
        use crate::section_0141::small_node_size;
    }}
}

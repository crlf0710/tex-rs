//! ` `

// @<Output node |p| for |vlist_out|...@>=
macro_rules! Output_node_p_for_vlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_h_eq_left_edge {
    ($globals:expr, $p:expr, $left_edge:expr, $cur_g:expr, $g_sign:expr) => {{
        region_forward_label!(
        |'next_p|
        {
        // begin if is_char_node(p) then confusion("vlistout")
        if is_char_node!($globals, $p) {
            confusion($globals, strpool_str!("vlistout"))?;
        }
        // @:this can't happen vlistout}{\quad vlistout@>
        // else @<Output the non-|char_node| |p| for |vlist_out|@>;
        else {
            Output_the_non_char_node_p_for_vlist_out!($globals, $p, $left_edge, $cur_g, $g_sign, 'next_p);
        }
        }
        // next_p:p:=link(p);
        'next_p <-
        );
        $p = link!($globals, $p);
        // end
        use crate::section_0095::confusion;
    }}
}

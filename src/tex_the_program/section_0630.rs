//! ` `

// @<Output node |p| for |vlist_out|...@>=
pub(crate) macro Output_node_p_for_vlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_h_eq_left_edge {
    ($globals:expr, $p:expr, $left_edge:expr, $this_box:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr) => {{
        crate::region_forward_label!(
        |'next_p|
        {
        // begin if is_char_node(p) then confusion("vlistout")
        if is_char_node!($globals, $p) {
            confusion($globals, crate::strpool_str!("vlistout"))?;
        }
        // @:this can't happen vlistout}{\quad vlistout@>
        // else @<Output the non-|char_node| |p| for |vlist_out|@>;
        else {
            crate::section_0631::Output_the_non_char_node_p_for_vlist_out!($globals, $p, $left_edge, $this_box, $cur_glue, $cur_g, $g_sign, $g_order, 'next_p);
        }
        }
        // next_p:p:=link(p);
        'next_p <-
        );
        $p = link!($globals, $p);
        // end
        use crate::section_0095::confusion;
        use crate::section_0118::link;
        use crate::section_0134::is_char_node;
    }}
}

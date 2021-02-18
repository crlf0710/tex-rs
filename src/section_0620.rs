//! @ We ought to give special care to the efficiency of one part of |hlist_out|,
//! since it belongs to \TeX's inner loop. When a |char_node| is encountered,
//! we save a little time by processing several nodes in succession until
//! reaching a non-|char_node|. The program uses the fact that |set_char_0=0|.
//! @^inner loop@>
//
// @<Output node |p| for |hlist_out|...@>=
macro_rules! Output_node_p_for_hlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_v_eq_base_line {
    ($globals:expr, $p:expr, $base_line:expr, $cur_g:expr, $g_sign:expr) => {{
        // reswitch: if is_char_node(p) then
        if is_char_node!($globals, $p) {
            // begin synch_h; synch_v;
            // repeat f:=font(p); c:=character(p);
            // if f<>dvi_f then @<Change font |dvi_f| to |f|@>;
            // if c>=qi(128) then dvi_out(set1);
            // dvi_out(qo(c));@/
            // cur_h:=cur_h+char_width(f)(char_info(f)(c));
            // p:=link(p);
            // until not is_char_node(p);
            // dvi_h:=cur_h;
            // end
            todo!("char node p");
        }
        // else @<Output the non-|char_node| |p| for |hlist_out|
        //     and move to the next node@>
        else {
            Output_the_non_char_node_p_for_hlist_out_and_move_to_the_next_node!
                ($globals, $p, $base_line, $cur_g, $g_sign);
        }
    }}
}

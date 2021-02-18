//! ` `
// @<Move down or output leaders@>=
macro_rules! Move_down_or_output_leaders {
    ($globals:expr, $p:expr, $cur_g:expr, $g_sign:expr, $lbl_move_past:lifetime) => {{
        // begin g:=glue_ptr(p); rule_ht:=width(g)-cur_g;
        $globals.ship_out_g = glue_ptr!($globals, $p);
        $globals.rule_ht = width!($globals, $globals.ship_out_g) - $cur_g;
        // if g_sign<>normal then
        if $g_sign != glue_sign::normal {
            // begin if g_sign=stretching then
            //   begin if stretch_order(g)=g_order then
            //     begin cur_glue:=cur_glue+stretch(g);
            //     vet_glue(float(glue_set(this_box))*cur_glue);
            // @^real multiplication@>
            //     cur_g:=round(glue_temp);
            //     end;
            //   end
            // else if shrink_order(g)=g_order then
            //     begin cur_glue:=cur_glue-shrink(g);
            //     vet_glue(float(glue_set(this_box))*cur_glue);
            //     cur_g:=round(glue_temp);
            //     end;
            // end;
            todo!("g_sign != normal");
        }
        // rule_ht:=rule_ht+cur_g;
        $globals.rule_ht += $cur_g;
        // if subtype(p)>=a_leaders then
        if subtype!($globals, $p) as integer >= glue_node_subtype::a_leaders as integer {
            // @<Output leaders in a vlist, |goto fin_rule| if a rule
            //   or to |next_p| if done@>;
            todo!("output leaders");
        }
        // goto move_past;
        goto_forward_label!($lbl_move_past);
        // end
        use crate::section_0149::glue_node_subtype;
    }};
}

//! ` `
// @d billion==float_constant(1000000000)
// @d vet_glue(#)== glue_temp:=#;
//   if glue_temp>billion then
//            glue_temp:=billion
//   else if glue_temp<-billion then
//            glue_temp:=-billion
//
// @<Move right or output leaders@>=
macro_rules! Move_right_or_output_leaders {
    ($globals:expr, $p:expr, $cur_g:expr, $g_sign:expr, $lbl_move_past:lifetime) => {{
        // begin g:=glue_ptr(p); rule_wd:=width(g)-cur_g;
        $globals.ship_out_g = glue_ptr!($globals, $p);
        $globals.rule_wd = width!($globals, $globals.ship_out_g) - $cur_g;
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
        // rule_wd:=rule_wd+cur_g;
        $globals.rule_wd += $cur_g;
        // if subtype(p)>=a_leaders then
        if r#subtype!($globals, $p) as integer >= glue_node_subtype::a_leaders as integer {
            // @<Output leaders in an hlist, |goto fin_rule| if a rule
            //   or to |next_p| if done@>;
            todo!("output leaders in hlist");
        }
        // goto move_past;
        goto_forward_label!($lbl_move_past);
        // end
        use crate::section_0149::glue_node_subtype;
    }};
}

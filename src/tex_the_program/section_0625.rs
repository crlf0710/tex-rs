//! ` `
// @d billion==float_constant(1000000000)
pub(crate) const billion: real = float_constant!(1000000000);

// @d vet_glue(#)== glue_temp:=#;
macro_rules! vet_glue {
    ($glue_temp:expr, $val:expr) => {{
        $glue_temp = $val;
        // if glue_temp>billion then
        if $glue_temp > billion {
            // glue_temp:=billion
            $glue_temp = billion;
        }
        // else if glue_temp<-billion then
        else if $glue_temp < -billion {
            // glue_temp:=-billion
            $glue_temp = -billion;
        }
        use crate::section_0625::billion;
    }}
}

// @<Move right or output leaders@>=
macro_rules! Move_right_or_output_leaders {
    ($globals:expr, $p:expr, $this_box:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr, $lbl_move_past:lifetime) => {{
        // begin g:=glue_ptr(p); rule_wd:=width(g)-cur_g;
        $globals.ship_out_g = glue_ptr!($globals, $p);
        $globals.rule_wd = width!($globals, $globals.ship_out_g) - $cur_g;
        // if g_sign<>normal then
        if $g_sign != glue_sign::normal {
            /// glue value before rounding
            let mut glue_temp: real;
            // begin if g_sign=stretching then
            if $g_sign == glue_sign::stretching {
                // begin if stretch_order(g)=g_order then
                if stretch_order!($globals, $globals.ship_out_g) as integer == $g_order as integer {
                    // begin cur_glue:=cur_glue+stretch(g);
                    $cur_glue += stretch!($globals, $globals.ship_out_g).inner() as real;
                    // vet_glue(float(glue_set(this_box))*cur_glue);
                    vet_glue!(glue_temp, float!(glue_set!($globals, $this_box)) * $cur_glue);
                    // @^real multiplication@>
                    // cur_g:=round(glue_temp);
                    $cur_g = scaled::new_from_inner(glue_temp.round() as _);
                    // end;
                }
                // end
            }
            // else if shrink_order(g)=g_order then
            else {
                if shrink_order!($globals, $globals.ship_out_g) as integer == $g_order as integer {
                    // begin cur_glue:=cur_glue-shrink(g);
                    $cur_glue -= shrink!($globals, $globals.ship_out_g).inner() as real;
                    // vet_glue(float(glue_set(this_box))*cur_glue);
                    vet_glue!(glue_temp, float!(glue_set!($globals, $this_box)) * $cur_glue);
                    // cur_g:=round(glue_temp);
                    $cur_g = scaled::new_from_inner(glue_temp.round() as _);
                    // end;
                }
            }
            // end;
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

use crate::pascal::real;

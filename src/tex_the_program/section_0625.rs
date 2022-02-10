//! ` `
// @d billion==float_constant(1000000000)
pub(crate) const billion: real = float_constant!(1000000000);

// @d vet_glue(#)== glue_temp:=#;
pub(crate) macro vet_glue($glue_temp:expr, $val:expr) {{
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

// @<Move right or output leaders@>=
pub(crate) macro Move_right_or_output_leaders($globals:expr, $p:expr, $this_box:expr, $base_line:expr, $left_edge:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr, $lbl_move_past:lifetime, $lbl_next_p:lifetime, $lbl_fin_rule:lifetime) {{
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
                $cur_glue += stretch!($globals, $globals.ship_out_g).inner_real();
                // vet_glue(float(glue_set(this_box))*cur_glue);
                vet_glue!(
                    glue_temp,
                    float!(glue_set!($globals, $this_box)) * $cur_glue
                );
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
                $cur_glue -= shrink!($globals, $globals.ship_out_g).inner_real();
                // vet_glue(float(glue_set(this_box))*cur_glue);
                vet_glue!(
                    glue_temp,
                    float!(glue_set!($globals, $this_box)) * $cur_glue
                );
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
    if subtype!($globals, $p) as integer >= glue_node_subtype::a_leaders as integer {
        // @<Output leaders in an hlist, |goto fin_rule| if a rule
        //   or to |next_p| if done@>;
        crate::section_0626::Output_leaders_in_an_hlist__goto_fin_rule_if_a_rule_or_to_next_p_if_done!($globals, $p, $base_line, $left_edge, $lbl_next_p, $lbl_fin_rule);
    }
    // goto move_past;
    crate::goto_forward_label!($lbl_move_past);
    // end
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0109::float;
    use crate::section_0133::subtype;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::width;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::glue_ord;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
}}

use crate::pascal::real;
use crate::section_0109::float_constant;

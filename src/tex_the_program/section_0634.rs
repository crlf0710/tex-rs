//! ` `
// @<Move down or output leaders@>=
pub(crate) macro Move_down_or_output_leaders($globals:expr, $p:expr, $this_box:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr, $lbl_move_past:lifetime) {{
    // begin g:=glue_ptr(p); rule_ht:=width(g)-cur_g;
    $globals.ship_out_g = glue_ptr!($globals, $p);
    $globals.rule_ht = width!($globals, $globals.ship_out_g) - $cur_g;
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
                $cur_glue -= shrink!($globals, $globals.ship_out_g).inner() as real;
                // vet_glue(float(glue_set(this_box))*cur_glue);
                vet_glue!(
                    glue_temp,
                    float!(glue_set!($globals, $this_box)) * $cur_glue
                );
                // cur_g:=round(glue_temp);
                $cur_g = scaled::new_from_inner(glue_temp.round() as _);
                // end;
            }
            // end;
        }
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
    crate::goto_forward_label!($lbl_move_past);
    // end
    use crate::pascal::integer;
    use crate::pascal::real;
    use crate::section_0101::scaled;
    use crate::section_0109::float;
    use crate::section_0133::subtype;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::width;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0625::vet_glue;
}}

//! ` `

// @<Let |d| be the natural width of node |p|...@>=
pub(crate) macro Let_d_be_the_natural_width_of_node_p__if_the_node_is_visible__goto_found__if_the_node_is_glue_that_stretches_or_shrinks__set_v_to_max_dimen($globals:expr, $p:expr, $d:expr, $v:expr, $lbl_found:lifetime) {{
    // reswitch: if is_char_node(p) then
    crate::region_backward_label! {
    'reswitch <-
    {
        if is_char_node!($globals, $p) {
            /// font in current `char_node`
            let f;

            // begin f:=font(p); d:=char_width(f)(char_info(f)(character(p)));
            f = font!($globals, $p);
            let char = character!($globals, $p);
            let char_info = char_info!($globals, f, char.numeric_value());
            let char_width = char_width!($globals, f, char_info);
            $d = char_width;
            // goto found;
            crate::goto_forward_label!($lbl_found);
            // end;
        }
        // case type(p) of
        let type_p = r#type!($globals, $p);
        // hlist_node,vlist_node,rule_node: begin d:=width(p); goto found;
        if type_p == hlist_node || type_p == vlist_node || type_p == rule_node {
            $d = width!($globals, $p);
            crate::goto_forward_label!($lbl_found);
            // end;
        }
        // ligature_node:@<Make node |p| look like a |char_node|...@>;
        else if type_p == ligature_node {
            crate::section_0652::Make_node_p_look_like_a_char_node_and_goto_reswitch!($globals, $p, 'reswitch);
        }
        // kern_node,math_node: d:=width(p);
        else if type_p == kern_node || type_p == math_node {
            $d = width!($globals, $p);
        }
        // glue_node:@<Let |d| be the natural width of this glue; if stretching
        //   or shrinking, set |v:=max_dimen|; |goto found| in the case of leaders@>;
        else if type_p == glue_node {
            crate::section_1148::Let_d_be_the_natural_width_of_this_glue__if_stretching_or_shrinking__set_v_to_max_dimen__goto_found_in_the_case_of_leaders!($globals, $p, $d, $v, $lbl_found);
        }
        // whatsit_node: @<Let |d| be the width of the whatsit |p|@>;
        else if type_p == whatsit_node {
            crate::section_1361::Let_d_be_the_width_of_the_whatsit_p!($globals, $p, $d);
        }
        // othercases d:=0
        else {
            $d = scaled::zero();
        }
        // endcases
    }
    |'reswitch|
    };
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0134::is_char_node;
    use crate::section_0135::hlist_node;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0143::ligature_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0147::math_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
}}

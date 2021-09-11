//! ` `
// @<Examine node |p| in the vlist, taking account of its effect...@>=
pub(crate) macro Examine_node_p_in_the_vlist__taking_account_of_its_effect_on_the_dimensions_of_the_new_box__then_advance_p_to_the_next_node($globals:expr, $p:expr, $w:expr, $d:expr, $x:expr) {{
    // begin if is_char_node(p) then confusion("vpack")
    if is_char_node!($globals, $p) {
        confusion($globals, crate::strpool_str!("vpack"))?;
    }
    // @:this can't happen vpack}{\quad vpack@>
    // else  case type(p) of
    else {
        let type_p = r#type!($globals, $p);
        // hlist_node,vlist_node,rule_node,unset_node:
        if type_p == hlist_node
            || type_p == vlist_node
            || type_p == rule_node
            || type_p == unset_node
        {
            // @<Incorporate box dimensions into the dimensions of
            //   the vbox that will contain~it@>;
            crate::section_0670::Incorporate_box_dimensions_into_the_dimensions_of_the_vbox_that_will_contain_it!(
                $globals, $p, $w, $d, $x
            );
        }
        // whatsit_node:@<Incorporate a whatsit node into a vbox@>;
        else if type_p == whatsit_node {
            crate::section_1359::Incorporate_a_whatsit_node_into_an_vbox!($globals);
        }
        // glue_node: @<Incorporate glue into the vertical totals@>;
        else if type_p == glue_node {
            crate::section_0671::Incorporate_glue_into_the_vertical_totals!(
                $globals, $p, $w, $d, $x
            );
        }
        // kern_node: begin x:=x+d+width(p); d:=0;
        else if type_p == kern_node {
            $x = $x + $d + width!($globals, $p);
            $d = scaled::zero();
        // end;
        }
        // othercases do_nothing
        else {
            do_nothing!();
        }
        // endcases;
    }
    // p:=link(p);
    $p = link!($globals, $p);
    // end
    use crate::section_0016::do_nothing;
    use crate::section_0095::confusion;
    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::hlist_node;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0159::unset_node;
}}

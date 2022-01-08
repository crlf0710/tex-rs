//! ` `
// @<Display node |p|@>=
pub(crate) macro Display_node_p($globals:expr, $p:expr) {{
    // if is_char_node(p) then print_font_and_char(p)
    if is_char_node!($globals, $p as pointer) {
        print_font_and_char($globals, $p);
    }
    // else  case type(p) of
    else {
        let type_p = r#type!($globals, $p as pointer);
        // hlist_node,vlist_node,unset_node: @<Display box |p|@>;
        if type_p == hlist_node || type_p == vlist_node || type_p == unset_node {
            crate::section_0184::Display_box_p!($globals, $p);
        }
        // rule_node: @<Display rule |p|@>;
        else if type_p == rule_node {
            crate::section_0187::Display_rule_p!($globals, $p);
        }
        // ins_node: @<Display insertion |p|@>;
        // whatsit_node: @<Display the whatsit node |p|@>;
        else if type_p == whatsit_node {
            crate::section_1356::Display_the_whatsit_node_p!($globals, $p);
        }
        // glue_node: @<Display glue |p|@>;
        else if type_p == glue_node {
            crate::section_0189::Display_glue_p!($globals, $p);
        }
        // kern_node: @<Display kern |p|@>;
        else if type_p == kern_node {
            crate::section_0191::Display_kern_p!($globals, $p);
        }
        // math_node: @<Display math node |p|@>;
        else if type_p == math_node {
            crate::section_0192::Display_math_node_p!($globals, $p);
        }
        // ligature_node: @<Display ligature |p|@>;
        else if type_p == ligature_node {
            crate::section_0193::Display_ligature_p!($globals, $p);
        }
        // penalty_node: @<Display penalty |p|@>;
        else if type_p == penalty_node {
            crate::section_0194::Display_penalty_p!($globals, $p);
        }
        // disc_node: @<Display discretionary |p|@>;
        else if type_p == disc_node {
            crate::section_0195::Display_discretionary_p!($globals, $p);
        }
        // mark_node: @<Display mark |p|@>;
        // adjust_node: @<Display adjustment |p|@>;
        // @t\4@>@<Cases of |show_node_list| that arise in mlists only@>@;
        else if crate::section_0690::Cases_of_show_node_list_that_arise_in_mlists_only!(
            $globals, $p, type_p
        ) {
            // already processed
        }
        // othercases print("Unknown node type!")
        else {
            crate::trace_error_expr!("type(p) = {}", r#type!($globals, $p as pointer));
            print(
                $globals,
                crate::strpool_str!("Unknown node type!").get() as _,
            );
        }
        // endcases
    }
    use crate::section_0059::print;
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::hlist_node;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0143::ligature_node;
    use crate::section_0145::disc_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0147::math_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::penalty_node;
    use crate::section_0159::unset_node;
    use crate::section_0176::print_font_and_char;
}}

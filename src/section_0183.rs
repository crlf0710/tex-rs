//! ` `
// @<Display node |p|@>=
macro_rules! Display_node_p {
    ($globals:expr, $p:expr) => {{
        // if is_char_node(p) then print_font_and_char(p)
        if is_char_node!($globals, $p as pointer) {
            print_font_and_char($globals, $p);
        }
        // else  case type(p) of
        else {
            let type_p = r#type!($globals, $p as pointer);
            // hlist_node,vlist_node,unset_node: @<Display box |p|@>;
            // rule_node: @<Display rule |p|@>;
            // ins_node: @<Display insertion |p|@>;
            // whatsit_node: @<Display the whatsit node |p|@>;
            if type_p == whatsit_node {
                Display_the_whatsit_node_p!($globals, $p);
            }
            // glue_node: @<Display glue |p|@>;
            else if type_p == glue_node {
                Display_glue_p!($globals, $p);
            }
            // kern_node: @<Display kern |p|@>;
            else if type_p == kern_node {
                Display_kern_p!($globals, $p);
            }
            // math_node: @<Display math node |p|@>;
            // ligature_node: @<Display ligature |p|@>;
            else if type_p == ligature_node {
                Display_ligature_p!($globals, $p);
            }
            // penalty_node: @<Display penalty |p|@>;
            // disc_node: @<Display discretionary |p|@>;
            // mark_node: @<Display mark |p|@>;
            // adjust_node: @<Display adjustment |p|@>;
            // @t\4@>@<Cases of |show_node_list| that arise in mlists only@>@;
            // othercases print("Unknown node type!")
            else {
                trace_error_expr!("type(p) = {}", r#type!($globals, $p as pointer));
                print($globals, strpool_str!("Unknown node type!").get() as _);
            }
            // endcases
        }
        use crate::section_0143::ligature_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0176::print_font_and_char;
    }}
}

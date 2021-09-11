//! ` `
// @<Print a short indication of the contents of node |p|@>=
pub(crate) macro Print_a_short_indication_of_the_contents_of_node_p {
    ($globals:expr, $p:expr) => {{
        // case type(p) of
        let type_p = r#type!($globals, $p);
        // hlist_node,vlist_node,ins_node,whatsit_node,mark_node,adjust_node,
        //   unset_node: print("[]");
        if type_p == hlist_node
            || type_p == vlist_node
            || type_p == ins_node
            || type_p == whatsit_node
            || type_p == mark_node
            || type_p == adjust_node
            || type_p == unset_node
        {
            print($globals, crate::strpool_str!("[]").get() as _);
        }
        // rule_node: print_char("|");
        else if type_p == rule_node {
            print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'|'));
        }
        // glue_node: if glue_ptr(p)<>zero_glue then print_char(" ");
        else if type_p == glue_node {
            if glue_ptr!($globals, $p) != zero_glue {
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b' '));
            }
        }
        // math_node: print_char("$");
        else if type_p == math_node {
            print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'$'));
        }
        // ligature_node: short_display(lig_ptr(p));
        else if type_p == ligature_node {
            short_display($globals, lig_ptr!($globals, $p) as _);
        }
        // disc_node: begin short_display(pre_break(p));
        else if type_p == disc_node {
            short_display($globals, pre_break!($globals, $p) as _);
            // short_display(post_break(p));@/
            short_display($globals, post_break!($globals, $p) as _);
            // n:=replace_count(p);
            todo!("disc_node");
            // while n>0 do
            //   begin if link(p)<>null then p:=link(p);
            //   decr(n);
            //   end;
            // end;
        }
        // othercases do_nothing
        else {
            do_nothing!();
        }
        // endcases
        use crate::section_0004::make_globals_io_string_log_view;
        use crate::section_0016::do_nothing;
        use crate::section_0018::ASCII_code_literal;
        use crate::section_0058::print_char;
        use crate::section_0059::print;
        use crate::section_0133::r#type;
        use crate::section_0135::hlist_node;
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0140::ins_node;
        use crate::section_0141::mark_node;
        use crate::section_0142::adjust_node;
        use crate::section_0143::ligature_node;
        use crate::section_0143::lig_ptr;
        use crate::section_0145::pre_break;
        use crate::section_0145::disc_node;
        use crate::section_0145::post_break;
        use crate::section_0146::whatsit_node;
        use crate::section_0147::math_node;
        use crate::section_0149::glue_node;
        use crate::section_0149::glue_ptr;
        use crate::section_0159::unset_node;
        use crate::section_0162::zero_glue;
        use crate::section_0174::short_display;
    }}
}

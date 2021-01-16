//! ` `

// @<Examine node |p| in the hlist, taking account of its effect...@>=
macro_rules! Examine_node_p_in_the_hlist__taking_account_of_its_effect_on_the_dimensions_of_the_new_box__or_moving_it_to_the_adjustment_list__then_advance_p_to_the_next_node {
    ($globals:expr, $p:expr, $h:expr, $d:expr, $x:expr) => {{
        // @^inner loop@>
        // begin reswitch: while is_char_node(p) do
        while is_char_node!($globals, $p) {
            // @<Incorporate character dimensions into the dimensions of
            //   the hbox that will contain~it, then move to the next node@>;
            Incorporate_character_dimensions_into_the_dimensions_of_the_hbox_that_will_contain_it__then_move_to_the_next_node!
                ($globals, $p, $h, $d, $x);
        }
        // if p<>null then
        if $p != null {
            // begin case type(p) of
            let type_p = r#type!($globals, $p);
            // hlist_node,vlist_node,rule_node,unset_node:
            if type_p == hlist_node || type_p == vlist_node || type_p == rule_node || type_p == unset_node {
                // @<Incorporate box dimensions into the dimensions of
                //   the hbox that will contain~it@>;
                Incorporate_box_dimensions_into_the_dimensions_of_the_hbox_that_will_contain_it!($globals, $p, $h, $d, $x);
            }
            // ins_node,mark_node,adjust_node: if adjust_tail<>null then
            else if type_p == ins_node || type_p == mark_node || type_p == adjust_node {
                if $globals.adjust_tail != null {
                    todo!("transfer node")
                    // @<Transfer node |p| to the adjustment list@>;
                }
            }
            // whatsit_node:@<Incorporate a whatsit node into an hbox@>;
            else if type_p == whatsit_node {
                todo!("whatsit");
            }
            // glue_node:@<Incorporate glue into the horizontal totals@>;
            else if type_p == glue_node {
                Incorporate_glue_into_the_horizontal_totals!($globals, $p, $h, $d, $x);
            }
            // kern_node,math_node: x:=x+width(p);
            else if type_p == kern_node || type_p == math_node {
                todo!("kern && math");
            }
            // ligature_node: @<Make node |p| look like a |char_node|
            //   and |goto reswitch|@>;
            else if type_p == ligature_node {
                todo!("make, reswitch");
            }
            // othercases do_nothing
            else {
                do_nothing!();
            }
            // endcases;@/
            // p:=link(p);
            $p = link!($globals, $p);
            // end;
        }
        // end
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0140::ins_node;
        use crate::section_0141::mark_node;
        use crate::section_0142::adjust_node;
        use crate::section_0143::ligature_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0147::math_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0159::unset_node;
    }}
}

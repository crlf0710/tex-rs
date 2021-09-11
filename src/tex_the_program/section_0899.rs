//! ` `
// @<Check that the nodes following |hb| permit hyphenation...@>=
pub(crate) macro Check_that_the_nodes_following_hb_permit_hyphenation_and_that_at_least_l_hyf_plus_r_hyf_letters_have_been_found__otherwise_goto_done1 {
    ($globals:expr, $s:expr, $lbl_done1:lifetime) => {{
        crate::region_forward_label!(
        |'done4|
        {
        // if hn<l_hyf+r_hyf then goto done1; {|l_hyf| and |r_hyf| are |>=1|}
        if ($globals.hn.get() as integer) < $globals.l_hyf + $globals.r_hyf {
            /// `l_hyf` and `r_hyf` are `>=1`
            const _ : () = ();
            crate::goto_forward_label!($lbl_done1);
        }
        // loop@+  begin if not(is_char_node(s)) then
        loop {
            if !is_char_node!($globals, $s) {
                // case type(s) of
                let type_s = r#type!($globals, $s);
                // ligature_node: do_nothing;
                if type_s == ligature_node {
                    do_nothing!();
                }
                // kern_node: if subtype(s)<>normal then goto done4;
                else if type_s == kern_node {
                    if subtype!($globals, $s) as integer != kern_node_subtype::normal as integer {
                        crate::goto_forward_label!('done4);
                    }
                }
                // whatsit_node,glue_node,penalty_node,ins_node,adjust_node,mark_node:
                else if type_s == whatsit_node || type_s == glue_node || type_s == penalty_node ||
                    type_s == ins_node || type_s == adjust_node || type_s == mark_node {
                    // goto done4;
                    crate::goto_forward_label!('done4);
                }
                // othercases goto done1
                else {
                    crate::goto_forward_label!($lbl_done1);
                }
                // endcases;
            }
            // s:=link(s);
            $s = link!($globals, $s);
            // end;
        }
        }
        // done4:
        'done4 <-
        );
        use crate::pascal::integer;
        use crate::section_0016::do_nothing;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0133::subtype;
        use crate::section_0134::is_char_node;
        use crate::section_0140::ins_node;
        use crate::section_0141::*;
        use crate::section_0142::adjust_node;
        use crate::section_0143::ligature_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0155::kern_node_subtype;
        use crate::section_0157::penalty_node;
    }}
}
